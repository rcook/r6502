.macpack util

.exportzp zword0

.code
.export MAIN
.proc MAIN
    print_buf hello

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
    print_buf passed
    lda #0
    jmp OSEXIT

@failed:
    print_buf failed
    lda #1
    jmp OSEXIT
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

.zeropage
zword0: .byte 0
za: .byte 0
zx: .byte 0
zy: .byte 0

.data
hello:
    .byte "REGISTER PRESERVATION TEST", 13, 10, 0
passed:
    .byte "Registers successfully preserved", 13, 10, 0
failed:
    .byte "Registers not preserved", 13, 10, 0
