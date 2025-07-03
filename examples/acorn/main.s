.import OSWRCH

.segment "ROCODE"
.export main
.proc main
    ldx #0
@loop:
    lda str, x
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    rts
.endproc

.rodata
str: .byte "Hello from acorn.r6502!", 13, 10, 0
