use crate::emulator::run_emulator;
use crate::font::Font;
use crate::gui::GraphicsTerminal;
use crate::terminal::Terminal;
use crate::terminal_event::TerminalEvent;
use crate::tui::TextTerminal;
use crate::util::set_main_thread;
use anyhow::{Result, anyhow};
use log::{Log, info};
use path_absolutize::Absolutize;
use r6502config::HostHookType;
use r6502core::emulator::{Bus, Cpu, Monitor, TracingMonitor};
use r6502core::symbols::MapFile;
use r6502core::{BusDevice, DeviceMapping, InterruptEvent};
use r6502hw::MachineInfo;
use r6502lib::ascii::{CR, DEL, ESC};
use r6502lib::keyboard::{KeyCode, KeyEvent, KeyModifiers};
use r6502lib::util::make_word;
use r6502lib::{AddressRange, Channel, RESET};
use r6502snapshot::MemoryImage;
use sdl3::libc::MAP_FAILED;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

const PA: u16 = 0xfc00;
const PA_CR: u16 = 0xfc01;
const PB: u16 = 0xfc02;
const PB_CR: u16 = 0xfc03;

enum MemoryEvent {
    Write(char),
}

enum StopReason {
    Halt,
    Disconnected,
    Closed,
}

struct FlatMemoryDevice {
    tx: Sender<MemoryEvent>,
    bytes: RefCell<Vec<u8>>,
}

impl FlatMemoryDevice {
    fn new(tx: Sender<MemoryEvent>) -> Self {
        Self {
            tx,
            bytes: RefCell::new(vec![0x00; 0x10000]),
        }
    }

    fn load_image(&self, image: &MemoryImage) {
        let load = image.load().unwrap_or_default() as usize;
        let image_bytes = image.bytes();
        let mut bytes = self.bytes.borrow_mut();
        bytes.deref_mut()[load..load + image_bytes.len()].copy_from_slice(image_bytes);
    }
}

impl BusDevice for FlatMemoryDevice {
    fn load(&self, addr: u16) -> u8 {
        match addr {
            PA_CR => todo!("PA_CR"),
            PB => return 0x00,
            PB_CR => todo!("PB_CR"),
            _ => {
                let bytes = self.bytes.borrow();
                bytes[addr as usize]
            }
        }
    }

    fn store(&self, addr: u16, value: u8) {
        match addr {
            PA_CR => todo!("PA_CR"),
            PB => _ = self.tx.send(MemoryEvent::Write(value as char)),
            PB_CR => todo!("PB_CR"),
            _ => {
                let mut bytes = self.bytes.borrow_mut();
                bytes[addr as usize] = value;
            }
        }
    }
}

pub fn run_gui(font: &Font) -> Result<()> {
    info!("main thread started");
    set_main_thread();

    let reason = GraphicsTerminal::with(font, |terminal, rx| -> Result<StopReason> {
        let image = MemoryImage::from_file(Path::new("examples/bbc-basic/bbc-basic.r6502"))?;
        let machine_tag = image
            .machine_tag()
            .ok_or_else(|| anyhow!("machine tag not defined"))?;
        let machine_info = MachineInfo::find_by_tag(machine_tag)?;

        let memory_channel = Channel::new();

        let device = FlatMemoryDevice::new(memory_channel.tx);

        if let Some(base_image_path) = machine_info.machine.base_image_path.as_ref() {
            device.load_image(&MemoryImage::from_file(
                &base_image_path.absolutize_from(&machine_info.config_dir)?,
            )?);
        }

        device.load_image(&image);

        let bus = Bus::new(
            machine_tag,
            vec![DeviceMapping {
                address_range: AddressRange::new(0x0000, 0xffff)?,
                device: Box::new(device),
                offset: 0x0000,
            }],
        );
        bus.start();

        let interrupt_channel = Channel::new();

        let map_file = MapFile {
            modules: Vec::new(),
            segments: Vec::new(),
            exports: Vec::new(),
        };
        let monitor = TracingMonitor::new(map_file);

        //let mut cpu = Cpu::new(bus.view(), Some(Box::new(monitor)), interrupt_channel.rx);
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        let reset_addr_lo = cpu.bus.load(RESET);
        let reset_addr_hi = cpu.bus.load(RESET.wrapping_add(1));
        let reset_addr = make_word(reset_addr_hi, reset_addr_lo);
        let cpu_state = image.get_initial_cpu_state(reset_addr);
        cpu.set_initial_state(&cpu_state);

        info!("running");
        run_gui_inner(
            cpu,
            &machine_info,
            terminal,
            rx,
            &memory_channel.rx,
            &interrupt_channel.tx,
            &bus,
        )
    })?;

    match reason {
        StopReason::Halt => info!("Halted"),
        StopReason::Disconnected => info!("Disconnected"),
        StopReason::Closed => info!("Closed"),
    }

    Ok(())
}

fn run_gui_inner(
    mut cpu: Cpu,
    machine_info: &MachineInfo,
    terminal: &GraphicsTerminal,
    terminal_rx: &Receiver<TerminalEvent>,
    memory_rx: &Receiver<MemoryEvent>,
    interrupt_tx: &Sender<InterruptEvent>,
    bus: &Bus,
) -> Result<StopReason> {
    loop {
        // TBD: Move this off this thread
        match memory_rx.try_recv() {
            Ok(MemoryEvent::Write(c)) => terminal.write_decode(c),
            Err(TryRecvError::Disconnected) => return Ok(StopReason::Disconnected),
            Err(TryRecvError::Empty) => {}
        }

        // TBD: Move this off this thread
        match terminal_rx.try_recv() {
            Ok(TerminalEvent::Closed) => return Ok(StopReason::Closed),
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
            })) => {
                bus.store(PA, DEL);
                _ = interrupt_tx.send(InterruptEvent::Irq)
            }
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Enter,
                modifiers,
            })) if modifiers == KeyModifiers::NONE || modifiers == KeyModifiers::SHIFT => {
                bus.store(PA, CR);
                _ = interrupt_tx.send(InterruptEvent::Irq)
            }
            Ok(TerminalEvent::Key(KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
            })) => {
                bus.store(PA, ESC);
                _ = interrupt_tx.send(InterruptEvent::Irq)
            }
            Ok(TerminalEvent::Key(key_event)) => info!("key_event: {key_event:?}"),
            Ok(TerminalEvent::TextInput(s)) => {
                for c in s.chars() {
                    bus.store(PA, c as u8);
                    _ = interrupt_tx.send(InterruptEvent::Irq)
                }
            }
            Ok(event) => info!("event: {event:?}"),
            Err(TryRecvError::Disconnected) => return Ok(StopReason::Disconnected),
            Err(TryRecvError::Empty) => {}
        }

        cpu.step_with_monitor_callbacks();

        if let Some(halt_addr) = machine_info.machine.halt_addr {
            if cpu.reg.pc == halt_addr {
                return Ok(StopReason::Halt);
            }
        }

        if let Some(host_hook) = &machine_info.machine.host_hook {
            if cpu.reg.pc == host_hook.addr {
                match host_hook.r#type {
                    HostHookType::Acorn => {
                        todo!()
                    }
                }
            }
        }
    }
    unreachable!()
}
