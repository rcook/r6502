.macpack r6502
.import HALT
.import main
.import __ROCODE_LOAD__

r6502_header "ACRN", __ROCODE_LOAD__, startup

.segment "ROCODE"
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr main
    jmp HALT
.endproc
