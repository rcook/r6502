.macpack generic
.macpack raw
.macpack r6502

.importzp CLIVHOSTHOOK
.importzp CR
.importzp DEL
.importzp FILEVHOSTHOOK
.importzp OSAREG
.importzp OSESC
.importzp OSKBD1
.importzp OSXREG
.importzp OSYREG
.importzp zbyte0
.importzp zbyte1
.importzp zword0
.importzp zword1

BIT7 = 1 << 7

.segment "MOS"
.export userv_entrypoint
.proc userv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: userv"
.endproc

.segment "MOS"
.export brkv_entrypoint
.proc brkv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: brkv"
.endproc

.segment "MOS"
.export irq1v_entrypoint
.proc irq1v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: irq1v"
.endproc

.segment "MOS"
.export irq2v_entrypoint
.proc irq2v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: irq2v"
.endproc

.segment "MOS"
.export cliv_entrypoint
.proc cliv_entrypoint
    lda #CLIVHOSTHOOK
    sta OSAREG
    jsr HOSTHOOK
    cmp #$00
    beq @done
    brk
.byte $FE
.byte "Bad command"
.byte $00
@done:
    rts
.endproc

.segment "MOS"
.export bytev_entrypoint
.proc bytev_entrypoint
    php
    cmp #$7E
    beq @osbyte_126_ack_esc
    cmp #$83
    beq @osbyte_131_oshwm
    cmp #$84
    beq @osbyte_132_himem
    plp
    rts

@osbyte_126_ack_esc:
    ldx OSESC
    cpx #$FF
    bne @osbyte_126_ack_esc_clear
    ldx #$00
    stx OSESC
    ldx #$FF
    bne @osbyte_126_ack_esc_done
@osbyte_126_ack_esc_clear:
    ldx #$00
    stx OSESC
@osbyte_126_ack_esc_done:
    plp
    clc
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

; Inspired by https://github.com/chelsea6502/BeebEater/blob/main/BeebEater.asm
.segment "MOS"
.export wordv_entrypoint
.proc wordv_entrypoint
    php
    cli
    sta OSAREG
    stx OSXREG
    sty OSYREG

    cmp #$00
    beq @osword_00

    plp
    rts
@osword_00:
    stx zword0      ; LSB of parameter block address
    sty zword0 + 1  ; MSB of parameter block address

    ldy #$00
    lda (zword0), y
    sta zword1      ; LSB of buffer address
    iny
    lda (zword0), y
    sta zword1 + 1  ; MSB of buffer address
    iny
    lda (zword0), y
    tax             ; Buffer length (caller must allocate buffer length + 1 to include CR)
    iny
    lda (zword0), y
    sta zbyte0      ; Minimum character value
    iny
    lda (zword0), y
    sta zbyte1      ; Maximum character value

    ldy #$00

loop:
    jsr OSRDCH
    bcc @check_del

    plp
    lda OSESC
    rol
    cli
    rts

@check_del:
    cmp #DEL
    bne @check_cr

    ; Backspace
    cpy #$00        ; Don't delete beyond start of buffer!
    beq loop
    dey

    jsr OSWRCH

    ; Overwrite last character
    lda #' '
    jsr OSWRCH
    lda #DEL
    jsr OSWRCH
    beq loop

@check_cr:
    cmp #CR
    beq done

    ; Check that we haven't exceeded buffer length
    cpx #$00
    bne @cont
    lda #$07            ; BEL
    jsr OSWRCH
    bne loop

@cont:
    jsr OSWRCH
    cmp zbyte0
    blt loop
    cmp zbyte1
    bgt loop
    sta (zword1), y
    dex
    iny
    bne loop

done:
    ; Terminate string with CR
    lda #CR
    sta (zword1), y

    jsr OSNEWL

    lda OSAREG
    ldx OSXREG
    ;ldy OSYREG         ; Documentation (Advanced User Guide etc.)
                        ; says "Y contains line length, including
                        ; carriage return if used". However, my experiments
                        ; on an emulator demonstrate that Y should not
                        ; include the CR.
    plp
    clc                 ; C = 1 indicates Escape, C = 0 otherwise
    rts
.endproc

.segment "MOS"
.export wrchv_entrypoint
.proc wrchv_entrypoint
@loop:
    bit DSP
    bmi @loop
    sta DSP
    rts
.endproc

.segment "MOS"
.export rdchv_entrypoint
.proc rdchv_entrypoint
    lda OSESC
    and #BIT7
    beq @loop
    lda #$00
    sec
    rts
@loop:
    lda OSKBD1
    beq @loop
    pha
    lda #$00
    sta OSKBD1
    pla
    clc
    rts
.endproc

.segment "MOS"
.export filev_entrypoint
.proc filev_entrypoint
    pha
    lda #FILEVHOSTHOOK
    sta OSAREG
    pla
    jsr HOSTHOOK
    cmp #$00
    beq @done
    brk
.byte $FE
.byte "File I/O error"
.byte $00
@done:
    rts
.endproc

.segment "MOS"
.export argsv_entrypoint
.proc argsv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: argsv"
.endproc

.segment "MOS"
.export bgetv_entrypoint
.proc bgetv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: bgetv"
.endproc

.segment "MOS"
.export bputv_entrypoint
.proc bputv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: bputv"
    brk
.endproc

.segment "MOS"
.export gbpbv_entrypoint
.proc gbpbv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: gbpbv"
.endproc

.segment "MOS"
.export findv_entrypoint
.proc findv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: findv"
.endproc

.segment "MOS"
.export fscv_entrypoint
.proc fscv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: fscv"
.endproc

.segment "MOS"
.export evntv_entrypoint
.proc evntv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: evntv"
.endproc

.segment "MOS"
.export uptv_entrypoint
.proc uptv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: uptv"
.endproc

.segment "MOS"
.export netv_entrypoint
.proc netv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: netv"
.endproc

.segment "MOS"
.export vduv_entrypoint
.proc vduv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: vduv"
.endproc

.segment "MOS"
.export keyv_entrypoint
.proc keyv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: keyv"
.endproc

.segment "MOS"
.export insv_entrypoint
.proc insv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: insv"
.endproc

.segment "MOS"
.export remv_entrypoint
.proc remv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: remv"
.endproc

.segment "MOS"
.export cnpv_entrypoint
.proc cnpv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: cnpv"
.endproc

.segment "MOS"
.export ind1v_entrypoint
.proc ind1v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind1v"
.endproc

.segment "MOS"
.export ind2v_entrypoint
.proc ind2v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind2v"
.endproc

.segment "MOS"
.export ind3v_entrypoint
.proc ind3v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind3v"
.endproc
