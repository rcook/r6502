use r6502lib::MemoryView;

const CONTROL_ADDR: u16 = 0xd001;
const VALUE_ADDR: u16 = 0xd002;

pub(crate) fn run_pia(memory_view: &mut MemoryView) {
    loop {
        let control = memory_view.load(CONTROL_ADDR);
        if control == 0x01 {
            memory_view.store(CONTROL_ADDR, 0x00);
            let value = memory_view.load(VALUE_ADDR);
            println!("{value}")
        }
    }
}
