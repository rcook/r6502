.macpack r6502
.import test_add16
.import test_div16
.import test_num_to_str
.import test_preserve_stack
.import test_str_to_num
.import copydata
.import __SIDEWAYSHEADER_LOAD__
.exportzp zword0

r6502_system "ACRN", __SIDEWAYSHEADER_LOAD__
sideways_rom_header entrypoint, , , , "tests", "1.0", "2025 Richard Cook"

.segment "SIDEWAYSHEADER"
.proc entrypoint
    jsr copydata

    jsr test_add16
    bne @failed
    jsr test_div16
    bne @failed
    jsr test_num_to_str
    bne @failed
    jsr test_str_to_num
    bne @failed
    jsr test_preserve_stack
    bne @failed

    syshalt $00
@failed:
    syshalt $01
.endproc

.zeropage
zword0: .word $0000
