; https://github.com/chelsea6502/BeebEater
; https://hackaday.io/project/177384-w65c816sxb-investigation/log/189547-porting-bbc-basic
; https://mdfs.net/Software/BBCBasic/Porting/Porting.htm
;
; The bare minimum your MOS needs to do is:
; BRKV, WRCHV:    vectors for BRK and WRCH
; BRK:            point &FD/E to error message, jump via BRKV
; OSWRCH/WRCHV:   print characters
; OSWORD 0:       read input line
; OSBYTE &83/&84: read bottom of memory/top of memory
; &0000-&005F:    zero page workspace for BASIC
; &0400-&07FF:    fixed workspace for BASIC
; Enter BASIC at its entry point with A=1

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
.addr $0000
.endproc

; TBD
.segment "OSCLI"

.segment "OSBYTE"
.export OSBYTE
.proc OSBYTE
    jmp osbyte_impl
.endproc

.segment "ROCODE"
.proc osbyte_impl
    rts
.endproc

.segment "OSWORD"
.export OSWORD
.proc OSWORD
    jmp osword_impl
.endproc

.segment "ROCODE"
.proc osword_impl
    rts
.endproc

.segment "OSWRCH"
.export OSWRCH
.proc OSWRCH
    jmp oswrch_impl
.endproc

.segment "ROCODE"
.proc oswrch_impl
    bit DSP
    bmi oswrch_impl
    sta DSP
    rts
.endproc

; TBD
.segment "OSNEWL"

; TBD
.segment "OSASCI"

; TBD
.segment "OSRDCH"

; TBD
.segment "OSFILE"

; TBD
.segment "OSARGS"

; TBD
.segment "OSBGET"

; TBD
.segment "OSBPUT"

; TBD
.segment "OSGBPB"

; TBD
.segment "OSFIND"

.segment "HALT"
.export HALT
.proc HALT
    brk
    nop
    rts
.endproc
