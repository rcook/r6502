use crate::{run, Controller, Memory, State};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{ErrorKind, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread::spawn;

fn load(memory: &mut Memory, path: &Path, addr: u16) -> Result<()> {
    let len = memory.len();
    let buffer = &mut memory[addr as usize..len];
    let mut file = File::open(path)?;
    match file.read_exact(buffer) {
        Ok(()) => {}
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => {}
        Err(e) => bail!(e),
    }
    Ok(())
}

pub(crate) fn demo() -> Result<()> {
    let (vm_tx, vm_rx) = channel();
    let mut controller = Controller::new(vm_tx)?;
    let thunk = controller.thunk();
    let mut state = State::new(thunk, vm_rx);
    spawn(move || {
        load(&mut state.memory, Path::new("examples\\Main.bin"), 0x2000).unwrap();
        state.pc = 0x2000u16;
        run(&mut state).unwrap();
    });
    controller.run();
    Ok(())
}
