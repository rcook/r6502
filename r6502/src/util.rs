use anyhow::{anyhow, Result};
use r6502lib::{Image, OpInfo, Opcode, Os, OsBuilder, Vm, MOS_6502, OSHALT};

pub(crate) fn initialize_vm(vm: &mut Vm, image: &Image) -> Result<(Os, OpInfo)> {
    let os = OsBuilder::default().build()?;

    let rti = MOS_6502
        .get_op_info(&Opcode::Rti)
        .ok_or_else(|| anyhow!("RTI must exist"))?
        .clone();

    os.initialize(&mut vm.s.memory);
    vm.s.memory.load(image);
    vm.s.push_word(OSHALT - 1);
    vm.s.reg.pc = image.start;

    Ok((os, rti))
}
