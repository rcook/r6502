.macpack helpers
.macpack r6502
.import HALT
.import OSASCI
.import OSBYTE
.import OSNEWL
.import OSWORD
.import OSWRCH
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

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
    jsr test_osasci
    jsr test_osbyte
    jsr test_osnewl
    jsr test_osword
    jsr test_oswrch
    lda #$00
    rts
.endproc

.proc test_osasci
    print_str test_osasci_str
    lda #65
    jsr OSASCI
    lda #13
    jsr OSASCI
    rts
.endproc

.proc test_osbyte
    print_str test_osbyte_str

    lda #$83
    jsr OSBYTE
    cpx #$00
    bne @failed
    cpy #$0e
    bne @failed

    lda #$84
    jsr OSBYTE
    cpx #$00
    bne @failed
    cpy #$80
    bne @failed

    rts

@failed:
    lda #$01
    print_str failed_str
    jmp HALT
.endproc

.proc test_osnewl
    print_str test_osnewl_str
    jsr OSNEWL
    rts
.endproc

.proc test_osword
    print_str test_osword_str

    ; Make sure first character in buffer is zero
    lda #$00
    sta buffer
    sta buffer + 1

    ; Set up parameter block
    lda #<buffer
    sta osword_print_line_params
    lda #>buffer
    sta osword_print_line_params + 1
    lda #buffer_end - buffer
    sta osword_print_line_params + 2
    lda #'A'
    sta osword_print_line_params + 3
    lda #'Z'
    sta osword_print_line_params + 4

    ; Call OSWORD $00 (read line)
    lda #$00
    ldx #<osword_print_line_params
    ldy #>osword_print_line_params
    jsr OSWORD

    assert lda buffer, cmp #'A', "character not A"
    assert lda buffer + 1, cmp #'B', "character not B"

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
test_osasci_str: .byte "Testing OSASCI", 13, 10, 0
test_osbyte_str: .byte "Testing OSBYTE", 13, 10, 0
test_osnewl_str: .byte "Testing OSNEWL", 13, 10, 0
test_osword_str: .byte "Testing OSWORD", 13, 10, 0
test_oswrch_str: .byte "Testing OSWRCH", 13, 10, 0
failed_str: .byte "Failed", 13, 10, 0

.data
buffer: .res 10
buffer_end:
osword_print_line_params: .res 5
