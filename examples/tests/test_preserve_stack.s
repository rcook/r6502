.macpack helpers
.macpack util
.importzp OSAREG
.importzp OSXREG
.importzp OSYREG

za = OSAREG
zx = OSXREG
zy = OSYREG

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
    return succeeded, $00
@failed:
    return failed, $01
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

.segment "SIDEWAYSDATA"
succeeded: .byte "test_preserve_stack passed", 13, 10, 0
failed: .byte "!!!!! test_preserve_stack failed", 13, 10, 0
