.import OSWRCH

.code
.export main
.proc main
    ldx #$00
@loop:
    lda str, x
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    rts
.endproc

.data
str: .byte "Hello World", 13, 10, 0
