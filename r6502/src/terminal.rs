use crate::args::RunOptions;
use anyhow::{anyhow, Result};
use chrono::Utc;
use log::LevelFilter;
use r6502lib::{
    Bus, BusEvent, Cpu, Image, Monitor, Opcode, Os, TracingMonitor, MOS_6502, OSHALT, OSWRCH,
};
use simple_logging::log_to_file;
use std::env::current_dir;
use std::process::exit;
use std::sync::mpsc::{channel, TryRecvError};

pub(crate) fn run_terminal(opts: &RunOptions) -> Result<()> {
    log_to_file("r6502.log", LevelFilter::Info)?;

    let (bus_tx, bus_rx) = channel();
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    let bus = Bus::configure_for(opts.emulation.into(), &bus_tx, Some(&image));
    bus.start();

    let start = if opts.reset {
        bus.load_reset_unsafe()
    } else {
        image.start
    };

    if opts.trace {
        show_image_info(opts, &image, start);
    }

    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    let os = Os::new(opts.emulation.into());

    let monitor: Option<Box<dyn Monitor>> = if opts.trace {
        Some(Box::new(TracingMonitor::default()))
    } else {
        None
    };

    let mut cpu = Cpu::new(bus.view(), monitor);
    cpu.reg.pc = start;

    let mut stopped_after_requested_cycles = false;
    'outer: loop {
        while cpu.step() {
            match bus_rx.try_recv() {
                Ok(BusEvent::HardwareBreak) => {
                    println!("Ctrl+C");
                    break 'outer;
                }
                Ok(BusEvent::Snapshot) => {
                    write_snapshot(&cpu)?;
                }
                Err(TryRecvError::Disconnected | TryRecvError::Empty) => {}
            }

            if let Some(stop_after) = opts.stop_after {
                if cpu.total_cycles >= stop_after {
                    stopped_after_requested_cycles = true;
                    break 'outer;
                }
            }
        }

        match os.is_os_vector(&cpu) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", cpu.reg.a as char);
                rti.execute_no_operand(&mut cpu);
            }
            _ => break,
        }
    }

    bus.join();

    if opts.cycles {
        if stopped_after_requested_cycles {
            println!("Stopped after {} cycles", cpu.total_cycles);
        } else {
            println!("Completed after {} total cycles", cpu.total_cycles);
        }
    }

    // If program hits BRK return contents of A as exit code, otherwise 0.
    exit(if stopped_after_requested_cycles {
        0
    } else {
        cpu.reg.a as i32
    })
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

fn write_snapshot(cpu: &Cpu) -> Result<()> {
    let now = Utc::now();
    let file_name = format!(
        "r6502-snapshot-{timestamp}.bin",
        timestamp = now.format("%Y%m%d%H%M%S")
    );

    let path = current_dir()?.join(file_name);
    cpu.write_snapshot(&path)?;
    Ok(())
}
