.macpack r6502
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.proc startup
    sysinit
    jsr main
    syshalt
.endproc
