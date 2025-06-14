.feature string_escapes

.import __DATA_LOAD__
.segment "HEADER"
.dbyt $6502
.byte "APL1"
.addr __DATA_LOAD__
.addr startup

; Interrupt Vectors
.segment "NMI"
.addr $0000
.segment "RESET"
.addr WOZMON
.segment "IRQ"
.addr $0000

.code
startup:
    ; TBD: Display welcome message!
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr welcome
    jmp WOZMON

.proc welcome
    ldx #$00
@loop:
    lda welcome_message, X
    beq @done
    clc
    ora #$80
    jsr echo
    inx
    bne @loop
@done:
    rts
.endproc

.proc echo
    bit DSP         ; DA bit (B7) cleared yet?
    bmi echo        ; No, wait for display.
    sta DSP         ; Output character. Sets DA.
    rts             ; Return.
.endproc

.data
welcome_message:
    .asciiz .sprintf("Welcome to r6502!\rCtrl+C to halt, Ctrl+R to reset, Ctrl+S to save snapshot\rBASIC is at $E000\rwozmon is at $FF00\r\r")
