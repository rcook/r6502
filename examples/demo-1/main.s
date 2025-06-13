.export MAIN

; The user program
.code
MAIN:
    ldx #0
loop:
    lda str, X
    beq done
    jsr OSWRCH
    inx
    bne loop
done:
    rts

.data
str:
.byte "Hello World", 0
