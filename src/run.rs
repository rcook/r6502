use crate::{run_vm, Args, ImageInfo, UI};
use anyhow::Result;
use clap::Parser;
use std::sync::mpsc::channel;
use std::thread::spawn;

pub(crate) fn run() -> Result<()> {
    let args = Args::parse();
    let image_info = Some(ImageInfo::new(&args.path, args.origin, args.start));
    if args.debug {
        let debug_channel = channel();
        let status_channel = channel();
        let mut ui = UI::new(status_channel.1, debug_channel.0)?;
        spawn(move || {
            run_vm(
                Some(debug_channel.1),
                Some(status_channel.0),
                image_info,
                !args.debug,
            )
            .expect("Must succeed");
        });
        ui.run();
    } else {
        run_vm(None, None, image_info, !args.debug)?;
    }
    Ok(())
}
