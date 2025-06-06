use crate::args::RunOptions;
use crate::run_pia;
use anyhow::{anyhow, Result};
use r6502lib::{
    DummyMonitor, Image, Memory, MemoryView, Monitor, Opcode, Os, Reg, TracingMonitor, Vm, VmState,
    MOS_6502, OSHALT, OSWRCH,
};
use std::process::exit;
use std::thread::scope;

pub(crate) fn run_terminal(opts: &RunOptions) -> Result<()> {
    let image = Image::load(&opts.path, opts.load, opts.start, None)?;

    let memory = Memory::new();
    memory.store_image(&image)?;

    let os = Os::emulate(opts.emulation.into())?;

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

    let mut code = None;
    let p = &mut code;

    scope(|scope| {
        let mut m = memory.view();
        _ = scope.spawn(move || run_pia(&mut m));

        let m = memory.view();
        _ = scope.spawn(move || *p = Some(run_vm(m, &os, start, opts).expect("Must succeed")));
    });

    // If program hit BRK: return contents of A as exit code
    match code {
        Some(code) => exit(code),
        None => exit(0),
    }
}

fn run_vm(memory: MemoryView, os: &Os, start: u16, opts: &RunOptions) -> Result<i32> {
    let monitor: Box<dyn Monitor> = if opts.trace {
        Box::new(TracingMonitor::default())
    } else {
        Box::new(DummyMonitor)
    };

    let mut vm = Vm::new(monitor, VmState::new(Reg::default(), memory));
    vm.s.reg.pc = start;

    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    let mut stopped_after = false;

    'outer: loop {
        while vm.step() {
            if let Some(stop_after) = opts.stop_after {
                if vm.total_cycles >= stop_after {
                    stopped_after = true;
                    break 'outer;
                }
            }
        }

        match os.is_os_vector(&vm) {
            Some(OSHALT) => {
                break;
            }
            Some(OSWRCH) => {
                print!("{}", vm.s.reg.a as char);
                rti.execute_no_operand(&mut vm.s);
            }
            _ => break,
        }
    }

    if opts.cycles {
        if stopped_after {
            println!("Stopped after {} cycles", vm.total_cycles)
        } else {
            println!("Completed after {} total cycles", vm.total_cycles);
        }
    }

    Ok(if stopped_after { 0 } else { vm.s.reg.a as i32 })
}
