use crate::terminal_ui::RunOptions;
use log::info;
use r6502lib::emulator::{CpuState, MemoryImage};
use std::fmt::Display;
use std::str::from_utf8;

pub struct Vectors {
    pub nmi: u16,
    pub reset: u16,
    pub irq: u16,
}

pub fn show_run_info(
    opts: &RunOptions,
    image: &MemoryImage,
    initial_cpu_state: &CpuState,
    vectors: &Vectors,
) {
    fn log_property<D: Display>(label: &str, value: D) {
        info!("{label:<12}: {value}");
    }

    log_property("Image", opts.path.display());
    log_property(
        "Image size",
        format!("{size} (${size:04X}) bytes", size = image.bytes().len()),
    );

    match image.machine_tag() {
        Some(tag) => log_property("Machine tag", from_utf8(&tag).unwrap()),
        None => log_property("Machine tag", "(unspecified)"),
    }

    log_property(
        "Load address",
        format!(
            "${load:04X}",
            load = image.load().or(opts.load).unwrap_or_default()
        ),
    );

    log_property(
        "Initial PC",
        format!("${pc:04X}", pc = initial_cpu_state.pc),
    );
    log_property("Initial A", format!("${a:02X}", a = initial_cpu_state.a));
    log_property("Initial X", format!("${x:02X}", x = initial_cpu_state.x));
    log_property("Initial Y", format!("${y:02X}", y = initial_cpu_state.y));
    log_property(
        "Initial SP",
        format!("${sp:02X}", sp = initial_cpu_state.sp),
    );
    log_property("Initial P", format!("${p:02X}", p = initial_cpu_state.p));
    log_property("Total cycles", initial_cpu_state.total_cycles);

    if let Some(stop_after) = opts.stop_after {
        log_property("Stop after cycles", stop_after);
    }

    log_property("NMI", format!("${:04X}", vectors.nmi));
    log_property("RESET", format!("${:04X}", vectors.reset));
    log_property("IRQ", format!("${:04X}", vectors.irq));
}
