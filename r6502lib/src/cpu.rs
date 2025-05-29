use crate::{absolute, adc, jmp, nop, zero_page, Op, OpByte, OpWord, Opcode};
use std::collections::HashMap;

pub(crate) struct Cpu(HashMap<Opcode, Op>);

impl Cpu {
    // http://www.6502.org/users/obelisk/6502/instructions.html
    #[allow(unused)]
    pub(crate) fn make_6502() -> Self {
        use crate::Opcode::*;

        Self(HashMap::from([
            (
                AdcAbs,
                Op::Word(OpWord::Wrapped {
                    wrapper: absolute,
                    f: adc,
                }),
            ),
            (AdcImm, Op::Byte(OpByte::Simple { f: adc })),
            (
                AdcZp,
                Op::Byte(OpByte::Wrapped {
                    wrapper: zero_page,
                    f: adc,
                }),
            ),
            (JmpAbs, Op::Word(OpWord::Simple { f: jmp })),
            (Nop, Op::NoOperand { f: nop }),
        ]))
    }

    pub(crate) fn get_op(&self, opcode: &Opcode) -> Option<&Op> {
        self.0.get(opcode)
    }
}
