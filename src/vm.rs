use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{split_word, Cpu, Flag, Op, OpFunc, IRQ, OPS, OSHALT, OSWRCH, STACK_BASE};
use anyhow::{bail, Result};

pub(crate) fn run_vm(cpu: &mut Cpu) -> Result<()> {
    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    // Set up interrupt vectors
    cpu.store_word(IRQ, IRQ_VALUE);

    // Set up operating system handlers
    set_brk(cpu, OSWRCH);
    set_brk(cpu, OSHALT);

    // Initialize the state
    cpu.push_word(OSHALT - 1);

    loop {
        while !cpu.get_flag(Flag::B) {
            cpu.show_registers();
            if !cpu.poll() {
                // Handle disconnection
                return Ok(());
            }
            let opcode = cpu.next();
            match ops[opcode as usize] {
                Some(op) => match op.func {
                    OpFunc::NoArgs(f) => {
                        cpu.println(&format!(
                            "{:02X}       {} {:?}",
                            op.opcode, op.mnemonic, op.addressing_mode
                        ));
                        f(cpu)
                    }
                    OpFunc::Byte(f) => {
                        let operand = cpu.next();
                        cpu.println(&format!(
                            "{:02X} {:02X}    {} {:?}",
                            op.opcode, operand, op.mnemonic, op.addressing_mode
                        ));
                        f(cpu, operand)
                    }
                    OpFunc::Word(f) => {
                        let operand = cpu.next_word();
                        let (hi, lo) = split_word(operand);
                        cpu.println(&format!(
                            "{:02X} {:02X} {:02X} {} {:?}",
                            op.opcode, lo, hi, op.mnemonic, op.addressing_mode
                        ));
                        f(cpu, operand)
                    }
                },
                None => bail!("Unsupported opcode {opcode:02X}"),
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
                cpu.println("Halted");
                return Ok(());
            }
            _ => panic!("Break at subroutine {:04X}", addr),
        }

        match RTI.func {
            OpFunc::NoArgs(f) => f(cpu),
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
