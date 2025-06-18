OSWRCH = $FFEE

.code
.export MAIN
MAIN:
    ldx #$00
hello_world_loop:
    lda hello_world_string, X
    beq hello_world_done
    jsr OSWRCH
    inx
    bne hello_world_loop
hello_world_done:
    rts

.data
hello_world_string:
    .byte "HELLO, WORLD!", 13, 10, 0
