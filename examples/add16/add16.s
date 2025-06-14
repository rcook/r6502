.import __CODE_LOAD__

EXIT = $FFC0

.segment "HEADER"
.dbyt $6502
.byte "ACRN"
.addr __CODE_LOAD__
.addr main

.code
main:
    jsr add16
    lda result
    bcs failed
    cmp #$68
    bne failed
    lda result + 1
    cmp #$ac
    bne failed
succeeded:
    lda #$00
    jmp EXIT
failed:
    lda #$01
    jmp EXIT

add16:
    clc
    lda left_operand
    adc right_operand
    sta result
    lda left_operand + 1
    adc right_operand + 1
    sta result + 1
    rts

left_operand:
    .word $3412

right_operand:
    .word $7856

result:
    .word $0000
