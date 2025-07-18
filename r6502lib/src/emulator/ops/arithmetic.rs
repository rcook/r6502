use crate::emulator::Cpu;
use crate::emulator::ops::helper::{is_carry, is_neg, is_overflow, is_zero};
use crate::num::Truncate;
use crate::{p_get, p_set, p_value};

// http://www.6502.org/tutorials/6502opcodes.html#ADC
// http://www.6502.org/users/obelisk/6502/reference.html#ADC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
pub fn adc(cpu: &mut Cpu, operand: u8) {
    if p_get!(cpu.reg, D) {
        let a = i32::from(cpu.reg.a);
        let value = i32::from(operand);
        let carry = p_value!(cpu.reg, C);

        let mut ah = 0;
        let tempb = (a + value + carry) & 0xff;
        p_set!(cpu.reg, Z, tempb == 0);
        let mut al = (a & 0xf) + (value & 0xf) + carry;
        if al > 9 {
            al -= 10;
            al &= 0xf;
            ah = 1;
        }
        ah += (a >> 4) + (value >> 4);
        p_set!(cpu.reg, N, (ah & 8) != 0);
        p_set!(
            cpu.reg,
            V,
            ((a ^ value) & 0x80) == 0 && (((a ^ (ah << 4)) & 0x80) != 0)
        );
        p_set!(cpu.reg, C, false);
        if ah > 9 {
            p_set!(cpu.reg, C, true);
            ah -= 10;
            ah &= 0xf;
        }
        cpu.reg.a = (u8::truncate(al) & 0xf) | (u8::truncate(ah) << 4);
    } else {
        let lhs = cpu.reg.a;
        let rhs = operand;

        let result_word = u16::from(lhs) + u16::from(rhs) + p_value!(cpu.reg, C);
        let result = u8::truncate(result_word);

        cpu.reg.a = result;
        p_set!(cpu.reg, N, is_neg(result));
        p_set!(cpu.reg, V, is_overflow(lhs, rhs, result));
        p_set!(cpu.reg, Z, is_zero(result));
        p_set!(cpu.reg, C, is_carry(result_word));
    }
}

/*
"MOS 6502 Emulator" refers to https://itema-as.github.io/6502js/
"Visual6502" refers to http://visual6502.org/JSSim/expert.html

Visual6502 is more or less authoritative and agress with Tom Harte tests. MOS 6502 Emulator does not match.

Conclusion: We're not calculating N properly

"e9 c4 08"
    SED
    SEI
    SEC
    LDA #$40
    STA $1000
    BIT $1000
    PHP
    LDA #$9c
    PLP         ; A=$9c, (MOS 6502 Emulator: P=0b01x11101) (Visual6502: P=nV‑BDIzC)
    SBC #$c4    ; A=$78, (MOS 6502 Emulator: P=0b00111100) (Visual6502: P=Nv‑BDIzc)

Address  Hexdump   Dissassembly
-------------------------------
$0600    f8        SED
$0601    78        SEI
$0602    38        SEC
$0603    a9 40     LDA #$40
$0605    8d 00 10  STA $1000
$0608    2c 00 10  BIT $1000
$060b    08        PHP
$060c    a9 9c     LDA #$9c
$060e    28        PLP
$060f    e9 c4     SBC #$c4

Initial:
  pc: $0084 (132)
  s : $26  (38)
  a : $9C  (156)
  x : $72  (114)
  y : $CC  (204)
        NV1BDIZC
  p : 0b01101101  ($6D) (109)
    0084 E9 (233)
    0085 C4 (196)
    0086 08 (8)
Final:
  pc: $0086 (134)
  s : $26  (38)
  a : $78  (120)
  x : $72  (114)
  y : $CC  (204)
        NV1BDIZC
  p : 0b10101100  ($AC) (172)
    0084 E9 (233)
    0085 C4 (196)
    0086 08 (8)
*/

// http://www.6502.org/tutorials/6502opcodes.html#SBC
// http://www.6502.org/users/obelisk/6502/reference.html#SBC
// https://stackoverflow.com/questions/29193303/6502-emulation-proper-way-to-implement-adc-and-sbc
// https://github.com/mre/mos6502.git
// https://github.com/mattgodbolt/jsbeeb
// http://www.visual6502.org/JSSim/expert.html?graphics=false&a=0&d=a900f8e988eaeaea&steps=18
// http://vice-emu.sourceforge.net/plain/64doc.txt
// https://github.com/mattgodbolt/jsbeeb/blob/main/src/6502.js
pub fn sbc(cpu: &mut Cpu, operand: u8) {
    if p_get!(cpu.reg, D) {
        let carry = i32::from(!p_get!(cpu.reg, C));

        let a = i32::from(cpu.reg.a);
        let value = i32::from(operand);

        let mut al = (a & 0xf) - (value & 0xf) - carry;
        let mut ah = (a >> 4) - (value >> 4);
        if (al & 0x10) != 0 {
            al = (al - 6) & 0xf;
            ah -= 1;
        }
        if (ah & 0x10) != 0 {
            ah = (ah - 6) & 0xf;
        }

        let result = a - value - carry;
        p_set!(cpu.reg, N, (result & 0x80) != 0);
        p_set!(cpu.reg, Z, result.trailing_zeros() >= 8);
        p_set!(cpu.reg, V, ((a ^ result) & (value ^ a) & 0x80) != 0);
        p_set!(cpu.reg, C, (result & 0x100) == 0);
        cpu.reg.a = u8::truncate(al) | (u8::truncate(ah) << 4);
    } else {
        adc(cpu, !operand);
    }
}

#[cfg(test)]
mod tests {
    use crate::emulator::ops::arithmetic::{adc, sbc};
    use crate::emulator::{Bus, Cpu, InterruptChannel, P};
    use crate::{_p, p};
    use rstest::rstest;

    #[rstest]
    // LDA #1; PHP; LDA #255; PLP; CLC; ADC #0
    #[case(0xff, 0x0000, p!(N), 0xff, 0x0000, p!(), 0x00)]
    // LDA #1; PHP; LDA #255; PLP; CLC; ADC #1
    #[case(0x00, 0x0000, p!(Z, C), 0xff, 0x0000, p!(), 0x01)]
    // LDA #1; PHP; LDA #255; PLP; SEC; ADC #0
    #[case(0x00, 0x0000, p!(Z, C), 0xff, 0x0000, p!(C), 0x00)]
    // LDA #1; PHP; LDA #255; PLP; SEC; ADC #1
    #[case(0x01, 0x0000, p!(C), 0xff, 0x0000, p!(C), 0x01)]
    // LDA #1; PHP; LDA #$12; PLP; CLC; ADC #$34
    #[case(0x46, 0x0000, p!(), 0x12, 0x0000, p!(), 0x34)]
    fn adc_basics(
        #[case] expected_a: u8,
        #[case] expected_pc: u16,
        #[case] expected_p: P,
        #[case] a: u8,
        #[case] pc: u16,
        #[case] p: P,
        #[case] operand: u8,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = a;
        cpu.reg.pc = pc;
        cpu.reg.p = p;
        adc(&mut cpu, operand);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(expected_pc, cpu.reg.pc);
        assert_eq!(expected_p, cpu.reg.p);
    }

    #[rstest]
    // cargo run -- validate-json '{ "name": "61 61 a2", "initial": { "pc": 31864, "s": 184, "a": 245, "x": 248, "y": 67, "p": 44, "ram": [ [31864, 97], [31865, 97], [31866, 162], [97, 187], [89, 99], [90, 228], [58467, 109]]}, "final": { "pc": 31866, "s": 184, "a": 200, "x": 248, "y": 67, "p": 45, "ram": [ [89, 99], [90, 228], [97, 187], [31864, 97], [31865, 97], [31866, 162], [58467, 109]]}, "cycles": [ [31864, 97, "read"], [31865, 97, "read"], [97, 187, "read"], [89, 99, "read"], [90, 228, "read"], [58467, 109, "read"]] }'
    #[case(45, 200, 44, 245, 109)]
    // cargo run -- validate-json '{ "name": "61 0f 6b", "initial": { "pc": 28568, "s": 191, "a": 136, "x": 3, "y": 89, "p": 175, "ram": [ [28568, 97], [28569, 15], [28570, 107], [15, 179], [18, 143], [19, 69], [17807, 106]]}, "final": { "pc": 28570, "s": 191, "a": 89, "x": 3, "y": 89, "p": 173, "ram": [ [15, 179], [18, 143], [19, 69], [17807, 106], [28568, 97], [28569, 15], [28570, 107]]}, "cycles": [ [28568, 97, "read"], [28569, 15, "read"], [15, 179, "read"], [18, 143, "read"], [19, 69, "read"], [17807, 106, "read"]] }'
    #[case(173, 89, 175, 136, 106)]
    // cargo run -- validate-json '{ "name": "61 7b 59", "initial": { "pc": 12139, "s": 205, "a": 80, "x": 208, "y": 251, "p": 235, "ram": [ [12139, 97], [12140, 123], [12141, 89], [123, 219], [75, 208], [76, 222], [57040, 4]]}, "final": { "pc": 12141, "s": 205, "a": 85, "x": 208, "y": 251, "p": 40, "ram": [ [75, 208], [76, 222], [123, 219], [12139, 97], [12140, 123], [12141, 89], [57040, 4]]}, "cycles": [ [12139, 97, "read"], [12140, 123, "read"], [123, 219, "read"], [75, 208, "read"], [76, 222, "read"], [57040, 4, "read"]] }'
    #[case(40, 85, 235, 80, 4)]
    // cargo run -- validate-json '{ "name": "61 14 ee", "initial": { "pc": 60514, "s": 195, "a": 123, "x": 182, "y": 114, "p": 170, "ram": [ [60514, 97], [60515, 20], [60516, 238], [20, 30], [202, 1], [203, 232], [59393, 82]]}, "final": { "pc": 60516, "s": 195, "a": 51, "x": 182, "y": 114, "p": 233, "ram": [ [20, 30], [202, 1], [203, 232], [59393, 82], [60514, 97], [60515, 20], [60516, 238]]}, "cycles": [ [60514, 97, "read"], [60515, 20, "read"], [20, 30, "read"], [202, 1, "read"], [203, 232, "read"], [59393, 82, "read"]] }'
    #[case(233, 51, 170, 123, 82)]
    // cargo run -- validate-json '{ "name": "61 1e 49", "initial": { "pc": 26086, "s": 108, "a": 250, "x": 117, "y": 104, "p": 173, "ram": [ [26086, 97], [26087, 30], [26088, 73], [30, 225], [147, 188], [148, 211], [54204, 79]]}, "final": { "pc": 26088, "s": 108, "a": 160, "x": 117, "y": 104, "p": 45, "ram": [ [30, 225], [147, 188], [148, 211], [26086, 97], [26087, 30], [26088, 73], [54204, 79]]}, "cycles": [ [26086, 97, "read"], [26087, 30, "read"], [30, 225, "read"], [147, 188, "read"], [148, 211, "read"], [54204, 79, "read"]] }'
    #[case(45, 160, 173, 250, 79)]
    // cargo run -- validate-json '{ "name": "61 8b 47", "initial": { "pc": 8970, "s": 138, "a": 190, "x": 116, "y": 121, "p": 169, "ram": [ [8970, 97], [8971, 139], [8972, 71], [139, 215], [255, 241], [0, 87], [22513, 19]]}, "final": { "pc": 8972, "s": 138, "a": 56, "x": 116, "y": 121, "p": 169, "ram": [ [0, 87], [139, 215], [255, 241], [8970, 97], [8971, 139], [8972, 71], [22513, 19]]}, "cycles": [ [8970, 97, "read"], [8971, 139, "read"], [139, 215, "read"], [255, 241, "read"], [0, 87, "read"], [22513, 19, "read"]] }'
    #[case(169, 56, 169, 190, 19)]
    fn adc_decimal_basics(
        #[case] expected_p: u8,
        #[case] expected_a: u8,
        #[case] p: u8,
        #[case] a: u8,
        #[case] value: u8,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = a;
        cpu.reg.p = _p!(p);
        adc(&mut cpu, value);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(_p!(expected_p), cpu.reg.p);
    }

    #[rstest]
    #[case(0xfe, 0x0000, p!(N, C), 0xff, 0x0000, p!(C), 0x01)]
    fn sbc_basics(
        #[case] expected_a: u8,
        #[case] expected_pc: u16,
        #[case] expected_p: P,
        #[case] a: u8,
        #[case] pc: u16,
        #[case] p: P,
        #[case] operand: u8,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = a;
        cpu.reg.pc = pc;
        cpu.reg.p = p;
        sbc(&mut cpu, operand);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(expected_pc, cpu.reg.pc);
        assert_eq!(expected_p, cpu.reg.p);
    }

    #[rstest]
    #[case(0x00, 0b1010_1001, 0xe3, 0b0010_1000, 0xb7)]
    fn adc_scenarios(
        #[case] expected_a: u8,
        #[case] expected_p: u8,
        #[case] a: u8,
        #[case] p: u8,
        #[case] operand: u8,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = a;
        cpu.reg.p = _p!(p);
        adc(&mut cpu, operand);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(_p!(expected_p), cpu.reg.p);
    }

    #[rstest]
    #[case(0x78, 0b1010_1100, 0x9c, 0b0110_1101, 0xc4)]
    #[case(0x2d, 0b1110_1000, 0x50, 0b1110_1010, 0xcc)]
    fn sbc_scenarios(
        #[case] expected_a: u8,
        #[case] expected_p: u8,
        #[case] a: u8,
        #[case] p: u8,
        #[case] operand: u8,
    ) {
        let bus = Bus::default();
        let interrupt_channel = InterruptChannel::new();
        let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
        cpu.reg.a = a;
        cpu.reg.p = _p!(p);
        sbc(&mut cpu, operand);
        assert_eq!(expected_a, cpu.reg.a);
        assert_eq!(_p!(expected_p), cpu.reg.p);
    }
}
