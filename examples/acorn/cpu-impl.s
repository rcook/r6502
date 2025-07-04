.segment "NMI"
    .addr $0000

.segment "RESET"
    .addr $0000

.segment "IRQ"
    .addr $0000

.segment "HALT"
.proc HALT
    brk
    nop
    rts
.endproc
