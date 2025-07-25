pub mod absolute {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, cpu.bus.load(addr));
                $cycles
            }
        };
    }

    macro_rules! wrap_jump {
        ($f: ident, $cycles: expr) => {
            #[allow(clippy::missing_const_for_fn)]
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, addr);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, addr);
                $cycles
            }
        };
    }

    wrap!(adc, 4);
    wrap!(and, 4);
    wrap_store!(asl, 6);
    wrap!(bit, 4);
    wrap!(cmp, 4);
    wrap!(cpx, 4);
    wrap!(cpy, 4);
    wrap_store!(dec, 6);
    wrap!(eor, 4);
    wrap_store!(inc, 6);
    wrap_jump!(jmp, 3);
    wrap_jump!(jsr, 6);
    wrap!(lda, 4);
    wrap!(ldx, 4);
    wrap!(ldy, 4);
    wrap_store!(lsr, 6);
    wrap!(ora, 4);
    wrap_store!(rol, 6);
    wrap_store!(ror, 6);
    wrap!(sbc, 4);
    wrap_store!(sta, 4);
    wrap_store!(stx, 4);
    wrap_store!(sty, 4);
}

pub mod absolute_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(u16::from(cpu.reg.x));
                $crate::emulator::ops::$f(cpu, cpu.bus.load(effective_addr));
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(u16::from(cpu.reg.x));
                $crate::emulator::ops::$f(cpu, effective_addr);
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 4, 5);
    wrap_store!(asl, 7, 7);
    wrap!(and, 4, 5);
    wrap!(cmp, 4, 5);
    wrap_store!(dec, 7, 7);
    wrap!(eor, 4, 5);
    wrap_store!(inc, 7, 7);
    wrap!(lda, 4, 5);
    wrap!(ldy, 4, 5);
    wrap_store!(lsr, 7, 7);
    wrap!(ora, 4, 5);
    wrap_store!(rol, 7, 7);
    wrap_store!(ror, 7, 7);
    wrap!(sbc, 4, 5);
    wrap_store!(sta, 5, 5);
}

pub mod absolute_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(u16::from(cpu.reg.y));
                $crate::emulator::ops::$f(cpu, cpu.bus.load(effective_addr));
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(u16::from(cpu.reg.y));
                $crate::emulator::ops::$f(cpu, effective_addr);
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 4, 5);
    wrap!(and, 4, 5);
    wrap!(cmp, 4, 5);
    wrap!(eor, 4, 5);
    wrap!(lda, 4, 5);
    wrap!(ldx, 4, 5);
    wrap!(ora, 4, 5);
    wrap!(sbc, 4, 5);
    wrap_store!(sta, 5, 5);
}

pub mod accumulator {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu);
                $cycles
            }
        };
    }

    wrap!(asl_acc, 2);
    wrap!(lsr_acc, 2);
    wrap!(rol_acc, 2);
    wrap!(ror_acc, 2);
}

pub mod immediate {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, value: u8) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, value);
                $cycles
            }
        };
    }

    wrap!(adc, 2);
    wrap!(and, 2);
    wrap!(cmp, 2);
    wrap!(cpx, 2);
    wrap!(cpy, 2);
    wrap!(eor, 2);
    wrap!(lda, 2);
    wrap!(ldx, 2);
    wrap!(ldy, 2);
    wrap!(ora, 2);
    wrap!(sbc, 2);
}

pub mod implied {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            #[allow(clippy::missing_const_for_fn)]
            pub fn $f(cpu: &mut $crate::emulator::Cpu) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu);
                $cycles
            }
        };
    }

    wrap!(brk, 7);
    wrap!(clc, 2);
    wrap!(cld, 2);
    wrap!(cli, 2);
    wrap!(clv, 2);
    wrap!(dex, 2);
    wrap!(dey, 2);
    wrap!(inx, 2);
    wrap!(iny, 2);
    wrap!(nop, 2);
    wrap!(pha, 3);
    wrap!(php, 3);
    wrap!(pla, 4);
    wrap!(plp, 4);
    wrap!(rti, 6);
    wrap!(rts, 6);
    wrap!(sec, 2);
    wrap!(sed, 2);
    wrap!(sei, 2);
    wrap!(tax, 2);
    wrap!(tay, 2);
    wrap!(tsx, 2);
    wrap!(txa, 2);
    wrap!(txs, 2);
    wrap!(tya, 2);
}

pub mod indexed_indirect_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr =
                    $crate::emulator::address_util::compute_effective_addr_indexed_indirect_x(
                        cpu, addr,
                    );
                $crate::emulator::ops::$f(cpu, cpu.bus.load(effective_addr));
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr =
                    $crate::emulator::address_util::compute_effective_addr_indexed_indirect_x(
                        cpu, addr,
                    );
                $crate::emulator::ops::$f(cpu, effective_addr);
                $cycles
            }
        };
    }

    wrap!(adc, 6);
    wrap!(and, 6);
    wrap!(cmp, 6);
    wrap!(eor, 6);
    wrap!(lda, 6);
    wrap!(ora, 6);
    wrap!(sbc, 6);
    wrap_store!(sta, 6);
}

pub mod indirect {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u16) -> $crate::OpCycles {
                // http://www.6502.org/tutorials/6502opcodes.html
                // "AN INDIRECT JUMP MUST NEVER USE A VECTOR BEGINNING ON THE LAST BYTE OF A PAGE"
                let lo_addr = cpu.bus.load(addr);
                let hi_addr = cpu
                    .bus
                    .load((addr & 0xff00) + ((addr & 0x00ff).wrapping_add(1) & 0x00ff));
                let effective_addr = (u16::from(hi_addr) << 8) + u16::from(lo_addr);
                $crate::emulator::ops::$f(cpu, effective_addr);
                $cycles
            }
        };
    }

    wrap!(jmp, 5);
}

pub mod indirect_indexed_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr =
                    $crate::emulator::address_util::compute_effective_addr_indirect_indexed_y(
                        cpu, addr,
                    );
                $crate::emulator::ops::$f(cpu, cpu.bus.load(effective_addr));
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr =
                    $crate::emulator::address_util::compute_effective_addr_indirect_indexed_y(
                        cpu, addr,
                    );
                $crate::emulator::ops::$f(cpu, effective_addr);
                if r6502lib::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 5, 6);
    wrap!(and, 5, 6);
    wrap!(cmp, 5, 6);
    wrap!(eor, 5, 6);
    wrap!(lda, 5, 6);
    wrap!(ora, 5, 6);
    wrap!(sbc, 5, 6);
    wrap_store!(sta, 5, 6);
}

pub mod relative {
    macro_rules! wrap {
        ($f: ident, $not_taken_cycles: expr, $taken_cycles: expr, $taken_cross_page_cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, offset: u8) -> $crate::OpCycles {
                match $crate::emulator::ops::$f(cpu, offset) {
                    $crate::emulator::ops::BranchResult::NotTaken => $not_taken_cycles,
                    $crate::emulator::ops::BranchResult::Taken => $not_taken_cycles,
                    $crate::emulator::ops::BranchResult::TakenCrossPage => $taken_cross_page_cycles,
                }
            }
        };
    }

    wrap!(bcc, 2, 3, 4);
    wrap!(bcs, 2, 3, 4);
    wrap!(beq, 2, 3, 4);
    wrap!(bmi, 2, 3, 4);
    wrap!(bne, 2, 3, 4);
    wrap!(bpl, 2, 3, 4);
    wrap!(bvc, 2, 3, 4);
    wrap!(bvs, 2, 3, 4);
}

pub mod zero_page {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, cpu.bus.load(u16::from(addr)));
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                $crate::emulator::ops::$f(cpu, u16::from(addr));
                $cycles
            }
        };
    }

    wrap!(adc, 3);
    wrap!(and, 3);
    wrap_store!(asl, 5);
    wrap!(bit, 3);
    wrap!(cmp, 3);
    wrap!(cpx, 3);
    wrap!(cpy, 3);
    wrap_store!(dec, 5);
    wrap!(eor, 3);
    wrap_store!(inc, 5);
    wrap!(lda, 3);
    wrap!(ldx, 3);
    wrap!(ldy, 3);
    wrap_store!(lsr, 5);
    wrap!(ora, 3);
    wrap_store!(rol, 5);
    wrap_store!(ror, 5);
    wrap!(sbc, 3);
    wrap_store!(sta, 3);
    wrap_store!(stx, 3);
    wrap_store!(sty, 3);
}

pub mod zero_page_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(cpu.reg.x);
                $crate::emulator::ops::$f(cpu, cpu.bus.load(u16::from(effective_addr)));
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(cpu.reg.x);
                $crate::emulator::ops::$f(cpu, u16::from(effective_addr));
                $cycles
            }
        };
    }

    wrap!(adc, 4);
    wrap!(and, 4);
    wrap_store!(asl, 6);
    wrap!(cmp, 4);
    wrap_store!(dec, 6);
    wrap!(eor, 4);
    wrap_store!(inc, 6);
    wrap!(lda, 4);
    wrap!(ldy, 4);
    wrap_store!(lsr, 6);
    wrap!(ora, 4);
    wrap_store!(rol, 6);
    wrap_store!(ror, 6);
    wrap!(sbc, 4);
    wrap_store!(sta, 4);
    wrap_store!(sty, 4);
}

pub mod zero_page_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(cpu.reg.y);
                $crate::emulator::ops::$f(cpu, cpu.bus.load(u16::from(effective_addr)));
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub fn $f(cpu: &mut $crate::emulator::Cpu, addr: u8) -> $crate::OpCycles {
                let effective_addr = addr.wrapping_add(cpu.reg.y);
                $crate::emulator::ops::$f(cpu, u16::from(effective_addr));
                $cycles
            }
        };
    }

    wrap!(ldx, 4);
    wrap_store!(stx, 4);
}
