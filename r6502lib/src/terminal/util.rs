use crate::emulator::Image;
use crate::run_options::RunOptions;
use log::info;
use std::str::from_utf8;

pub fn show_image_info(opts: &RunOptions, image: &Image, start: u16) {
    info!("Image: {}", opts.path.display());

    info!(
        "  {label:<25}: {s} (${s:04X}) bytes",
        label = "Image size",
        s = image.bytes().len()
    );

    match image.machine_tag() {
        Some(tag) => {
            info!(
                "  {label:<25}: {tag}",
                label = "Format",
                tag = from_utf8(&tag).expect("Must be valid UTF-8")
            );
        }
        None => {
            info!("  {label:<25}: (unspecified)", label = "Machine tag",);
        }
    }

    info!(
        "  {label:<25}: ${load:04X}",
        label = "Load address",
        load = image.load().or(opts.load).unwrap_or_default()
    );

    if opts.reset {
        info!(
            "  {label:<25}: ${start:04X} (RESET, overriding ${original_start:04X})",
            label = "Start address",
            start = start,
            original_start = image.start().or(opts.start).unwrap_or_default()
        );
    } else {
        info!(
            "  {label:<25}: ${start:04X}",
            label = "Start address",
            start = image.start().or(opts.start).unwrap_or_default()
        );
    }

    match image.sp() {
        Some(sp) => {
            info!(
                "  {label:<25}: ${sp:02X}",
                label = "Initial stack pointer",
                sp = sp
            );
        }
        None => {
            info!(
                "  {label:<25}: (unspecified)",
                label = "Initial stack pointer",
            );
        }
    }

    if let Some(stop_after) = opts.stop_after {
        info!("  {label:<25}: {stop_after} cycles", label = "Stop after");
    }
}
