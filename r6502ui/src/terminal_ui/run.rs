use crate::terminal_ui::raw_mode::RawMode;
use crate::terminal_ui::{RunOptions, output_device_type_util};
use crate::terminal_ui::{Runner, StopReason, TerminalChannel, Vectors, show_run_info};
use anyhow::Result;
use log::info;
use r6502cpu::InterruptChannel;
use r6502lib::emulator::{Cpu, IoChannel, MachineInfo, MemoryImage, Monitor, TracingMonitor};
use std::process::exit;

pub fn run(opts: &RunOptions) -> Result<()> {
    fn run_inner(opts: &RunOptions) -> Result<i32> {
        let image = MemoryImage::from_file(&opts.path)?;
        let machine_info = match image.machine_tag() {
            Some(tag) => MachineInfo::find_by_tag(tag)?,
            None => MachineInfo::find_by_name(&opts.machine)?,
        };

        let terminal_channel = TerminalChannel::new();
        let io_channel = IoChannel::new();
        let io_tx = io_channel.tx.clone();
        let interrupt_channel = InterruptChannel::new();

        let output =
            output_device_type_util::create_output_device(&machine_info.machine.output_device_type);
        let (bus, bus_rx) =
            machine_info.create_bus(output, io_channel, interrupt_channel.tx, &image)?;
        bus.start();

        let nmi = bus.load_nmi_unsafe();
        let reset = bus.load_reset_unsafe();
        let irq = bus.load_irq_unsafe();
        let vectors = Vectors { nmi, reset, irq };

        let monitor: Option<Box<dyn Monitor>> = if opts.trace {
            Some(Box::new(TracingMonitor::default()))
        } else {
            None
        };

        let mut cpu = Cpu::new(bus.view(), monitor, interrupt_channel.rx);
        let cpu_state = image.get_initial_cpu_state(&cpu);
        show_run_info(opts, &image, &cpu_state, &vectors);
        cpu_state.apply_to(&mut cpu);

        let stop_reason = Runner {
            cpu: &mut cpu,
            bus_rx,
            io_tx,
            terminal_channel,
            stop_after: opts.stop_after,
            machine_info,
            bus: &bus,
        }
        .run()?;

        let (total_cycles, code) = match stop_reason {
            StopReason::UnexpectedInterrupt { total_cycles } => {
                info!("Program stopped due to unexpected interrupt (BRK)");
                (total_cycles, 2)
            }
            StopReason::UserBreak { total_cycles } => {
                info!("Program stopped due to user break (Ctrl+C)");
                (total_cycles, 1)
            }
            StopReason::RequestedCyclesExecuted { total_cycles } => {
                info!("Program stopped after requested cycle count");
                (total_cycles, 0)
            }
            StopReason::Halt { total_cycles, a } => {
                info!("Program stopped by call to EXIT");
                (total_cycles, i32::from(a))
            }
        };

        if opts.cycles {
            if matches!(stop_reason, StopReason::RequestedCyclesExecuted { .. }) {
                info!("Stopped after {total_cycles} cycles with exit code {code}");
            } else {
                info!("Completed after {total_cycles} total cycles with exit code {code}");
            }
        }

        Ok(code)
    }

    let raw_mode = RawMode::enable()?;
    let code = run_inner(opts)?;
    drop(raw_mode);

    exit(code);
}
