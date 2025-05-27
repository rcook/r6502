use crate::constants::IRQ_VALUE;
use crate::ops::{BRK, NOP, RTI, RTS};
use crate::{
    DebugMessage, Flag, Instruction, MachineState, Op, OpFunc, ProgramInfo, RegisterFile, Status,
    StatusMessage, IRQ, OPS, OSHALT, OSWRCH, STACK_BASE,
};
use anyhow::{bail, Result};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub(crate) fn run_vm(
    debug_rx: Receiver<DebugMessage>,
    status_tx: Sender<StatusMessage>,
    program_info: Option<ProgramInfo>,
) -> Result<()> {
    let mut free_running = false;

    let mut m = MachineState::new();

    let ops = {
        let mut ops: [Option<Op>; 256] = [None; 256];
        for op in OPS {
            ops[op.opcode as usize] = Some(op)
        }
        ops
    };

    if let Some(ref program_info) = program_info {
        program_info.load(&mut m.memory)?;
        m.reg.pc = program_info.start();
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
            let opcode = m.next();
            let instruction = match ops[opcode as usize] {
                Some(op) => match op.func {
                    OpFunc::NoOperand(f) => Instruction::NoOperand(op, f),
                    OpFunc::Byte(f) => Instruction::Byte(op, f, m.next()),
                    OpFunc::Word(f) => Instruction::Word(op, f, m.next_word()),
                },
                None => bail!("Unsupported opcode {opcode:02X}"),
            };

            report_before_execute(&status_tx, m.reg.clone(), cycles, &instruction);

            if !poll(&debug_rx, &mut free_running) {
                // Handle disconnection
                return Ok(());
            }

            cycles += match instruction {
                Instruction::NoOperand(_, f) => f(&mut m),
                Instruction::Byte(_, f, operand) => f(&mut m, operand),
                Instruction::Word(_, f, operand) => f(&mut m, operand),
            };

            report_after_execute(&status_tx, m.reg.clone(), cycles, &instruction);
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
                write_stdout(&status_tx, c);
            }
            OSHALT => {
                report_status(&status_tx, Status::Halted);
                if let Some(ref program_info) = program_info {
                    program_info.save_dump(&m.memory)?;
                }
                return Ok(());
            }
            _ => panic!("Break at unimplemented subroutine {:04X}", addr),
        }

        cycles += match RTI.func {
            OpFunc::NoOperand(f) => f(&mut m),
            _ => unreachable!(),
        };
    }
}

// Set up operating system routine
fn set_brk(m: &mut MachineState, addr: u16) {
    m.store(addr, BRK.opcode); // Software interrupt
    m.store(addr + 1, NOP.opcode); // Padding
    m.store(addr + 2, RTS.opcode); // Return
}

fn report_before_execute(
    status_tx: &Sender<StatusMessage>,
    reg: RegisterFile,
    cycles: u32,
    instruction: &Instruction,
) {
    status_tx
        .send(StatusMessage::BeforeExecute(
            reg,
            cycles,
            instruction.clone(),
        ))
        .expect("Must succeed")
}

fn report_after_execute(
    status_tx: &Sender<StatusMessage>,
    reg: RegisterFile,
    cycles: u32,
    instruction: &Instruction,
) {
    status_tx
        .send(StatusMessage::AfterExecute(
            reg,
            cycles,
            instruction.clone(),
        ))
        .expect("Must succeed")
}

fn report_status(status_tx: &Sender<StatusMessage>, status: Status) {
    status_tx
        .send(StatusMessage::Status(status))
        .expect("Must succeed")
}

fn write_stdout(status_tx: &Sender<StatusMessage>, c: char) {
    status_tx
        .send(StatusMessage::WriteStdout(c))
        .expect("Must succeed")
}

fn poll(debug_rx: &Receiver<DebugMessage>, free_running: &mut bool) -> bool {
    loop {
        if *free_running {
            match debug_rx.try_recv() {
                Err(TryRecvError::Disconnected) => return false,
                Err(TryRecvError::Empty) => return true,
                Ok(DebugMessage::Step) => {}
                Ok(DebugMessage::Run) => {}
                Ok(DebugMessage::Break) => *free_running = false,
            }
        } else {
            match debug_rx.recv() {
                Err(_) => return false,
                Ok(DebugMessage::Step) => return true,
                Ok(DebugMessage::Run) => *free_running = true,
                Ok(DebugMessage::Break) => {}
            }
        }
    }
}
