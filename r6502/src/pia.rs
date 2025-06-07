use r6502lib::MemoryView;

// Apple I PIA addresses etc.
#[allow(unused)]
pub(crate) const KBD: u16 = 0xD010; // PIA.A keyboard input
#[allow(unused)]
pub(crate) const KBDCR: u16 = 0xD011; // PIA.A keyboard control register
pub(crate) const DSP: u16 = 0xD012; // PIA.B display output register
pub(crate) const DSPCR: u16 = 0xD013; //  PIA.B display control register

pub(crate) fn run_pia(memory: MemoryView) {
    loop {
        let dspcr = memory.load(DSPCR);
        if dspcr == 0x01 {
            memory.store(DSPCR, 0x00);
            let value = memory.load(DSP);
            println!("{value}")
        }
    }
}
