macro_rules! flag_fn {
    ($name: ident, $flag: ident, $value: expr) => {
        pub fn $name(cpu: &mut $crate::emulator::Cpu) {
            $crate::p_set!(cpu.reg, $flag, $value);
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
