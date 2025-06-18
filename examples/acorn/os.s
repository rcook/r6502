KBD = $FC00
KBDCR = $FC01
DSP = $FC02
DSPCR = $FC03

.segment "NMI"
.export NMI
NMI:
.addr $0000

.segment "RESET"
.export RESET
RESET:
.addr STARTUP

.segment "IRQ"
.export IRQ
IRQ:
.addr OSIRQ

.segment "OSHALT"
.export OSHALT
OSHALT:
    brk
    nop
    rts

.segment "OSIRQ"
.export OSIRQ
OSIRQ:
    brk
    nop

.segment "OSWRCH"
.export OSWRCH
OSWRCH:
    jmp oswrch_impl

.code
oswrch_impl:
    bit DSP
    bmi oswrch_impl
    sta DSP
    rts
