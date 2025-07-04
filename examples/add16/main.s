.macpack r6502
.macpack util
.import copydata
.import print
.import __CODE_LOAD__
.exportzp zword0

r6502_header "ACRN", __CODE_LOAD__, startup

.code
.proc startup
    sysinit
    jsr copydata
    jsr test_add16
    syshalt
.endproc

.proc test_add16
    print_buf welcome
    add16 left, right
    lda result
    bcs @failed
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

.zeropage
zword0: .word $0000

.data
welcome: .byte "ADD16 TEST", 13, 10, 0
succeeded: .byte "Test passed", 13, 10, 0
failed: .byte "Test failed", 13, 10, 0
left: .word $3412
right: .word $7856
result: .word $0000
