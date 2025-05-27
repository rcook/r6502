use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{
    split_word, ByteFn, Cpu, Flag, NoOperandFn, Op, OpFunc, ProgramInfo, WordFn, IRQ, OPS, OSHALT,
    OSWRCH, STACK_BASE,
};
use anyhow::{bail, Result};

pub(crate) fn run_vm(cpu: &mut Cpu, program_info: Option<ProgramInfo>) -> Result<()> {
    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    if let Some(ref program_info) = program_info {
        program_info.load(&mut cpu.memory)?;
        cpu.pc = program_info.start();
    }

    // Set up interrupt vectors
    cpu.store_word(IRQ, IRQ_VALUE);

    // Set up operating system handlers
    set_brk(cpu, OSWRCH);
    set_brk(cpu, OSHALT);

    // Initialize the state
    cpu.push_word(OSHALT - 1);

    enum Instruction {
        NoOperand(Op, NoOperandFn),
        Byte(Op, ByteFn, u8),
        Word(Op, WordFn, u16),
    }

    loop {
        while !cpu.get_flag(Flag::B) {
            cpu.registers();
            let opcode = cpu.next();

            let instruction = match ops[opcode as usize] {
                Some(op) => match op.func {
                    OpFunc::NoOperand(f) => Instruction::NoOperand(op, f),
                    OpFunc::Byte(f) => Instruction::Byte(op, f, cpu.next()),
                    OpFunc::Word(f) => Instruction::Word(op, f, cpu.next_word()),
                },
                None => bail!("Unsupported opcode {opcode:02X}"),
            };

            match instruction {
                Instruction::NoOperand(op, _) => cpu.current(&format!(
                    "{:02X}       {} {:?}",
                    op.opcode, op.mnemonic, op.addressing_mode
                )),
                Instruction::Byte(op, _, operand) => cpu.current(&format!(
                    "{:02X} {:02X}    {} {:?}",
                    op.opcode, operand, op.mnemonic, op.addressing_mode
                )),
                Instruction::Word(op, _, operand) => {
                    let (hi, lo) = split_word(operand);
                    cpu.current(&format!(
                        "{:02X} {:02X} {:02X} {} {:?}",
                        op.opcode, lo, hi, op.mnemonic, op.addressing_mode
                    ))
                }
            }

            if !cpu.poll() {
                // Handle disconnection
                return Ok(());
            }

            match instruction {
                Instruction::NoOperand(op, f) => {
                    cpu.history(&format!(
                        "{:02X}       {} {:?}",
                        op.opcode, op.mnemonic, op.addressing_mode
                    ));
                    f(cpu)
                }
                Instruction::Byte(op, f, operand) => {
                    cpu.history(&format!(
                        "{:02X} {:02X}    {} {:?}",
                        op.opcode, operand, op.mnemonic, op.addressing_mode
                    ));
                    f(cpu, operand)
                }
                Instruction::Word(op, f, operand) => {
                    let (hi, lo) = split_word(operand);
                    cpu.history(&format!(
                        "{:02X} {:02X} {:02X} {} {:?}",
                        op.opcode, lo, hi, op.mnemonic, op.addressing_mode
                    ));
                    f(cpu, operand)
                }
            }
        }

        // Check for expected interrupt request value
        if cpu.pc != IRQ_VALUE {
            bail!("Unexpected IRQ value {:04X}", cpu.pc);
        }

        // Address of operating system routine being invoked
        let addr = cpu.fetch_word(STACK_BASE + (cpu.s + 2) as u16) - 1;

        match addr {
            OSWRCH => {
                let c = cpu.a as char;
                cpu.write_stdout(c);
            }
            OSHALT => {
                cpu.history("Halted");
                if let Some(ref program_info) = program_info {
                    program_info.save_dump(&cpu.memory)?;
                }
                cpu.on_halted();
                return Ok(());
            }
            _ => panic!("Break at unimplemented subroutine {:04X}", addr),
        }

        match RTI.func {
            OpFunc::NoOperand(f) => f(cpu),
            _ => unreachable!(),
        }
    }
}

// Set up operating system routine
fn set_brk(cpu: &mut Cpu, addr: u16) {
    cpu.store(addr, BRK.opcode); // Software interrupt
    cpu.store(addr + 1, NOP.opcode); // Padding
    cpu.store(addr + 2, RTS.opcode); // Return
}
