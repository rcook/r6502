pub(crate) mod absolute {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                _ = $crate::ops::$f(s, s.memory[addr]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                $crate::ops::$f(s, addr);
                $cycles
            }
        };
    }

    wrap!(adc, 4);
    wrap!(cmp, 4);
    wrap!(cpx, 4);
    wrap!(cpy, 4);
    wrap!(lda, 4);
    wrap!(ldx, 4);
    wrap!(ldy, 4);
    wrap!(sbc, 4);
    wrap_store!(sta, 4);
}

pub(crate) mod absolute_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.x as u16);
                _ = $crate::ops::$f(s, s.memory[effective_addr]);
                if $crate::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.x as u16);
                $crate::ops::$f(s, effective_addr);
                if $crate::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 4, 5);
    wrap!(cmp, 4, 5);
    wrap!(lda, 4, 5);
    wrap!(ldy, 4, 5);
    wrap!(sbc, 4, 5);
    wrap_store!(sta, 5, 5);
}

pub(crate) mod absolute_y {
    #[allow(unused)]
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.y as u16);
                _ = $crate::ops::$f(s, s.memory[effective_addr]);
                if $crate::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u16) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.y as u16);
                $crate::ops::$f(s, effective_addr);
                if $crate::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 4, 5);
    wrap!(cmp, 4, 5);
    wrap!(lda, 4, 5);
    wrap!(ldx, 4, 5);
    wrap!(sbc, 4, 5);
    wrap_store!(sta, 5, 5);
}

pub(crate) mod indexed_indirect_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                let effective_addr = s.memory.fetch_word(addr.wrapping_add(s.reg.x) as u16);
                _ = $crate::ops::$f(s, s.memory[effective_addr]);
                $cycles
            }
        };
    }

    wrap!(adc, 6);
    wrap!(cmp, 6);
    wrap!(lda, 6);
    wrap!(sbc, 6);
}

pub(crate) mod indirect_indexed_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr, $cross_page_cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                let effective_addr = s
                    .memory
                    .fetch_word(addr as u16)
                    .wrapping_add(s.reg.y as u16);
                _ = $crate::ops::$f(s, s.memory[effective_addr]);
                if $crate::util::crosses_page_boundary(effective_addr) {
                    $cross_page_cycles
                } else {
                    $cycles
                }
            }
        };
    }

    wrap!(adc, 5, 6);
    wrap!(cmp, 5, 6);
    wrap!(lda, 5, 6);
    wrap!(sbc, 5, 6);
}

pub(crate) mod zero_page {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                _ = $crate::ops::$f(s, s.memory[addr as u16]);
                $cycles
            }
        };
    }

    macro_rules! wrap_store {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                $crate::ops::$f(s, addr as u16);
                $cycles
            }
        };
    }

    wrap!(adc, 3);
    wrap!(cmp, 3);
    wrap!(cpx, 3);
    wrap!(cpy, 3);
    wrap!(lda, 3);
    wrap!(ldx, 3);
    wrap!(ldy, 3);
    wrap!(sbc, 3);
    wrap_store!(sta, 3);
}

pub(crate) mod zero_page_x {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.x);
                _ = $crate::ops::$f(s, s.memory[effective_addr as u16]);
                $cycles
            }
        };
    }

    wrap!(adc, 4);
    wrap!(cmp, 4);
    wrap!(lda, 4);
    wrap!(sbc, 4);
}

pub(crate) mod zero_page_y {
    macro_rules! wrap {
        ($f: ident, $cycles: expr) => {
            pub(crate) fn $f(s: &mut $crate::VmState, addr: u8) -> $crate::Cycles {
                let effective_addr = addr.wrapping_add(s.reg.y);
                _ = $crate::ops::$f(s, s.memory[effective_addr as u16]);
                $cycles
            }
        };
    }

    wrap!(ldx, 4);
}
