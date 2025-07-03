.macpack r6502
.import OSHALT
.import main
.import __ROCODE_LOAD__

r6502_header "ACN2", __ROCODE_LOAD__, startup

.segment "ROCODE"
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr main
    jmp OSHALT
.endproc
