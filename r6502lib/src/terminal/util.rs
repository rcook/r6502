use crate::emulator::Image;
use crate::run_options::RunOptions;
use log::info;
use std::fmt::Display;
use std::str::from_utf8;

pub fn show_image_info(opts: &RunOptions, image: &Image, start: u16) {
    fn log_property<D: Display>(label: &str, value: D) {
        info!("{label}: {value}");
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

    if opts.reset {
        log_property(
            "Start address",
            format!(
                "${start:04X} (RESET, overriding ${original_start:04X})",
                start = start,
                original_start = image.start().or(opts.start).unwrap_or_default()
            ),
        );
    } else {
        log_property(
            "Start address",
            format!(
                "${start:04X}",
                start = image.start().or(opts.start).unwrap_or_default()
            ),
        );
    }

    match image.sp() {
        Some(sp) => log_property("Initial stack pointer", format!("${sp:02X}")),
        None => log_property("Initial stack pointer", "(unspecified)"),
    }

    if let Some(stop_after) = opts.stop_after {
        log_property("Stop after cycles", stop_after);
    }
}
