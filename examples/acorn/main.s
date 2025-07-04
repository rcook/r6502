.macpack r6502
.macpack raw
.import OSASCI
.import OSBYTE
.import OSNEWL
.import OSRDCH
.import OSWORD
.import OSWRCH
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.zeropage
zword0: .word $0000

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    sysinit
    jsr main
    syshalt
.endproc

.proc main
    ;jsr test_osasci
    ;jsr test_osbyte
    ;jsr test_osnewl
    ;jsr test_osrdch
    jsr test_osword
    ;jsr test_oswrch
    lda #$00
    rts
.endproc

.proc test_osasci
    raw_write_str test_osasci_str
    lda #65
    jsr OSASCI
    lda #13
    jsr OSASCI
    rts
.endproc

.proc test_osbyte
    raw_write_str test_osbyte_str

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
    raw_write_str failed_str
    syshalt
.endproc

.proc test_osnewl
    raw_write_str test_osnewl_str
    jsr OSNEWL
    rts
.endproc

.proc test_osrdch
    raw_write_str test_osrdch_str
    raw_write_str prompt_str
    jsr OSRDCH
    pha
    jsr OSNEWL
    raw_write_str you_pressed_str
    pla
    jsr OSWRCH
    jsr OSNEWL
    rts
.endproc

.proc test_osword
    raw_write_str test_osword_str

    raw_write_str line_prompt_str

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
    lda #$00
    sta osword_print_line_params + 3
    lda #$FF
    sta osword_print_line_params + 4

    ; Call OSWORD $00 (read line)
    lda #$00
    ldx #<osword_print_line_params
    ldy #>osword_print_line_params
    jsr OSWORD

    ; Y contains number of characters read
    ; Move this to X
    tya
    tax

    raw_write_str line_result_str

    lda #<buffer
    sta zword0
    lda #>buffer
    sta zword0 + 1

    cpx #$00
    beq @done
    ldy #$00
@loop:
    lda (zword0), y
    jsr OSWRCH
    iny
    dex
    bne @loop
@done:

    rts
.endproc

.proc test_oswrch
    raw_write_str test_oswrch_str
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

.segment "SIDEWAYSDATA"
test_osasci_str: .byte "Testing OSASCI", 13, 10, 0
test_osbyte_str: .byte "Testing OSBYTE", 13, 10, 0
test_osnewl_str: .byte "Testing OSNEWL", 13, 10, 0
test_osrdch_str: .byte "Testing OSRDCH", 13, 10, 0
test_osword_str: .byte "Testing OSWORD", 13, 10, 0
test_oswrch_str: .byte "Testing OSWRCH", 13, 10, 0
prompt_str: .byte "Press a key: ", 0
you_pressed_str: .byte "You pressed: ", 0
failed_str: .byte "Failed", 13, 10, 0
line_prompt_str: .byte "Enter some text followed by Enter: ", 0
line_result_str: .byte "You typed: ", 0

.segment "USERDATA"
buffer: .res 10
buffer_end:
osword_print_line_params: .res 5
