.export NMI
.export RESET
.export IRQ

.segment "NMI"
NMI:
.word $0000

.segment "RESET"
RESET:
.word STARTUP

.segment "IRQ"
IRQ:
.word $0000
