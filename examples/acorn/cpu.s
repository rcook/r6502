.export STACKBASE = $0100

; MOS initialization routine
.export MOSINIT = $D000

; 6502 vectors
.export NMI = $FFFA
.export RESET = $FFFC
.export IRQ = $FFFE

; Debugging and host communication hooks
.export HALT = $FFA0
.export HOSTHOOK = $FFA2
.exportzp CLIVHOSTHOOK = 100
.exportzp FILEVHOSTHOOK = 101
