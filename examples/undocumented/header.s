.macpack r6502
.import copydata
.import main
.import __DATA_LOAD__

r6502_header "ACRN", __DATA_LOAD__, startup

HALT = $FFC0

.code
.proc startup
    jsr copydata
    jsr main
    jmp HALT
.endproc
