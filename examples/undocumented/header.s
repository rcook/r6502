.macpack r6502
.import copydata
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

HALT = $FFC0

.segment "SIDEWAYSCODE"
.proc startup
    jsr copydata
    jsr main
    jmp HALT
.endproc
