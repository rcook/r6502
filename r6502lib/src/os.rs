use crate::{Opcode, Vm, IRQ, OSWRCH};

// Set up OSWRCH as a software interrupt etc.
pub(crate) fn set_up_os(vm: &mut Vm, os: u16) {
    // TBD: Set up other standard vectors such as RESET etc.
    vm.s.memory.store_word(IRQ, os);

    vm.s.memory[OSWRCH] = Opcode::Brk as u8;
    vm.s.memory[OSWRCH + 1] = Opcode::Nop as u8;
    vm.s.memory[OSWRCH + 2] = Opcode::Rts as u8;
}
