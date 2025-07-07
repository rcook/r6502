.macpack util
.importzp OSAREG
.import OSWRCH

.segment "SIDEWAYSCODE"
.export test_preserve_stack
.proc test_preserve_stack
    lda #$11
    ldx #$22
    ldy #$33

    jsr test

    cmp #$11
    bne @failed
    cpx #$22
    bne @failed
    cpy #$33
    bne @failed
    lda za
    cmp #$11
    bne @failed
    lda zx
    cmp #$22
    bne @failed
    lda zy
    cmp #$33
    bne @failed

@passed:
    lda #<succeeded
    sta OSAREG
    lda #>succeeded
    sta OSAREG + 1
    jsr print_str
    lda #$00
    rts

@failed:
    lda #<failed
    sta OSAREG
    lda #>failed
    sta OSAREG + 1
    jsr print_str
    lda #$01
    rts
.endproc

.proc test
    save_registers
    sta za
    stx zx
    sty zy
    lda #0
    ldx #0
    ldy #0
    restore_registers
.endproc

.proc print_str
    ldy #$00
@loop:
    lda (OSAREG),Y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc

.zeropage
zword0: .byte 0
za: .byte 0
zx: .byte 0
zy: .byte 0

.segment "SIDEWAYSDATA"
succeeded: .byte "test_preserve_stack passed", 13, 10, 0
failed: .byte "test_preserve_stack failed", 13, 10, 0
