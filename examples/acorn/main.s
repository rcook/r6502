.import OSWRCH

; The user program
.code
.export main
.proc main
    ldx #0
@loop:
    lda str, X
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    rts
.endproc

.segment "OSRODATA"
str: .byte "Hello World", 13, 10, 0
