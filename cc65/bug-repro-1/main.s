.code
    lda #<str
    sta $80
    lda #>str
    sta $81
    rts

.data
str:
    .byte "hello world"
