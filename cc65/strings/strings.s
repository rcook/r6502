PRINT_STRS_ARG  = $80
PRINT_STR_ARG   = $82

.segment "EXEHDR"
.export __EXEHDR__
__EXEHDR__:
.byte   "sim65"	; magic number
.byte   2	; simulator version: 2 = current
.byte   0	; CPU version: 0 = 6502, 1 = 65c02
.byte   $FF	; initial SP
.addr   $0000   ; load address
.addr   _main   ; start address (these are the same if _main is first in STARTUP)

.segment "STARTUP"
.export _main
.org $0E00
_main:
start:
    lda #<array
    sta PRINT_STRS_ARG
    lda #>array
    sta PRINT_STRS_ARG + 1
    jsr print_strs
    rts
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
    jsr OSWRCH
    iny
    jmp print_str_loop
print_str_end:
    rts
str0:
    .byte "string0", 10, 0
str1:
    .byte "string1", 10, 0
array:
    .byte 2
    .word str0, str1

.segment "OSWRCH"
.org $FFEE
OSWRCH:
    ora #$80            ; Set high bit
oswrch_loop:
    bit $fc02
    bmi oswrch_loop
    sta $fc02
    rts
