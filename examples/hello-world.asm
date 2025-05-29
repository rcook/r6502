.alias OSWRCH $ffee
.org $0e00

        ldx #0
loop:   lda hello, x
        beq done
        jsr OSWRCH
        inx
        bne loop
done:   rts

hello:  .byte "HELLO, WORLD!", 0
