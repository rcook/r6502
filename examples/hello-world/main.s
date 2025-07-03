.import OSWRCH

.segment "SIDEWAYSCODE"
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

.segment "SIDEWAYSDATA"
str: .byte "Hello World!", 13, 10, 0
