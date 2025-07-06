; https://github.com/chelsea6502/BeebEater
; https://hackaday.io/project/177384-w65c816sxb-investigation/log/189547-porting-bbc-basic
; https://mdfs.net/Software/BBCBasic/Porting/Porting.htm
;
; The bare minimum your MOS needs to do is:
; BRKV, WRCHV:    vectors for BRK and WRCH [IMPLEMENTED]
; BRK:            point &FD/E to error message, jump via BRKV [IMPLEMENTED]
; OSWRCH/WRCHV:   print characters [IMPLEMENTED]
; OSWORD 0:       read input line [IMPLEMENTED]
; OSBYTE &83/&84: read bottom of memory/top of memory [IMPLEMENTED]
; &0000-&005F:    zero page workspace for BASIC [IMPLEMENTED]
; &0400-&07FF:    fixed workspace for BASIC [IMPLEMENTED]
; Enter BASIC at its entry point with A=1 [IMPLEMENTED]

.macpack r6502
.import __MOSINIT_LOAD__

r6502_system "ACRN", __MOSINIT_LOAD__
