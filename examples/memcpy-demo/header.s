.macpack r6502
.import HALT
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.proc startup
    jsr main
    jmp HALT
.endproc
