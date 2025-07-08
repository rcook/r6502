.macpack helpers
.macpack util

.data
left: .word $3412
right: .word $7856
result: .word $0000

.segment "SIDEWAYSCODE"
.export test_add16
.proc test_add16
    add16 left, right, result
    bcs @failed
    lda result
    cmp #$68
    bne @failed
    lda result + 1
    cmp #$ac
    bne @failed
@succeeded:
    return succeeded, $00
@failed:
    return failed, $01
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "test_add16 passed", 13, 10, 0
failed: .byte "!!!!! test_add16 failed", 13, 10, 0
