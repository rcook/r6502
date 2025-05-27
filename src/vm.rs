use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{
    Cpu, Flag, Instruction, Op, OpFunc, ProgramInfo, IRQ, OPS, OSHALT, OSWRCH, STACK_BASE,
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

    let mut cycles = 0;

    loop {
        while !cpu.get_flag(Flag::B) {
            cpu.registers();
            cpu.cycles(cycles);
            let opcode = cpu.next();

            let instruction = match ops[opcode as usize] {
                Some(op) => match op.func {
                    OpFunc::NoOperand(f) => Instruction::NoOperand(op, f),
                    OpFunc::Byte(f) => Instruction::Byte(op, f, cpu.next()),
                    OpFunc::Word(f) => Instruction::Word(op, f, cpu.next_word()),
                },
                None => bail!("Unsupported opcode {opcode:02X}"),
            };

            cpu.current(&instruction);

            if !cpu.poll() {
                // Handle disconnection
                return Ok(());
            }

            cycles += match instruction {
                Instruction::NoOperand(_, f) => {
                    cpu.disassembly(&instruction);
                    f(cpu)
                }
                Instruction::Byte(_, f, operand) => {
                    cpu.disassembly(&instruction);
                    f(cpu, operand)
                }
                Instruction::Word(_, f, operand) => {
                    cpu.disassembly(&instruction);
                    f(cpu, operand)
                }
            };
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
                cpu.status("Halted");
                if let Some(ref program_info) = program_info {
                    program_info.save_dump(&cpu.memory)?;
                }
                cpu.on_halted();
                return Ok(());
            }
            _ => panic!("Break at unimplemented subroutine {:04X}", addr),
        }

        cycles += match RTI.func {
            OpFunc::NoOperand(f) => f(cpu),
            _ => unreachable!(),
        };
    }
}

// Set up operating system routine
fn set_brk(cpu: &mut Cpu, addr: u16) {
    cpu.store(addr, BRK.opcode); // Software interrupt
    cpu.store(addr + 1, NOP.opcode); // Padding
    cpu.store(addr + 2, RTS.opcode); // Return
}
