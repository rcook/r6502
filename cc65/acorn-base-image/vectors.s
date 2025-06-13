.export NMI
.export RESET
.export IRQ

.segment "NMI"
NMI:
.addr $0000

.segment "RESET"
RESET:
.addr STARTUP

.segment "IRQ"
IRQ:
.addr OSIRQ
