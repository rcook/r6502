; Reference: https://www.llx.com/Neil/a2/mult.html

.org $0e00
start:
    lda #0
    sta REM
    sta REM + 1
    ldx #16
l1:
    asl NUM1
    rol NUM1 + 1
    rol REM
    rol REM + 1
    lda REM
    sec
    sbc NUM2
    tay
    lda REM + 1
    sbc NUM2 + 1
    bcc l2
    sta REM + 1
    sty REM
    inc NUM1
l2:
    dex
    bne l1
    rts

NUM1:
    .word $1234
NUM2:
    .word $000a
REM:
    .word $0000
