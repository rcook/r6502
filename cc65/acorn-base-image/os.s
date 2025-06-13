.export OSHALT
.export OSIRQ
.export OSWRCH

.segment "OSHALT"
OSHALT:
    brk
    nop
    rts

.segment "OSIRQ"
OSIRQ:
    brk
    nop

.segment "OSWRCH"
OSWRCH:
    brk
    nop
    rts
