.macpack r6502
.import __RODATA_LOAD__

r6502_header "ACRN", __RODATA_LOAD__, startup

.code
.proc startup
    jsr copydata
    jsr MAIN
    jmp OSEXIT
.endproc
