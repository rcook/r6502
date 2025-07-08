.macpack helpers
.importzp OSAREG
.import num_to_str

MAX_STR_LEN = 33

.data
result_len: .byte 0
result: .res MAX_STR_LEN

.segment "SIDEWAYSCODE"
.export test_num_to_str
.proc test_num_to_str
    lda #$00
    ldx #<value
    ldy #>value
    ora #%10000000
    jsr num_to_str
    sta result_len
    stx OSAREG
    sty OSAREG + 1
    tay
@loop:
    lda (OSAREG),Y
    sta result,Y
    dey
    bpl @loop

.proc check_result
    ldx result_len
    cpx expected_len
    bne @failed
@loop:
    dex
    lda result,X
    cmp expected,X
    bne @failed
    cpx #$00
    bne @loop
@success:
    return succeeded, $00
@failed:
    return failed, $01
.endproc
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "test_num_to_str passed", 13, 10, 0
failed: .byte "!!!!! test_num_to_str failed", 13, 10, 0
value: .dword $12345678
expected_len: .byte 9
expected: .byte "305419896", 0
