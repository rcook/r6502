macro_rules! no_operand_op {
    ($opcode: ident, $addressing_mode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::$addressing_mode,
            op: $crate::Op::NoOperand($crate::NoOperandOp::new($crate::ops::$f)),
        }
    };
}

pub(crate) use no_operand_op;

macro_rules! byte_op {
    ($opcode: ident, $addressing_mode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::$addressing_mode,
            op: $crate::Op::Byte($crate::ByteOp::new($crate::ops::$f)),
        }
    };
}

pub(crate) use byte_op;

macro_rules! word_op {
    ($opcode: ident, $addressing_mode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::$addressing_mode,
            op: $crate::Op::Word($crate::WordOp::new($crate::ops::$f)),
        }
    };
}

pub(crate) use word_op;

macro_rules! absolute {
    ($opcode: ident, $f: ident) => {
        $crate::op_info::macros::word_op!($opcode, Absolute, $f)
    };
}

pub(crate) use absolute;

macro_rules! immediate {
    ($opcode: ident, $f: ident) => {
        $crate::op_info::macros::byte_op!($opcode, Immediate, $f)
    };
}

pub(crate) use immediate;

macro_rules! implied {
    ($opcode: ident, $f: ident) => {
        $crate::op_info::macros::no_operand_op!($opcode, Implied, $f)
    };
}

pub(crate) use implied;

macro_rules! indirect {
    ($opcode: ident, $f: ident) => {
        $crate::op_info::macros::word_op!($opcode, Indirect, $f)
    };
}

pub(crate) use indirect;

macro_rules! relative {
    ($opcode: ident, $f: ident) => {
        $crate::op_info::macros::byte_op!($opcode, Relative, $f)
    };
}

pub(crate) use relative;

macro_rules! absolute_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::Absolute,
            op: $crate::Op::Word($crate::WordOp::new($crate::op_info::wrappers::absolute::$f)),
        }
    };
}

pub(crate) use absolute_wrapped;

macro_rules! absolute_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::AbsoluteX,
            op: $crate::Op::Word($crate::WordOp::new(
                $crate::op_info::wrappers::absolute_x::$f,
            )),
        }
    };
}

pub(crate) use absolute_x_wrapped;

macro_rules! absolute_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::AbsoluteY,
            op: $crate::Op::Word($crate::WordOp::new(
                $crate::op_info::wrappers::absolute_y::$f,
            )),
        }
    };
}

pub(crate) use absolute_y_wrapped;

macro_rules! indexed_indirect_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::IndexedIndirectX,
            op: $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::indexed_indirect_x::$f,
            )),
        }
    };
}

pub(crate) use indexed_indirect_x_wrapped;

macro_rules! indirect_indexed_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::IndirectIndexedY,
            op: $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::indirect_indexed_y::$f,
            )),
        }
    };
}

pub(crate) use indirect_indexed_y_wrapped;

macro_rules! zero_page_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::ZeroPage,
            op: $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page::$f,
            )),
        }
    };
}

pub(crate) use zero_page_wrapped;

macro_rules! zero_page_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::ZeroPageX,
            op: $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page_x::$f,
            )),
        }
    };
}

pub(crate) use zero_page_x_wrapped;

macro_rules! zero_page_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::OpInfo {
            opcode: $crate::Opcode::$opcode,
            addressing_mode: $crate::AddressingMode::ZeroPageY,
            op: $crate::Op::Byte($crate::ByteOp::new(
                $crate::op_info::wrappers::zero_page_y::$f,
            )),
        }
    };
}

pub(crate) use zero_page_y_wrapped;
