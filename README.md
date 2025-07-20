# r6502

MOS 6502 Emulator and Debugger

This is my emulator and debugger for the 6502 CPU. It is _not_
cycle-accurate in that it does not fully emulate the
fetch-decode-execute steps of a real 6502. This leads to a couple of fun
little hacks ([example](r6502core/src/ops/jump.rs#L21)). It also does not
emulate the 6502's timing and currently runs "at full speed" - i.e. it
will run instructions as quickly as the host operating system handle it.
I have, however, verified the (reasonably) correct functioning of the
all implemented opcodes using the [SingleStepTests/65x02][single-step-tests]
test suite. It's also able to run [wozmon](cc65/apple1/README.md) and
Apple 1 Integer BASIC. I plan to get it to run Microsoft BASIC
eventually.

Note that r6502 does not currently implement any undocumented 6502
opcodes and will panic if any are encountered.

## Dev setup

### Ubuntu

Install prerequisites:

```bash
sudo apt install freeglut3-dev libfreetype-dev libglu1-mesa-dev mesa-common-dev mesa-utils
export LD_LIBRARY_PATH=/usr/local/lib
#export LIBGL_ALWAYS_SOFTWARE=1
```

* [Download SDL3-3.2.18.tar.gz][sdl3-devel]
* Build with CMake: `cmake . && sudo make install`
* [Download SDL3_ttf-3.2.2.tar.gz][sdl3-ttf-devel]
* Build with CMake: `sudo apt install  && cmake . && sudo make install`

### macOS

* `brew install sdl3 sdl3_ttf`

### Windows

* [Download SDL3-devel-3.2.18-VC.zip][sdl3-devel]
* [Download SDL3_ttf-devel-3.2.2-VC.zip][sdl3-ttf-devel]
* Extract the contents of the `lib` directories into the root of this project

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
* https://electronics.stackexchange.com/questions/678427/mc6820-pia-operation-on-the-apple-1

[sdl3-devel]: https://github.com/libsdl-org/SDL/releases/tag/release-3.2.18
[sdl3-ttf-devel]: https://github.com/libsdl-org/SDL_ttf/releases/tag/release-3.2.2
[single-step-tests]: https://github.com/SingleStepTests/65x02
