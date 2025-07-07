.macpack util
.importzp OSAREG
.import OSWRCH
.import num_to_str
.import print

MAX_STR_LEN = 33

.segment "SIDEWAYSCODE"
.export test_num_to_str
.proc test_num_to_str
    lda #$00
    ldx #<value
    ldy #>value
    ora #%10000000
    jsr num_to_str
    sta result_str_len
    stx OSAREG
    sty OSAREG + 1
    tay
@loop:
    lda (OSAREG),Y
    sta result_str,Y
    dey
    bpl @loop

.proc check_result_str
    ldx result_str_len
    cpx expected_str_len
    bne @failed
@loop:
    dex
    lda result_str,X
    cmp expected_str,X
    bne @failed
    cpx #$00
    bne @loop
@success:
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

.data
result_str_len: .byte 0
result_str: .res MAX_STR_LEN

.segment "SIDEWAYSDATA"
succeeded: .byte "test_num_to_str passed", 13, 10, 0
failed: .byte "test_num_to_str failed", 13, 10, 0
value: .dword $12345678
expected_str_len: .byte 9
expected_str: .byte "305419896", 0
