# r6502

MOS 6502 emulator and debugger

Emulator verified using [SingleStepTests/65x02][single-step-tests] test
suite

# r6502

This is the emulator and debugger.

# r6502validation

This runs the 6502 validation suite.

# r6502asm

This is a wrapper script to generate binary files in the format expected
by r6502.

```bash
python -m pip install --upgrade pip
pip install -e r6502asm
```

# Buggy opcodes

```
$20 1
$26 1
$2E 1
$30 1
$3D 1
$3E 1
$56 1
$5D 1
$66 1
$8C 1
$8D 1
$98 1
$A1 1
$A2 1
$AA 1
$AC 1
$B5 1
$BC 1
$C6 1
$D0 1
$D8 1
$FE 1
```

https://logicalmoon.com/2017/11/using-vs-code-to-create-a-6502-hello-world/
https://github.com/stardot/beebasm
http://www.6502.org/tutorials/6502opcodes.html
https://superuser.com/questions/346658/does-the-6502-put-ff-in-the-stack-pointer-register-as-soon-as-it-gets-power-for
https://stackoverflow.com/questions/49078096/im-failing-to-understand-how-the-stack-works
https://www.pagetable.com/?p=410
https://c64os.com/post/6502instructions#RTI
https://cafbit.com/post/cursive_writing_terminal_applications_in_rust/
https://stackoverflow.com/questions/78122826/how-to-create-a-new-dialog-after-cursive-run-has-been-called
https://web.archive.org/web/20211204234443if_/http://archive.6502.org/datasheets/mos_6501-6505_mpu_preliminary_aug_1975.pdf
https://www.masswerk.at/6502/6502_instruction_set.html
https://github.com/Klaus2m5/6502_65C02_functional_tests


[single-step-tests]: https://github.com/SingleStepTests/65x02