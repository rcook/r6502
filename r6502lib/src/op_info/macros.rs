macro_rules! absolute_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Absolute,
            $crate::Op::Word($crate::WordOp::new($crate::op_info::wrappers::absolute::$f)),
        )
    };
}

pub(crate) use absolute_wrapped;

macro_rules! absolute_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::AbsoluteX,
            $crate::Op::Word($crate::WordOp::new(
                $crate::op_info::wrappers::absolute_x::$f,
            )),
        )
    };
}

pub(crate) use absolute_x_wrapped;

macro_rules! absolute_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::AbsoluteY,
            $crate::Op::Word($crate::WordOp::new(
                $crate::op_info::wrappers::absolute_y::$f,
            )),
        )
    };
}

pub(crate) use absolute_y_wrapped;

macro_rules! accumulator_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Accumulator,
            $crate::Op::NoOperand($crate::NoOperandOp::new(
                $crate::op_info::wrappers::accumulator::$f,
            )),
        )
    };
}

pub(crate) use accumulator_wrapped;

macro_rules! immediate_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Immediate,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::immediate::$f,
            )),
        )
    };
}

pub(crate) use immediate_wrapped;

macro_rules! implied_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Implied,
            $crate::Op::NoOperand($crate::NoOperandOp::new(
                $crate::op_info::wrappers::implied::$f,
            )),
        )
    };
}

pub(crate) use implied_wrapped;

macro_rules! indexed_indirect_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::IndexedIndirectX,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::indexed_indirect_x::$f,
            )),
        )
    };
}

pub(crate) use indexed_indirect_x_wrapped;

macro_rules! indirect_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Indirect,
            $crate::Op::Word($crate::WordOp::new($crate::op_info::wrappers::indirect::$f)),
        )
    };
}

pub(crate) use indirect_wrapped;

macro_rules! indirect_indexed_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::IndirectIndexedY,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::indirect_indexed_y::$f,
            )),
        )
    };
}

pub(crate) use indirect_indexed_y_wrapped;

macro_rules! relative_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::Relative,
            $crate::Op::Byte($crate::ByteOp::new($crate::op_info::wrappers::relative::$f)),
        )
    };
}

pub(crate) use relative_wrapped;

macro_rules! zero_page_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::ZeroPage,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page::$f,
            )),
        )
    };
}

pub(crate) use zero_page_wrapped;

macro_rules! zero_page_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::ZeroPageX,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page_x::$f,
            )),
        )
    };
}

pub(crate) use zero_page_x_wrapped;

macro_rules! zero_page_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::AddressingMode::ZeroPageY,
            $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page_y::$f,
            )),
        )
    };
}

pub(crate) use zero_page_y_wrapped;
