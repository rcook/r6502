.alias EXIT $ffc0
.alias OSWRCH $ffee

.org $0e00

main:
        ldx #0
loop:
        lda hello, x
        beq done
        jsr OSWRCH
        inx
        bne loop
done:
        jmp EXIT

hello:  .byte "HELLO, WORLD!", 0
