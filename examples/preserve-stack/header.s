.macpack r6502
.import __DATA_LOAD__

r6502_header "ACRN", __DATA_LOAD__, startup

.code
.proc startup
    jsr copydata
    jsr MAIN
    jmp OSEXIT
.endproc
