.macpack r6502
.macpack util
;.import copydata
.import print
.import __SIDEWAYSCODE_LOAD__
.exportzp zword0

r6502_system "ACRN", __SIDEWAYSCODE_LOAD__

.zeropage
zword0: .word $0000

.segment "SIDEWAYSCODE"
.proc entrypoint
    sideways_rom_header @go, , , , "brk", "1.0", "2025 Richard Cook"
@go:
    print_buf message
    brk
.endproc

.segment "SIDEWAYSDATA"
message: .byte "One break coming up...", 13, 10, 0
