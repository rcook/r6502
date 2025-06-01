macro_rules! flag_fn {
    ($name: ident, $flag: ident, $value: expr) => {
        pub(crate) fn $name(s: &mut $crate::VmState) -> $crate::OpCycles {
            $crate::p_set!(s.reg, $flag, $value);
            2
        }
    };
}

flag_fn!(clc, C, false);
flag_fn!(cld, D, false);
flag_fn!(cli, I, false);
flag_fn!(clv, V, false);
flag_fn!(sec, C, true);
flag_fn!(sed, D, true);
flag_fn!(sei, I, true);
