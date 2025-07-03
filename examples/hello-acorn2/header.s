.macpack r6502
.import OSEXIT ; TBD: Rename to OSHALT for consistency
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACN2", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.proc startup
    jsr main
    jmp OSEXIT
.endproc
