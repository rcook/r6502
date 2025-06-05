use anyhow::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use r6502lib::{Image, Vm, _p};

// div16 takes approx. 938 cycles
// On a real 6502 at 1 MHz this ought to run in around 1 ms.
fn div16_benchmark(c: &mut Criterion) {
    fn load_into_vm(bytes: &[u8]) -> Result<(Vm, Image)> {
        let image = Image::from_bytes(bytes, None, None, None)?;
        let mut vm = Vm::default();
        vm.s.memory.load(&image);
        Ok((vm, image))
    }

    let bytes = include_bytes!("../../examples/div16.bin");
    let (mut vm, image) = load_into_vm(bytes).expect("Must succeed");
    assert_eq!(0x35, vm.s.memory[0x106c]);
    assert_eq!(0x12, vm.s.memory[0x106d]);

    c.bench_function("vm.step", |b| {
        b.iter(|| {
            // Reset state so that full 938 cycles is executed in every iteration
            vm.s.memory[0x106c] = 0x35;
            vm.s.memory[0x106d] = 0x12;
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
