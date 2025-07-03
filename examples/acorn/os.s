; 6502 vectors
.export NMI = $FFFA
.export RESET = $FFFC
.export IRQ = $FFFE

; Debugging hooks
.export HALT = $FFA0

; Acorn MOS operating system routines
.export OSCLI = $FFF7
.export OSBYTE = $FFF4
.export OSWORD = $FFF1
.export OSWRCH = $FFEE
.export OSNEWL = $FFE7
.export OSASCI = $FFE3
.export OSRDCH = $FFE0
.export OSFILE = $FFDD
.export OSARGS = $FFDA
.export OSBGET = $FFD7
.export OSBPUT = $FFD4
.export OSGBPB = $FFD1
.export OSFIND = $FFCE
