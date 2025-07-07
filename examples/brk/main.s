.macpack r6502
.macpack util
.import print
.import __SIDEWAYSHEADER_LOAD__
.exportzp zword0

r6502_system "ACRN", __SIDEWAYSHEADER_LOAD__
sideways_rom_header entrypoint, , , , "brk", "1.0", "2025 Richard Cook"

.zeropage
zword0: .word $0000

.segment "SIDEWAYSCODE"
.proc entrypoint
    print_buf message
    brk
.endproc

.segment "SIDEWAYSDATA"
message: .byte "One break coming up...", 13, 10, 0
