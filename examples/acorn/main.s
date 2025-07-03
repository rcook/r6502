.macpack r6502
.import HALT
.import OSBYTE
.import OSWORD
.import OSWRCH
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.macro print_str addr
    lda #<addr
    sta zword0
    lda #>addr
    sta zword0 + 1
    jsr print
.endmacro

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr main
    jmp HALT
.endproc

.proc main
    jsr test_osbyte
    jsr test_osword
    jsr test_oswrch
    lda #$00
    rts
.endproc

.proc test_osbyte
    print_str test_osbyte_str
    jsr OSBYTE
    rts
.endproc

.proc test_osword
    print_str test_osword_str
    jsr OSWORD
    rts
.endproc

.proc test_oswrch
    print_str test_oswrch_str
    lda #65
    jsr OSWRCH
    rts
.endproc

.proc print
    ldy #0
@loop:
    lda (zword0), y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc

.zeropage
zword0: .word $0000

.segment "SIDEWAYSDATA"
test_osbyte_str: .byte "Testing OSBYTE", 13, 10, 0
test_osword_str: .byte "Testing OSWORD", 13, 10, 0
test_oswrch_str: .byte "Testing OSWRCH", 13, 10, 0
