.import HALT
.import OSWRCH
.import copydata
.import strbin
.importzp pfac

ZPPTR = $80

.code
.export startup
startup:
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr test_strbin
    jmp HALT

.proc test_strbin
    ldx #<str
    ldy #>str
    jsr strbin
    bcc @continue

    lda #<failure_overflow_str
    sta ZPPTR
    lda #>failure_overflow_str
    sta ZPPTR + 1
    jsr print_str
    lda #$02
    rts

@continue:
    .repeat 4, I
    lda pfac + I
    cmp expected_value + I
    bne @failed
    .endrepeat

    lda #<success_str
    sta ZPPTR
    lda #>success_str
    sta ZPPTR + 1
    jsr print_str
    lda #$00
    rts

@failed:
    lda #<failure_str
    sta ZPPTR
    lda #>failure_str
    sta ZPPTR + 1
    jsr print_str
    lda #$00
    rts
.endproc

.proc print_str
    ldy #$00
@loop:
    lda (ZPPTR),Y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc

.data
str:
    .byte "12345", 0

expected_value:
    .dword $00003039

success_str:
    .byte "strbin returned expected value", 13, 10, 0

failure_str:
    .byte "strbin did not return expected value", 13, 10, 0

failure_overflow_str:
    .byte "strbin failed due to overflow", 13, 10, 0
