use crate::args::RunOptions;
use anyhow::{anyhow, Result};
use log::LevelFilter;
use r6502lib::{
    Cpu, CpuState, DummyMonitor, Image, Memory, Monitor, Opcode, Os, Reg, TracingMonitor, MOS_6502,
    OSHALT, OSWRCH,
};
use simple_logging::log_to_file;
use std::process::exit;

pub(crate) fn run_terminal(opts: &RunOptions) -> Result<()> {
    log_to_file("r6502.log", LevelFilter::Info)?;

    let memory = Memory::emulate(opts.emulation.into());
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;
    memory.store_image(&image)?;
    memory.start();

    let start = if opts.reset {
        memory.load_reset_unsafe()
    } else {
        image.start
    };

    if opts.trace {
        println!("Image: {}", opts.path.display());

        println!(
            "  {label:<25}: {s} (${s:04X}) bytes",
            label = "Image size",
            s = image.values.len()
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
            println!("  {label:<25}: {stop_after} cycles", label = "Stop after")
        }
    }

    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    let os = Os::emulate(opts.emulation.into())?;

    let monitor: Box<dyn Monitor> = if opts.trace {
        Box::new(TracingMonitor::default())
    } else {
        Box::new(DummyMonitor)
    };

    let mut cpu = Cpu::new(monitor, CpuState::new(Reg::default(), memory.view()));
    cpu.s.reg.pc = start;

    let mut stopped_after_requested_cycles = false;
    'outer: loop {
        while cpu.step() {
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
                print!("{}", cpu.s.reg.a as char);
                rti.execute_no_operand(&mut cpu.s);
            }
            _ => break,
        }
    }

    if opts.cycles {
        if stopped_after_requested_cycles {
            println!("Stopped after {} cycles", cpu.total_cycles)
        } else {
            println!("Completed after {} total cycles", cpu.total_cycles);
        }
    }

    // If program hits BRK return contents of A as exit code, otherwise 0.
    exit(if stopped_after_requested_cycles {
        0
    } else {
        cpu.s.reg.a as i32
    })
}
