EXIT = $FFC0
DSP = $D012
PRINT_STRS_ARG = $80
PRINT_STR_ARG = $82

.segment "HEADER"
.byte $65
.byte $02
.byte "APL1"
.addr $8000
.addr _main

.code
.export _main
.org $C000
_main:
start:
    lda #<array
    sta PRINT_STRS_ARG
    lda #>array
    sta PRINT_STRS_ARG + 1
    jsr print_strs
    lda #$00
    jmp EXIT
print_strs:
    ldy #$00
    lda (PRINT_STRS_ARG), y
    tax                             ; X tracks number of elements remaining in array
print_strs_loop:
    cpx #$00
    beq print_strs_end
    iny
    lda (PRINT_STRS_ARG), y
    sta PRINT_STR_ARG
    iny
    lda (PRINT_STRS_ARG), y
    sta PRINT_STR_ARG + 1
    tya
    pha
    jsr print_str
    pla
    tay
    dex
    jmp print_strs_loop
print_strs_end:
    rts
print_str:
    ldy #$00
print_str_loop:
    lda (PRINT_STR_ARG), y
    cmp #$00
    beq print_str_end
    jsr write_char
    iny
    jmp print_str_loop
print_str_end:
    rts
str0:
    .byte "string0", 13, 0
str1:
    .byte "string1", 13, 0
array:
    .byte 2
    .word str0, str1

write_char:
    ora #$80            ; Set high bit
write_char_loop:
    bit DSP
    bmi write_char_loop
    sta DSP
    rts
