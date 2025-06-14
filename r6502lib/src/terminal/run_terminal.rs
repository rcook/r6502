use crate::emulator::{BusEvent, Cpu, Image, Monitor, Opcode, TracingMonitor, MOS_6502};
use crate::machine_config::MachineInfo;
use crate::run_options::RunOptions;
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::env::current_dir;
use std::process::exit;
use std::str::from_utf8;
use std::sync::mpsc::TryRecvError;

pub fn run_terminal(opts: &RunOptions) -> Result<()> {
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    let machine_info = match image.tag {
        Some(tag) => MachineInfo::find_by_tag(tag)?,
        None => MachineInfo::find_by_name(&opts.machine)?,
    };

    let (bus, bus_rx) = machine_info.create_bus(&image)?;
    bus.start();

    let start = if opts.reset {
        bus.load_reset_unsafe()
    } else {
        image.start
    };

    if opts.trace {
        show_image_info(opts, &image, start);
    }

    let rts = MOS_6502
        .get_op_info(&Opcode::Rts)
        .ok_or_else(|| anyhow!("RTS must exist"))?
        .clone();

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
                Ok(BusEvent::UserBreak) => {
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

            if let Some(halt_addr) = machine_info.machine.halt_addr {
                if cpu.reg.pc == halt_addr {
                    break 'outer;
                }
            }

            if let Some(write_char_addr) = machine_info.machine.write_char_addr {
                if cpu.reg.pc == write_char_addr {
                    print!("{}", cpu.reg.a as char);
                    rts.execute_no_operand(&mut cpu);
                }
            }
        }
    }

    bus.stop();

    // If program hits BRK return contents of A as exit code, otherwise 0.
    let code = if stopped_after_requested_cycles {
        0
    } else {
        cpu.reg.a as i32
    };

    if opts.cycles {
        if stopped_after_requested_cycles {
            println!(
                "Stopped after {cycles} cycles with exit code {code}",
                cycles = cpu.total_cycles
            );
        } else {
            println!(
                "Completed after {cycles} total cycles with exit code {code}",
                cycles = cpu.total_cycles
            );
        }
    }

    exit(code)
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

    match image.tag {
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
