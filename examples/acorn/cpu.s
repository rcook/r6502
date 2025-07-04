.export STACKBASE = $0100

; MOS initialization routine
.export MOSINIT = $D000

; 6502 vectors
.export NMI = $FFFA
.export RESET = $FFFC
.export IRQ = $FFFE

; Debugging hooks
.export HALT = $FFA0
