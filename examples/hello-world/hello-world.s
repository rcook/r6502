EXIT = $FFC0
OSWRCH = $FFEE

.segment "HEADER"
.byte $65
.byte $02
.byte "ACRN"
.addr $8000
.addr main

.code
.org $c000
main:
    ldx #0
loop:
    lda hello, X
    beq done
    jsr OSWRCH
    inx
    bne loop
done:
    jmp EXIT

hello:
    .byte "HELLO, WORLD!", 0
