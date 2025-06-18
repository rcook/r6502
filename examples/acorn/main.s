.export MAIN

; The user program
.code
MAIN:
    ldx #0
@loop:
    lda str, X
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    rts

.segment "OSRODATA"
str:
    .byte "Hello World", 13, 10, 0
