use crate::emulator::Image;
use crate::run_options::RunOptions;
use std::str::from_utf8;

pub fn show_image_info(opts: &RunOptions, image: &Image, start: u16) {
    println!("Image: {}", opts.path.display());

    println!(
        "  {label:<25}: {s} (${s:04X}) bytes",
        label = "Image size",
        s = image.bytes.len()
    );

    match image.machine_tag() {
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
        load = image.load().or(opts.load).unwrap_or_default()
    );

    if opts.reset {
        println!(
            "  {label:<25}: ${start:04X} (RESET, overriding ${original_start:04X})",
            label = "Start address",
            start = start,
            original_start = image.start().or(opts.start).unwrap_or_default()
        );
    } else {
        println!(
            "  {label:<25}: ${start:04X}",
            label = "Start address",
            start = image.start().or(opts.start).unwrap_or_default()
        );
    }

    match image.sp() {
        Some(sp) => {
            println!(
                "  {label:<25}: ${sp:02X}",
                label = "Initial stack pointer",
                sp = sp
            );
        }
        None => {
            println!(
                "  {label:<25}: (unspecified)",
                label = "Initial stack pointer",
            );
        }
    }

    if let Some(stop_after) = opts.stop_after {
        println!("  {label:<25}: {stop_after} cycles", label = "Stop after");
    }
}
