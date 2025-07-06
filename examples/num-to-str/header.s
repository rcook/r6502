.macpack r6502
.import copydata
.import main
.import __SIDEWAYSCODE_LOAD__

r6502_module "ACRN", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    sysinit
    jsr copydata
    jsr main
    syshalt
.endproc
