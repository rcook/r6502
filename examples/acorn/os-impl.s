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

.zeropage
ztempword0: .word $0000
ztempword1: .word $0000

LF = 10
CR = 13
KBD = $FC00
KBDCR = $FC01
DSP = $FC02
DSPCR = $FC03
OSHWM = $0E00
HIMEM = $8000

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
    php
    cmp #$83
    beq @osbyte_131_oshwm
    cmp #$84
    beq @osbyte_132_himem
    plp
    rts
@osbyte_131_oshwm:
    ldx #<OSHWM
    ldy #>OSHWM
    plp
    rts
@osbyte_132_himem:
    ldx #<HIMEM
    ldy #>HIMEM
    plp
    rts
.endproc

.segment "OSWORD"
.export OSWORD
.proc OSWORD
    jmp osword_impl
.endproc

.segment "ROCODE"
.proc osword_impl
    php
    cmp #$00
    beq @osword_read_line
    plp
    rts
@osword_read_line:
    txa
    pha
    tya
    pha

    stx ztempword0      ; LSB of parameter block address
    sty ztempword0 + 1  ; MSB of parameter block address

    ldy #$00
    lda (ztempword0), y
    sta ztempword1      ; LSB of buffer
    iny
    lda (ztempword0), y
    sta ztempword1 + 1  ; MSB of buffer

    ldy #$00
    lda #'A'
    sta (ztempword1), y

    iny
    lda #'B'
    sta (ztempword1), y

    pla
    tay
    pla
    tax
    plp
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

.segment "OSNEWL"
.export OSNEWL
.proc OSNEWL
    jmp osnewl_impl
.endproc

.segment "ROCODE"
.proc osnewl_impl
    php
    pha
    lda #LF
    jsr OSWRCH
    lda #CR
    jsr OSWRCH
    pla
    plp
    rts
.endproc

.segment "OSASCI"
.export OSASCI
.proc OSASCI
    jmp osasci_impl
.endproc

.segment "ROCODE"
.proc osasci_impl
    php
    pha
    cmp #CR
    beq @line_break
    jsr OSWRCH
    pla
    plp
    rts
@line_break:
    jsr OSNEWL
    pla
    plp
    rts
.endproc

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
