.macpack generic
.macpack helpers

.importzp ztempbyte0
.importzp ztempbyte1
.importzp ztempword0
.importzp ztempword1

.importzp CR
.importzp DEL
.importzp LF

.import DSP
.import KBD
.import KBDCR

.import HIMEM
.import OSHWM

.segment "ROCODE"
.export userv_entrypoint
.proc userv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export brkv_entrypoint
.proc brkv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export irq1v_entrypoint
.proc irq1v_entrypoint
    brk
.endproc

.segment "ROCODE"
.export irq2v_entrypoint
.proc irq2v_entrypoint
    brk
.endproc

.segment "ROCODE"
.export cliv_entrypoint
.proc cliv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export bytev_entrypoint
.proc bytev_entrypoint
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

.segment "ROCODE"
.export wordv_entrypoint
.proc wordv_entrypoint
    php
    cmp #$00
    beq @osword_00
    plp
    rts
@osword_00:
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

.segment "ROCODE"
.export wrchv_entrypoint
.proc wrchv_entrypoint
    write_char
    rts
.endproc

.segment "ROCODE"
.export rdchv_entrypoint
.proc rdchv_entrypoint
    read_char
    rts
.endproc

.segment "ROCODE"
.export filev_entrypoint
.proc filev_entrypoint
    brk
.endproc

.segment "ROCODE"
.export argsv_entrypoint
.proc argsv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export bgetv_entrypoint
.proc bgetv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export bputv_entrypoint
.proc bputv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export gbpbv_entrypoint
.proc gbpbv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export findv_entrypoint
.proc findv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export fscv_entrypoint
.proc fscv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export evntv_entrypoint
.proc evntv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export uptv_entrypoint
.proc uptv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export netv_entrypoint
.proc netv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export vduv_entrypoint
.proc vduv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export keyv_entrypoint
.proc keyv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export insv_entrypoint
.proc insv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export remv_entrypoint
.proc remv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export cnpv_entrypoint
.proc cnpv_entrypoint
    brk
.endproc

.segment "ROCODE"
.export ind1v_entrypoint
.proc ind1v_entrypoint
    brk
.endproc

.segment "ROCODE"
.export ind2v_entrypoint
.proc ind2v_entrypoint
    brk
.endproc

.segment "ROCODE"
.export ind3v_entrypoint
.proc ind3v_entrypoint
    brk
.endproc
