.macpack generic
.macpack r6502
.macpack raw

.importzp OSAREG
.importzp OSXREG
.importzp OSYREG

.importzp zbyte0
.importzp zbyte1
.importzp zword0
.importzp zword1

.importzp DEL
.importzp ESC

.import HIMEM
.import OSHWM

.import HOSTHOOK
.importzp CLIVHOSTHOOK
.importzp FILEVHOSTHOOK

.segment "ROCODE"
.export userv_entrypoint
.proc userv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: userv"
.endproc

.segment "ROCODE"
.export brkv_entrypoint
.proc brkv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: brkv"
.endproc

.segment "ROCODE"
.export irq1v_entrypoint
.proc irq1v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: irq1v"
.endproc

.segment "ROCODE"
.export irq2v_entrypoint
.proc irq2v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: irq2v"
.endproc

.segment "ROCODE"
.export cliv_entrypoint
.proc cliv_entrypoint
    lda #CLIVHOSTHOOK
    jmp HOSTHOOK
    raw_not_impl "NOT IMPLEMENTED: cliv"
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

; Inspired by https://github.com/chelsea6502/BeebEater/blob/main/BeebEater.asm
.segment "ROCODE"
.export wordv_entrypoint
.proc wordv_entrypoint
    php
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

@loop:
    raw_read_char_to_a
@check_esc:
    cmp #ESC
    bne @check_del

    lda OSAREG
    ldx OSXREG
    plp
    sec                 ; C = 1 indicates Escape, C = 0 otherwise
    rts

@check_del:
    cmp #DEL
    bne @check_cr

    ; Backspace
    cpy #$00        ; Don't delete beyond start of buffer!
    beq @loop
    dey

    raw_write_char_from_a

    ; Overwrite last character
    lda #' '
    raw_write_char_from_a
    lda #DEL
    raw_write_char_from_a
    lda #$00
    beq @loop

@check_cr:
    cmp #CR
    beq @done

    ; Check that we haven't exceeded buffer length
    cpx #$00
    bne @cont
    lda #$07            ; BEL
    raw_write_char_from_a
    bne @loop

@cont:
    raw_write_char_from_a
    cmp zbyte0
    blt @loop
    cmp zbyte1
    bgt @loop
    sta (zword1), y
    dex
    iny
    bne @loop

@done:
    ; Terminate string with CR
    lda #CR
    sta (zword1), y

    raw_write_new_line

    lda OSAREG
    ldx OSXREG
    ;ldy OSYREG         ; Document (Advanced User Guide etc.)
                        ; says "Y contains line length, including
                        ; carriage return if used". However, my experiments
                        ; on an emulator demonstrate that Y should not
                        ; include the CR.
    plp
    clc                 ; C = 1 indicates Escape, C = 0 otherwise
    rts
.endproc

.segment "ROCODE"
.export wrchv_entrypoint
.proc wrchv_entrypoint
    raw_write_char_from_a
    rts
.endproc

.segment "ROCODE"
.export rdchv_entrypoint
.proc rdchv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: rdchv"
    raw_read_char_to_a
    rts
.endproc

.segment "ROCODE"
.export filev_entrypoint
.proc filev_entrypoint
    lda #FILEVHOSTHOOK
    jmp HOSTHOOK
    raw_not_impl "NOT IMPLEMENTED: filev"
.endproc

.segment "ROCODE"
.export argsv_entrypoint
.proc argsv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: argsv"
.endproc

.segment "ROCODE"
.export bgetv_entrypoint
.proc bgetv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: bgetv"
.endproc

.segment "ROCODE"
.export bputv_entrypoint
.proc bputv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: bputv"
    brk
.endproc

.segment "ROCODE"
.export gbpbv_entrypoint
.proc gbpbv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: gbpbv"
.endproc

.segment "ROCODE"
.export findv_entrypoint
.proc findv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: findv"
.endproc

.segment "ROCODE"
.export fscv_entrypoint
.proc fscv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: fscv"
.endproc

.segment "ROCODE"
.export evntv_entrypoint
.proc evntv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: evntv"
.endproc

.segment "ROCODE"
.export uptv_entrypoint
.proc uptv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: uptv"
.endproc

.segment "ROCODE"
.export netv_entrypoint
.proc netv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: netv"
.endproc

.segment "ROCODE"
.export vduv_entrypoint
.proc vduv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: vduv"
.endproc

.segment "ROCODE"
.export keyv_entrypoint
.proc keyv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: keyv"
.endproc

.segment "ROCODE"
.export insv_entrypoint
.proc insv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: insv"
.endproc

.segment "ROCODE"
.export remv_entrypoint
.proc remv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: remv"
.endproc

.segment "ROCODE"
.export cnpv_entrypoint
.proc cnpv_entrypoint
    raw_not_impl "NOT IMPLEMENTED: cnpv"
.endproc

.segment "ROCODE"
.export ind1v_entrypoint
.proc ind1v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind1v"
.endproc

.segment "ROCODE"
.export ind2v_entrypoint
.proc ind2v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind2v"
.endproc

.segment "ROCODE"
.export ind3v_entrypoint
.proc ind3v_entrypoint
    raw_not_impl "NOT IMPLEMENTED: ind3v"
.endproc
