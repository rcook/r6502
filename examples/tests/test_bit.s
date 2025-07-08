.macpack bit
.macpack helpers
.macpack util

.data
value: .byte $00

.segment "SIDEWAYSCODE"
.export test_extensions
.proc test_extensions
    lda #$00
    setbit 2
    cmp #$04
    bne @failed

    lda #$04
    clearbit 2
    cmp #$00
    bne @failed

    lda #$04
    ifbit 2, @cont
    jmp @failed
@cont:

    lda #$04
    ifnbit 2, @failed

@succeeded:
    return succeeded, $00
@failed:
    return failed, $01
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "test_bit passed", 13, 10, 0
failed: .byte "!!!!! test_bit failed", 13, 10, 0
