.importzp OSAREG
.import OSWRCH
.import str_to_num
.importzp pfac

.segment "SIDEWAYSCODE"
.export test_str_to_num
.proc test_str_to_num
    ldx #<str
    ldy #>str
    jsr str_to_num
    bcc @continue

    lda #<failed_due_to_overflow
    sta OSAREG
    lda #>failed_due_to_overflow
    sta OSAREG + 1
    jsr print_str
    lda #$02
    rts

@continue:
    .repeat 4, I
    lda pfac + I
    cmp expected_value + I
    bne @failed
    .endrepeat

    lda #<succeeded
    sta OSAREG
    lda #>succeeded
    sta OSAREG + 1
    jsr print_str
    lda #$00
    rts

@failed:
    lda #<failed
    sta OSAREG
    lda #>failed
    sta OSAREG + 1
    jsr print_str
    lda #$01
    rts
.endproc

.proc print_str
    ldy #$00
@loop:
    lda (OSAREG),Y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "test_str_to_num passed", 13, 10, 0
failed: .byte "test_str_to_num failed", 13, 10, 0
failed_due_to_overflow: .byte "test_str_to_num failed due to overflow", 13, 10, 0
str: .byte "12345", 0
expected_value: .dword $00003039
