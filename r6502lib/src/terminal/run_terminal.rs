use crate::emulator::{
    write_snapshot_with_unique_name, Bus, BusEvent, Channel, Cpu, Image, Monitor, Opcode,
    OutputDevice, PiaChannel, PiaEvent, Snapshot, TracingMonitor, MOS_6502, RESET,
};
use crate::machine_config::MachineInfo;
use crate::run_options::RunOptions;
use anyhow::{anyhow, Result};
use cursive::backends::crossterm::crossterm::event::{poll, read, Event};
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{stdout, Write};
use std::path::Path;
use std::process::exit;
use std::str::from_utf8;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::spawn;
use std::time::Duration;

#[derive(Debug)]
enum TerminalEvent {
    Shutdown,
}

type TerminalChannel = Channel<TerminalEvent>;

struct RawMode;

impl RawMode {
    fn new() -> Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        disable_raw_mode().expect("Must succeed");
    }
}

fn event_loop(terminal_rx: &Receiver<TerminalEvent>, pia_tx: &Sender<PiaEvent>) -> Result<()> {
    fn try_read_event() -> Result<Option<Event>> {
        if poll(Duration::from_millis(100))? {
            Ok(Some(read()?))
        } else {
            Ok(None)
        }
    }

    let raw_mode = RawMode::new()?;

    loop {
        match terminal_rx.try_recv() {
            Ok(TerminalEvent::Shutdown) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => {}
        }

        if let Some(event) = try_read_event()? {
            _ = pia_tx.send(PiaEvent::Input(event));
        }
    }

    drop(raw_mode);

    Ok(())
}

struct TerminalOutput;

impl OutputDevice for TerminalOutput {
    fn write(&self, ch: char) -> Result<()> {
        let mut stdout = stdout();
        if ch == '\n' {
            stdout.write_all(&[13, 10])?;
        } else {
            stdout.write_all(&[ch as u8])?;
        }
        stdout.flush()?;
        Ok(())
    }
}

pub fn run_terminal(opts: &RunOptions) -> Result<()> {
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    let machine_info = match image.machine_tag {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    let terminal_channel = TerminalChannel::new();
    let pia_channel = PiaChannel::new();
    let pia_tx = pia_channel.sender.clone();

    let (bus, bus_rx) = machine_info.create_bus(Box::new(TerminalOutput), pia_channel, &image)?;
    bus.start();

    let start = if opts.reset {
        bus.load_reset_unsafe()
    } else {
        image.start
    };

    if opts.trace {
        show_image_info(opts, &image, start);
    }

    let monitor: Option<Box<dyn Monitor>> = if opts.trace {
        Some(Box::new(TracingMonitor::default()))
    } else {
        None
    };

    let mut cpu = Cpu::new(bus.view(), monitor);
    cpu.reg.pc = start;

    Runner {
        cpu: &mut cpu,
        bus_rx,
        pia_tx,
        terminal_channel,
        stop_after: opts.stop_after,
        machine_info,
        bus: &bus,
        cycles: opts.cycles,
    }
    .run()
}

pub fn run_terminal_from_snapshot(path: &Path) -> Result<()> {
    let snapshot = Snapshot::read(path)?;
    let _machine_info = MachineInfo::find_by_tag(snapshot.machine_tag)?;
    //let (bus, bus_rx) = machine_info.create_bus(Box::new(TerminalOutput), pia_channel, &image)?;
    todo!();
}

// TBD: This is ugly but it'll work for now
struct Runner<'a> {
    cpu: &'a mut Cpu<'a>,
    bus_rx: Receiver<BusEvent>,
    terminal_channel: TerminalChannel,
    pia_tx: Sender<PiaEvent>,
    stop_after: Option<u64>,
    machine_info: MachineInfo,
    bus: &'a Bus,
    cycles: bool,
}

impl<'a> Runner<'a> {
    fn run(self) -> Result<()> {
        let handle = spawn(move || {
            event_loop(&self.terminal_channel.receiver, &self.pia_tx).expect("Must succeed");
        });

        let jmp_ind = MOS_6502
            .get_op_info(&Opcode::JmpInd)
            .ok_or_else(|| anyhow!("JMP_IND must exist"))?
            .clone();

        let rts = MOS_6502
            .get_op_info(&Opcode::Rts)
            .ok_or_else(|| anyhow!("RTS must exist"))?
            .clone();

        let mut stopped_after_requested_cycles = false;
        'outer: loop {
            while self.cpu.step() {
                match self.bus_rx.try_recv() {
                    Ok(BusEvent::UserBreak) => {
                        break 'outer;
                    }
                    Ok(BusEvent::Reset) => {
                        jmp_ind.execute_word(self.cpu, RESET);
                    }
                    Ok(BusEvent::Snapshot) => write_snapshot_with_unique_name(self.cpu)?,
                    Err(TryRecvError::Disconnected | TryRecvError::Empty) => {}
                }

                if let Some(stop_after) = self.stop_after {
                    if self.cpu.total_cycles >= stop_after {
                        stopped_after_requested_cycles = true;
                        break 'outer;
                    }
                }

                if let Some(halt_addr) = self.machine_info.machine.halt_addr {
                    if self.cpu.reg.pc == halt_addr {
                        break 'outer;
                    }
                }

                if let Some(write_char_addr) = self.machine_info.machine.write_char_addr {
                    if self.cpu.reg.pc == write_char_addr {
                        print!("{}", self.cpu.reg.a as char);
                        rts.execute_no_operand(self.cpu);
                    }
                }
            }
        }

        _ = self.terminal_channel.sender.send(TerminalEvent::Shutdown);
        _ = handle.join();

        self.bus.stop();

        // If program hits BRK return contents of A as exit code, otherwise 0.
        let code = if stopped_after_requested_cycles {
            0
        } else {
            self.cpu.reg.a as i32
        };

        if self.cycles {
            if stopped_after_requested_cycles {
                println!(
                    "Stopped after {cycles} cycles with exit code {code}",
                    cycles = self.cpu.total_cycles
                );
            } else {
                println!(
                    "Completed after {cycles} total cycles with exit code {code}",
                    cycles = self.cpu.total_cycles
                );
            }
        }

        exit(code)
    }
}

fn show_image_info(opts: &RunOptions, image: &Image, start: u16) {
    println!("Image: {}", opts.path.display());

    println!(
        "  {label:<25}: {s} (${s:04X}) bytes",
        label = "Image size",
        s = image.bytes.len()
    );

    println!(
        "  {label:<25}: {format:?}",
        label = "Format",
        format = image.format
    );

    match image.machine_tag {
        Some(tag) => {
            println!(
                "  {label:<25}: {tag}",
                label = "Format",
                tag = from_utf8(&tag).expect("Must be valid UTF-8")
            );
        }
        None => {
            println!("  {label:<25}: (unspecified)", label = "Machine tag",);
        }
    }

    println!(
        "  {label:<25}: ${load:04X}",
        label = "Load address",
        load = image.load
    );

    if opts.reset {
        println!(
            "  {label:<25}: ${start:04X} (RESET, overriding ${original_start:04X})",
            label = "Start address",
            start = start,
            original_start = image.start
        );
    } else {
        println!(
            "  {label:<25}: ${start:04X}",
            label = "Start address",
            start = image.start
        );
    }

    println!(
        "  {label:<25}: ${sp:02X}",
        label = "Initial stack pointer",
        sp = image.sp
    );

    if let Some(stop_after) = opts.stop_after {
        println!("  {label:<25}: {stop_after} cycles", label = "Stop after");
    }
}
