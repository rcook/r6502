#[macro_export]
macro_rules! absolute_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Absolute,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! absolute_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::AbsoluteX,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute_x::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! absolute_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::AbsoluteY,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::absolute_y::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! accumulator_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Accumulator,
            $crate::emulator::Op::NoOperand($crate::emulator::NoOperandOp::new(
                $crate::emulator::op_info::wrappers::accumulator::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! immediate_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Immediate,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::immediate::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! implied_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Implied,
            $crate::emulator::Op::NoOperand($crate::emulator::NoOperandOp::new(
                $crate::emulator::op_info::wrappers::implied::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! indexed_indirect_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::IndexedIndirectX,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::indexed_indirect_x::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! indirect_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Indirect,
            $crate::emulator::Op::Word($crate::emulator::WordOp::new(
                $crate::emulator::op_info::wrappers::indirect::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! indirect_indexed_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::IndirectIndexedY,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::indirect_indexed_y::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! relative_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::Relative,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::relative::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! zero_page_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPage,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! zero_page_x_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPageX,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page_x::$f,
            )),
        )
    };
}

#[macro_export]
macro_rules! zero_page_y_wrapped {
    ($opcode: ident, $f: ident) => {
        $crate::emulator::OpInfo::new(
            $crate::Opcode::$opcode,
            $crate::emulator::AddressingMode::ZeroPageY,
            $crate::emulator::Op::Byte($crate::emulator::ByteOp::new(
                $crate::emulator::op_info::wrappers::zero_page_y::$f,
            )),
        )
    };
}
