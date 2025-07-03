.macpack r6502
.import HALT
.import copydata
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr main
    jmp HALT
.endproc
