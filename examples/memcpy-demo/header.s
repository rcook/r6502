.macpack r6502
.import HALT
.import copydata
.import main
.import __RODATA_LOAD__

r6502_header "ACRN", __RODATA_LOAD__, startup

.code
.proc startup
    jsr copydata
    jsr main
    jmp HALT
.endproc
