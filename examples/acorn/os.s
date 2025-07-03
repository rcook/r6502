.import startup

KBD = $FC00
KBDCR = $FC01
DSP = $FC02
DSPCR = $FC03

.segment "NMI"
.export NMI
.proc NMI
.addr $0000
.endproc

.segment "RESET"
.export RESET
.proc RESET
.addr startup
.endproc

.segment "IRQ"
.export IRQ
.proc IRQ
.addr OSIRQ
.endproc

.segment "HALT"
.export HALT
.proc HALT
    brk
    nop
    rts
.endproc

.segment "OSIRQ"
.export OSIRQ
.proc OSIRQ
    brk
    nop
.endproc

.segment "OSWRCH"
.export OSWRCH
.proc OSWRCH
    jmp oswrch_impl
.endproc

.code
.proc oswrch_impl
    bit DSP
    bmi oswrch_impl
    sta DSP
    rts
.endproc
