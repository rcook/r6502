.macpack r6502
.import HALT
.import main
.import copydata
.import __DATA_LOAD__

r6502_header "ACRN", __DATA_LOAD__, startup

.code
.proc startup
    jsr copydata
    jsr main
    jmp HALT
.endproc
