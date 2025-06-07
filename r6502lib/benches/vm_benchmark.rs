use criterion::{criterion_group, criterion_main, Criterion};
use r6502lib::{DummyMonitor, Image, Memory, Reg, Vm, VmState, _p};

// div16 takes approx. 938 cycles
// On a real 6502 at 1 MHz this ought to run in around 1 ms.
fn div16_benchmark(c: &mut Criterion) {
    let memory = Memory::default();
    let mut vm = Vm::new(
        Box::new(DummyMonitor),
        VmState::new(Reg::default(), memory.view()),
    );

    let bytes = include_bytes!("../../examples/div16.bin");
    let image = Image::from_bytes(bytes, None, None, None).expect("Must succeed");
    memory.store_image(&image).expect("Must succeed");

    assert_eq!(0x35, memory.load(0x106c));
    assert_eq!(0x12, memory.load(0x106d));

    c.bench_function("vm.step", |b| {
        b.iter(|| {
            // Reset state so that full 938 cycles is executed in every iteration
            memory.store(0x106c, 0x35);
            memory.store(0x106d, 0x12);
            vm.s.reg.p = _p!(0b00000000);
            vm.s.reg.pc = image.start;
            vm.s.reg.sp = image.sp;
            let before_total_cycles = vm.total_cycles;
            while vm.step() {}
            let after_total_cycles = vm.total_cycles;
            assert_eq!(0, vm.s.reg.a);
            assert_eq!(938, after_total_cycles - before_total_cycles);
        })
    });
}

criterion_group!(benches, div16_benchmark);
criterion_main!(benches);
