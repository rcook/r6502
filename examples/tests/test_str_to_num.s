.macpack helpers
.import str_to_num
.importzp pfac

.segment "SIDEWAYSCODE"
.export test_str_to_num
.proc test_str_to_num
    ldx #<str
    ldy #>str
    jsr str_to_num
    bcc @continue

    return failed_due_to_overflow, $02

@continue:
    .repeat 4, I
    lda pfac + I
    cmp expected_value + I
    bne @failed
    .endrepeat

    return succeeded, $00

@failed:
    return failed, $01
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "test_str_to_num passed", 13, 10, 0
failed: .byte "!!!!! test_str_to_num failed", 13, 10, 0
failed_due_to_overflow: .byte "test_str_to_num failed due to overflow", 13, 10, 0
str: .byte "12345", 0
expected_value: .dword $00003039
