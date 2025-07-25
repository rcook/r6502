use criterion::{Criterion, criterion_group, criterion_main};
use r6502core::emulator::{Bus, Cpu};
use r6502core::{_p, InterruptChannel, p_get};
use r6502lib::RESET;
use r6502lib::constants::{DEFAULT_SP, IRQ};
use r6502lib::util::{make_word, split_word};
use r6502snapshot::MemoryImage;

// div16 takes approx. 938 cycles
// On a real 6502 at 1 MHz this ought to run in around 1 ms.
fn div16_benchmark(c: &mut Criterion) {
    const IRQ_ADDR: u16 = 0x2000;

    let bytes = [
        0x73, 0x69, 0x6d, 0x36, 0x35, 0x02, 0x00, 0xff, 0x00, 0x10, 0x00, 0x10, 0x20, 0x38, 0x10,
        0xad, 0x6c, 0x10, 0xc9, 0xd2, 0xf0, 0x05, 0xa9, 0x01, 0x4c, 0xf9, 0xff, 0xad, 0x6d, 0x10,
        0xc9, 0x01, 0xf0, 0x05, 0xa9, 0x02, 0x4c, 0xf9, 0xff, 0xad, 0x70, 0x10, 0xc9, 0x01, 0xf0,
        0x05, 0xa9, 0x03, 0x4c, 0xf9, 0xff, 0xad, 0x71, 0x10, 0xc9, 0x00, 0xf0, 0x05, 0xa9, 0x04,
        0x4c, 0xf9, 0xff, 0xa9, 0x00, 0x4c, 0xf9, 0xff, 0xa9, 0x00, 0x8d, 0x70, 0x10, 0x8d, 0x71,
        0x10, 0xa2, 0x10, 0x0e, 0x6c, 0x10, 0x2e, 0x6d, 0x10, 0x2e, 0x70, 0x10, 0x2e, 0x71, 0x10,
        0xad, 0x70, 0x10, 0x38, 0xed, 0x6e, 0x10, 0xa8, 0xad, 0x71, 0x10, 0xed, 0x6f, 0x10, 0x90,
        0x09, 0x8d, 0x71, 0x10, 0x8c, 0x70, 0x10, 0xee, 0x6c, 0x10, 0xca, 0xd0, 0xd8, 0x60, 0x19,
        0x35, 0x12, 0x0a, 0x00, 0xff, 0xff,
    ];
    let image = MemoryImage::from_bytes(&bytes).expect("Must succeed");

    let bus = Bus::default_with_image(&image).expect("Must succeed");
    let interrupt_channel = InterruptChannel::new();
    let mut cpu = Cpu::new(bus.view(), None, interrupt_channel.rx);
    let reset_addr_lo = cpu.bus.load(RESET);
    let reset_addr_hi = cpu.bus.load(RESET.wrapping_add(1));
    let reset_addr = make_word(reset_addr_hi, reset_addr_lo);
    let cpu_state = image.get_initial_cpu_state(reset_addr);
    cpu.set_initial_state(&cpu_state);

    cpu.bus.store(IRQ_ADDR, 0x40);
    let (hi, lo) = split_word(IRQ_ADDR);
    bus.store(IRQ, lo);
    bus.store(IRQ.wrapping_add(1), hi);

    assert_eq!(0x35, bus.load(0x106c));
    assert_eq!(0x12, bus.load(0x106d));

    c.bench_function("cpu.step", |b| {
        b.iter(|| {
            // Reset state so that full 938 cycles is executed in every iteration
            bus.store(0x106c, 0x35);
            bus.store(0x106d, 0x12);
            // image.set_initial_cpu_state(&mut cpu);
            cpu.reg.p = _p!(0b0000_0000);
            cpu.reg.pc = image.start().unwrap_or_default();
            cpu.reg.sp = image.sp().unwrap_or(DEFAULT_SP);
            let before_total_cycles = cpu.total_cycles;
            loop {
                cpu.step();
                if p_get!(cpu.reg, I) {
                    break;
                }
            }
            let after_total_cycles = cpu.total_cycles;
            assert_eq!(0, cpu.reg.a);
            assert_eq!(938, after_total_cycles - before_total_cycles);
        });
    });
}

criterion_group!(benches, div16_benchmark);
criterion_main!(benches);
