; https://github.com/chelsea6502/BeebEater
; https://hackaday.io/project/177384-w65c816sxb-investigation/log/189547-porting-bbc-basic
; https://mdfs.net/Software/BBCBasic/Porting/Porting.htm
;
; The bare minimum your MOS needs to do is:
; BRKV, WRCHV:    vectors for BRK and WRCH [TBD]
; BRK:            point &FD/E to error message, jump via BRKV [TBD]
; OSWRCH/WRCHV:   print characters [IMPLEMENTED]
; OSWORD 0:       read input line [IMPLEMENTED]
; OSBYTE &83/&84: read bottom of memory/top of memory [IMPLEMENTED]
; &0000-&005F:    zero page workspace for BASIC [IMPLEMENTED]
; &0400-&07FF:    fixed workspace for BASIC [IMPLEMENTED]
; Enter BASIC at its entry point with A=1
.macpack generic
.macpack helpers
.import startup

.macro read_char
.local @loop
@loop:
    lda KBDCR
    bpl @loop
    lda KBD
    and #$7F
.endmacro

.macro write_char
.local @loop
@loop:
    bit DSP
    bmi @loop
    sta DSP
.endmacro

.macro new_line
    php
    pha
    lda #LF
    write_char
    lda #CR
    write_char
    pla
    plp
.endmacro

.zeropage
ztempword0: .word $0000
ztempword1: .word $0000
ztempbyte0: .byte $00
ztempbyte1: .byte $00

; ASCII control characters
LF = 10
CR = 13
DEL = 127

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
    pha
    txa
    pha

    stx ztempword0      ; LSB of parameter block address
    sty ztempword0 + 1  ; MSB of parameter block address

    ldy #$00
    lda (ztempword0), y
    sta ztempword1      ; LSB of buffer
    iny
    lda (ztempword0), y
    sta ztempword1 + 1  ; MSB of buffer
    iny
    lda (ztempword0), y
    tax                 ; Buffer length
    iny
    lda (ztempword0), y
    sta ztempbyte0      ; Minimum character value
    iny
    lda (ztempword0), y
    sta ztempbyte1      ; Maximum character value    

@loop:
    read_char
    cmp #DEL
    bne @check_cr

    ; Backspace
    cpy #$00
    beq @loop
    dey

    write_char

    ; Overwrite last character
    lda #' '
    write_char
    lda #DEL
    write_char
    lda #$00
    beq @loop

@check_cr:
    cmp #CR
    beq @done

    ; Check that we haven't exceeded buffer length
    cpx #$00
    bne @cont
    lda #$07            ; BEL
    write_char
    bne @loop

@cont:
    write_char
    cmp ztempbyte0
    blt @loop
    cmp ztempbyte1
    bgt @loop
    sta (ztempword1), y
    dex
    iny
    bne @loop

@done:
    new_line

    pla
    tax
    pla
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
    write_char
    rts
.endproc

.segment "OSNEWL"
.export OSNEWL
.proc OSNEWL
    jmp osnewl_impl
.endproc

.segment "ROCODE"
.proc osnewl_impl
    new_line
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

.segment "OSRDCH"
.export OSRDCH
.proc OSRDCH
    jmp osrdch_impl
.endproc

.segment "ROCODE"
.proc osrdch_impl
@loop:
    read_char
    rts
.endproc

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
