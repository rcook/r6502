use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{
    iter_ops, Flag, ImageSource, Instruction, MachineState, Op, OpFunc, RunVMResult, RunVMStatus,
    Status, VMHost, IRQ, IRQ_VALUE, OSHALT, OSWRCH, STACK_BASE,
};
use anyhow::{bail, Result};

pub(crate) fn run_vm<H: VMHost>(
    host: &H,
    image_source: Option<ImageSource>,
    mut free_running: bool,
) -> Result<RunVMResult> {
    let mut m = MachineState::new();

    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in iter_ops() {
            ops[op.opcode as usize] = Some(*op)
        }
        ops
    };

    if let Some(ref image_info) = image_source {
        let start_info = image_info.load_into_memory(&mut m.memory)?;
        m.reg.pc = start_info.start;
    }

    // Set up interrupt vectors
    m.store_word(IRQ, IRQ_VALUE);

    // Set up operating system handlers
    set_brk(&mut m, OSWRCH);
    set_brk(&mut m, OSHALT);

    // Initialize the state
    m.push_word(OSHALT - 1);

    let mut cycles = 0;

    loop {
        while !m.get_flag(Flag::B) {
            // Fetch next instruction without incrementing PC
            let (instruction, pc) = peek(&mut m, &ops)?;
            host.report_before_execute(&m.reg, cycles, &instruction, pc);

            let result = host.poll(free_running);
            free_running = result.free_running;
            if !result.is_active {
                // Handle disconnection
                return Ok(RunVMResult::new(RunVMStatus::Disconnected, m, cycles));
            }

            // Increment PC and execute the instruction
            cycles += match instruction {
                Instruction::NoOperand(_, f) => {
                    m.reg.pc += 1;
                    f(&mut m)
                }
                Instruction::Byte(_, f, operand) => {
                    m.reg.pc += 2;
                    f(&mut m, operand)
                }
                Instruction::Word(_, f, operand) => {
                    m.reg.pc += 3;
                    f(&mut m, operand)
                }
            };

            host.report_after_execute(&m.reg, cycles, &instruction);
        }

        // Check for expected interrupt request value
        if m.reg.pc != IRQ_VALUE {
            bail!("Unexpected IRQ value {:04X}", m.reg.pc);
        }

        // Address of operating system routine being invoked
        let addr = m.fetch_word(STACK_BASE + (m.reg.s + 2) as u16) - 1;

        match addr {
            OSWRCH => {
                let c = m.reg.a as char;
                host.write_stdout(c);
            }
            OSHALT => {
                host.report_status(Status::Halted);
                return Ok(RunVMResult::new(RunVMStatus::Halted, m, cycles));
            }
            _ => panic!("Break at unimplemented subroutine {:04X}", addr),
        }

        cycles += match RTI.func {
            OpFunc::NoOperand(f) => f(&mut m),
            _ => unreachable!(),
        };
    }

    fn peek(m: &mut MachineState, ops: &[Option<Op>]) -> Result<(Instruction, u16)> {
        let opcode = m.fetch(m.reg.pc);
        match ops[opcode as usize] {
            Some(op) => match op.func {
                OpFunc::NoOperand(f) => Ok((Instruction::NoOperand(op, f), m.reg.pc + 1)),
                OpFunc::Byte(f) => Ok((
                    Instruction::Byte(op, f, m.fetch(m.reg.pc + 1)),
                    m.reg.pc + 2,
                )),
                OpFunc::Word(f) => Ok((
                    Instruction::Word(op, f, m.fetch_word(m.reg.pc + 1)),
                    m.reg.pc + 3,
                )),
            },
            None => bail!("Unsupported opcode {opcode:02X}"),
        }
    }
}

// Set up operating system routine
fn set_brk(m: &mut MachineState, addr: u16) {
    m.store(addr, BRK.opcode); // Software interrupt
    m.store(addr + 1, NOP.opcode); // Padding
    m.store(addr + 2, RTS.opcode); // Return
}
