.macpack r6502
.macpack util
.import copydata
.import print
.import __SIDEWAYSCODE_LOAD__
.exportzp zword0

r6502_system "ACRN", __SIDEWAYSCODE_LOAD__

.zeropage
zword0: .word $0000

.data
left: .word $3412
right: .word $7856
result: .word $0000

.segment "SIDEWAYSCODE"
.proc entrypoint
    sideways_rom_header @go, , , , "add16", "1.0", "2025 Richard Cook"
@go:
    jsr copydata
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
    syshalt
@failed:
    print_buf failed
    lda #$01
    syshalt
.endproc

.segment "SIDEWAYSDATA"
succeeded: .byte "Test passed", 13, 10, 0
failed: .byte "Test failed", 13, 10, 0
