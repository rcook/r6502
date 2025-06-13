macro_rules! absolute_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Absolute,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute::$f,
            )),
        )
    };
}

pub(crate) use absolute_wrapped;

macro_rules! absolute_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::AbsoluteX,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute_x::$f,
            )),
        )
    };
}

pub(crate) use absolute_x_wrapped;

macro_rules! absolute_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::AbsoluteY,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute_y::$f,
            )),
        )
    };
}

pub(crate) use absolute_y_wrapped;

macro_rules! accumulator_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Accumulator,
            $crate::emulator::Op::NoOperand($crate::emulator::NoOperandOp::new(
                $crate::emulator::op_info::wrappers::accumulator::$f,
            )),
        )
    };
}

pub(crate) use accumulator_wrapped;

macro_rules! immediate_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Immediate,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::immediate::$f,
            )),
        )
    };
}

pub(crate) use immediate_wrapped;

macro_rules! implied_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Implied,
            $crate::emulator::Op::NoOperand($crate::emulator::NoOperandOp::new(
                $crate::emulator::op_info::wrappers::implied::$f,
            )),
        )
    };
}

pub(crate) use implied_wrapped;

macro_rules! indexed_indirect_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::IndexedIndirectX,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::indexed_indirect_x::$f,
            )),
        )
    };
}

pub(crate) use indexed_indirect_x_wrapped;

macro_rules! indirect_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Indirect,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::indirect::$f,
            )),
        )
    };
}

pub(crate) use indirect_wrapped;

macro_rules! indirect_indexed_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::IndirectIndexedY,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::indirect_indexed_y::$f,
            )),
        )
    };
}

pub(crate) use indirect_indexed_y_wrapped;

macro_rules! relative_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::Relative,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::relative::$f,
            )),
        )
    };
}

pub(crate) use relative_wrapped;

macro_rules! zero_page_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPage,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page::$f,
            )),
        )
    };
}

pub(crate) use zero_page_wrapped;

macro_rules! zero_page_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPageX,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page_x::$f,
            )),
        )
    };
}

pub(crate) use zero_page_x_wrapped;

macro_rules! zero_page_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::emulator::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPageY,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page_y::$f,
            )),
        )
    };
}

pub(crate) use zero_page_y_wrapped;
