#[derive(Clone, Debug)]
pub(crate) struct RegisterFile {
    pub(crate) p: u8,
    pub(crate) pc: u16,
    pub(crate) a: u8,
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) s: u8,
}

impl RegisterFile {
    pub(crate) fn new() -> Self {
        Self {
            pc: 0x0000u16,
            p: 0x00u8,
            a: 0x00u8,
            x: 0x00u8,
            y: 0x00u8,
            s: 0xffu8,
        }
    }

    pub(crate) fn pretty(&self) -> String {
        format!(
            "pc={:04X} NV1BDIZC={:08b} a={:02X} x={:02X} y={:02X} s={:02X}",
            self.pc, self.p, self.a, self.x, self.y, self.s,
        )
    }
}
