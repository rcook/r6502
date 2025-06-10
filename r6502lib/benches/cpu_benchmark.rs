use criterion::{criterion_group, criterion_main, Criterion};
use r6502lib::{Bus, Cpu, Image, _p};

// div16 takes approx. 938 cycles
// On a real 6502 at 1 MHz this ought to run in around 1 ms.
fn div16_benchmark(c: &mut Criterion) {
    let bus = Bus::default();
    let mut cpu = Cpu::new(bus.view(), None);

    let bytes = include_bytes!("../../examples/div16.bin");
    let image = Image::from_bytes(bytes, None, None, None).expect("Must succeed");
    bus.store_image(&image).expect("Must succeed");

    assert_eq!(0x35, bus.load(0x106c));
    assert_eq!(0x12, bus.load(0x106d));

    c.bench_function("cpu.step", |b| {
        b.iter(|| {
            // Reset state so that full 938 cycles is executed in every iteration
            bus.store(0x106c, 0x35);
            bus.store(0x106d, 0x12);
            cpu.reg.p = _p!(0b00000000);
            cpu.reg.pc = image.start;
            cpu.reg.sp = image.sp;
            let before_total_cycles = cpu.total_cycles;
            while cpu.step() {}
            let after_total_cycles = cpu.total_cycles;
            assert_eq!(0, cpu.reg.a);
            assert_eq!(938, after_total_cycles - before_total_cycles);
        })
    });
}

criterion_group!(benches, div16_benchmark);
criterion_main!(benches);
