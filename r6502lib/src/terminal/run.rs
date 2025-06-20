use crate::emulator::{Cpu, Image, Monitor, PiaChannel, TracingMonitor};
use crate::machine_config::MachineInfo;
use crate::run_options::RunOptions;
use crate::terminal::{show_image_info, Runner, StopReason, TerminalChannel, TerminalOutput};
use anyhow::Result;
use cursive::backends::crossterm::crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use log::info;
use std::process::exit;

pub fn run(opts: &RunOptions) -> Result<()> {
    #[allow(unused)]
    struct RawMode(i32);

    impl RawMode {
        fn new() -> Result<Self> {
            enable_raw_mode()?;
            Ok(Self(0))
        }
    }

    impl Drop for RawMode {
        fn drop(&mut self) {
            _ = disable_raw_mode();
        }
    }

    fn run_inner(opts: &RunOptions) -> Result<i32> {
        let image = Image::from_file(&opts.path)?;
        let machine_info = match image.machine_tag() {
            Some(tag) => MachineInfo::find_by_tag(tag)?,
            None => MachineInfo::find_by_name(&opts.machine)?,
        };

        let terminal_channel = TerminalChannel::new();
        let pia_channel = PiaChannel::new();
        let pia_tx = pia_channel.tx.clone();

        let (bus, bus_rx) =
            machine_info.create_bus(Box::new(TerminalOutput), pia_channel, &image)?;
        bus.start();

        let start = if opts.reset {
            bus.load_reset_unsafe()
        } else {
            image.start().or(opts.start).unwrap_or_default()
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
        image.set_initial_cpu_state(&mut cpu);

        let stop_reason = Runner {
            cpu: &mut cpu,
            bus_rx,
            pia_tx,
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
                (total_cycles, a as i32)
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

    let raw_mode = RawMode::new()?;
    let code = run_inner(opts)?;
    drop(raw_mode);

    exit(code);
}
