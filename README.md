# r6502

MOS 6502 Emulator and Debugger

This is my emulator and debugger for the 6502 CPU. It is _not_ cycle-accurate in that
it does not fully emulate the fetch-decode-execute steps of a real 6502. This leads to
a couple of fun little hacks ([example](r6502lib/src/ops/jump.rs#L21)). It also does
not emulate the 6502's timing and currently runs "at full speed" - i.e. it will run
instructions as quickly as the host operating system handle it. I have, however,
verified the (reasonably) correct functioning of the all implemented opcodes using
the [SingleStepTests/65x02][single-step-tests] test suite. It's also able to run
[wozmon](cc65/apple1/README.md). I plan to get it to run Integer BASIC and
Microsoft Basic eventually.

# r6502

This is the emulator and debugger.

# r6502validation

This runs the 6502 validation suite.

# Licence

[MIT License](LICENSE)

# Resources

* https://logicalmoon.com/2017/11/using-vs-code-to-create-a-6502-hello-world/
* https://github.com/stardot/beebasm
* http://www.6502.org/tutorials/6502opcodes.html
* https://superuser.com/questions/346658/does-the-6502-put-ff-in-the-stack-pointer-register-as-soon-as-it-gets-power-for
* https://stackoverflow.com/questions/49078096/im-failing-to-understand-how-the-stack-works
* https://www.pagetable.com/?p=410
* https://c64os.com/post/6502instructions#RTI
* https://cafbit.com/post/cursive_writing_terminal_applications_in_rust/
* https://stackoverflow.com/questions/78122826/how-to-create-a-new-dialog-after-cursive-run-has-been-called
* https://web.archive.org/web/20211204234443if_/http://archive.6502.org/datasheets/mos_6501-6505_mpu_preliminary_aug_1975.pdf
* https://www.masswerk.at/6502/6502_instruction_set.html
* https://github.com/Klaus2m5/6502_65C02_functional_tests
* https://github.com/C-Chads/MyLittle6502
* https://github.com/andymccall/neo6502-development
* https://github.com/PeteGollan/Neo6502-programs/tree/main/HelloNeo6502-CC65
* https://github.com/paulscottrobson/neo6502-firmware/releases
* https://www.steckschwein.de/post/wozmon-a-memory-monitor-in-256-bytes/
* https://www.sbprojects.net/projects/apple1/wozmon.php
* https://www.applefritter.com/replica/chapter7
* http://www.brouhaha.com/~eric/retrocomputing/apple/apple1/basic/
* https://jefftranter.blogspot.com/2012/05/source-code-for-apple-1-basic.html

[single-step-tests]: https://github.com/SingleStepTests/65x02
