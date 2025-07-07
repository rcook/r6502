.macpack r6502
.macpack raw
.importzp OSAREG
.import OSASCI
.import OSBYTE
.import OSNEWL
.import OSRDCH
.import OSWORD
.import OSWRCH
.import __SIDEWAYSHEADER_LOAD__

r6502_system "ACRN", __SIDEWAYSHEADER_LOAD__
sideways_rom_header entrypoint, , , , "acorn-test", "1.0", "2025 Richard Cook"

BUFFER_LEN = 10

.segment "SIDEWAYSCODE"
.proc entrypoint
    ;jsr test_osasci
    ;jsr test_osbyte
    ;jsr test_osnewl
    ;jsr test_osrdch
    jsr test_osword
    ;jsr test_oswrch
    syshalt $00
.endproc

.proc test_osasci
    raw_write_str test_osasci_banner

    lda #65
    jsr OSASCI
    lda #13
    jsr OSASCI

    rts
.endproc

.proc test_osbyte
    raw_write_str test_osbyte_banner

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
    raw_write_str failed
    syshalt $01
.endproc

.proc test_osnewl
    raw_write_str test_osnewl_banner

    jsr OSNEWL

    rts
.endproc

.proc test_osrdch
    raw_write_str test_osrdch_banner

    raw_write_str key_prompt
    jsr OSRDCH
    pha
    jsr OSNEWL
    raw_write_str key_result
    pla
    jsr OSWRCH
    jsr OSNEWL

    rts
.endproc

.proc test_osword
    raw_write_str test_osword_banner

    raw_write_str line_prompt

    ; Set up parameter block
    lda #<buffer
    sta param_block
    lda #>buffer
    sta param_block + 1
    lda #BUFFER_LEN - 1          ; OSWORD $00 returns extra CR character at end
    sta param_block + 2
    lda #$00
    sta param_block + 3
    lda #$FF
    sta param_block + 4

    ; Call OSWORD $00 (read line)
    lda #$00
    ldx #<param_block
    ldy #>param_block
    jsr OSWORD

    ; C = 1 if Esc was pressed
    bcc @display_str
    raw_write_new_line
    raw_write_str escaped

    rts

    ; Y contains number of characters read not including the CR
    ; Move this to X
@display_str:
    tya
    tax

    raw_write_str line_result
    raw_write_str buffer
    rts
.endproc

.proc test_oswrch
    raw_write_str test_oswrch_banner

    lda #65
    jsr OSWRCH

    rts
.endproc

.segment "SIDEWAYSDATA"
test_osasci_banner: .byte "Testing OSASCI", 13, 10, 0
test_osbyte_banner: .byte "Testing OSBYTE", 13, 10, 0
test_osnewl_banner: .byte "Testing OSNEWL", 13, 10, 0
test_osrdch_banner: .byte "Testing OSRDCH", 13, 10, 0
test_osword_banner: .byte "Testing OSWORD", 13, 10, 0
test_oswrch_banner: .byte "Testing OSWRCH", 13, 10, 0
failed: .byte "Failed", 13, 10, 0
key_prompt: .byte "Press a key: ", 0
key_result: .byte "You pressed: ", 0
line_prompt: .byte "Enter some text followed by Enter: ", 0
line_result: .byte "You typed: ", 0
escaped: .byte "You pressed Esc", 0

.data
buffer: .res BUFFER_LEN
param_block: .res 5
