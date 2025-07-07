.macpack util
.import print
.importzp zword0

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
    print_buf succeeded
    lda #$00
    rts
@failed:
    print_buf failed
    lda #$01
    rts
.endproc

.data
left: .word $3412
right: .word $7856
result: .word $0000

.segment "SIDEWAYSDATA"
succeeded: .byte "test_add16 passed", 13, 10, 0
failed: .byte "test_add16 failed", 13, 10, 0
