use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{
    Cpu, CpuMessage, Flag, Instruction, Op, OpFunc, ProgramInfo, RegisterFile, Status, UIMessage,
    IRQ, OPS, OSHALT, OSWRCH, STACK_BASE,
};
use anyhow::{bail, Result};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub(crate) fn run_vm(
    cpu_rx: Receiver<CpuMessage>,
    ui_tx: Sender<UIMessage>,
    program_info: Option<ProgramInfo>,
) -> Result<()> {
    let mut free_running = false;

    let mut cpu = Cpu::new();

    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    if let Some(ref program_info) = program_info {
        program_info.load(&mut cpu.memory)?;
        cpu.reg.pc = program_info.start();
    }

    // Set up interrupt vectors
    cpu.store_word(IRQ, IRQ_VALUE);

    // Set up operating system handlers
    set_brk(&mut cpu, OSWRCH);
    set_brk(&mut cpu, OSHALT);

    // Initialize the state
    cpu.push_word(OSHALT - 1);

    let mut cycles = 0;

    loop {
        while !cpu.get_flag(Flag::B) {
            let opcode = cpu.next();
            let instruction = match ops[opcode as usize] {
                Some(op) => match op.func {
                    OpFunc::NoOperand(f) => Instruction::NoOperand(op, f),
                    OpFunc::Byte(f) => Instruction::Byte(op, f, cpu.next()),
                    OpFunc::Word(f) => Instruction::Word(op, f, cpu.next_word()),
                },
                None => bail!("Unsupported opcode {opcode:02X}"),
            };

            report_before_execute(&ui_tx, cpu.reg.clone(), cycles, &instruction);

            if !poll(&cpu_rx, &mut free_running) {
                // Handle disconnection
                return Ok(());
            }

            cycles += match instruction {
                Instruction::NoOperand(_, f) => f(&mut cpu),
                Instruction::Byte(_, f, operand) => f(&mut cpu, operand),
                Instruction::Word(_, f, operand) => f(&mut cpu, operand),
            };

            report_after_execute(&ui_tx, cpu.reg.clone(), cycles, &instruction);
        }

        // Check for expected interrupt request value
        if cpu.reg.pc != IRQ_VALUE {
            bail!("Unexpected IRQ value {:04X}", cpu.reg.pc);
        }

        // Address of operating system routine being invoked
        let addr = cpu.fetch_word(STACK_BASE + (cpu.reg.s + 2) as u16) - 1;

        match addr {
            OSWRCH => {
                let c = cpu.reg.a as char;
                write_stdout(&ui_tx, c);
            }
            OSHALT => {
                report_status(&ui_tx, Status::Halted);
                if let Some(ref program_info) = program_info {
                    program_info.save_dump(&cpu.memory)?;
                }
                return Ok(());
            }
            _ => panic!("Break at unimplemented subroutine {:04X}", addr),
        }

        cycles += match RTI.func {
            OpFunc::NoOperand(f) => f(&mut cpu),
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

fn report_before_execute(
    ui_tx: &Sender<UIMessage>,
    reg: RegisterFile,
    cycles: u32,
    instruction: &Instruction,
) {
    ui_tx
        .send(UIMessage::BeforeExecute(reg, cycles, instruction.clone()))
        .expect("Must succeed")
}

fn report_after_execute(
    ui_tx: &Sender<UIMessage>,
    reg: RegisterFile,
    cycles: u32,
    instruction: &Instruction,
) {
    ui_tx
        .send(UIMessage::AfterExecute(reg, cycles, instruction.clone()))
        .expect("Must succeed")
}

fn report_status(ui_tx: &Sender<UIMessage>, status: Status) {
    ui_tx.send(UIMessage::Status(status)).expect("Must succeed")
}

fn write_stdout(ui_tx: &Sender<UIMessage>, c: char) {
    ui_tx.send(UIMessage::WriteStdout(c)).expect("Must succeed")
}

fn poll(cpu_rx: &Receiver<CpuMessage>, free_running: &mut bool) -> bool {
    loop {
        if *free_running {
            match cpu_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return false,
                Err(TryRecvError::Empty) => return true,
                Ok(CpuMessage::Step) => {}
                Ok(CpuMessage::Run) => {}
                Ok(CpuMessage::Break) => *free_running = false,
            }
        } else {
            match cpu_rx.recv() {
                Err(_) => return false,
                Ok(CpuMessage::Step) => return true,
                Ok(CpuMessage::Run) => *free_running = true,
                Ok(CpuMessage::Break) => {}
            }
        }
    }
}
