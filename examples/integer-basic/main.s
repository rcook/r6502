.macpack r6502
.import copydata
.import __DATA_LOAD__

r6502_module "APL1", __DATA_LOAD__, startup

; Make sure these symbols always show up in the map file
.forceimport DSP
.forceimport DSPCR
.forceimport HALT
.forceimport KBDCR
.forceimport ECHO
.forceimport GETLINE
.forceimport PRBYTE
.forceimport PRHEX
.forceimport WOZMON

; Interrupt Vectors
.segment "NMI"
.addr $0000
.segment "RESET"
.addr WOZMON
.segment "IRQ"
.addr $0000

.code
startup:
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
    .byte "Welcome to r6502 Integer Basic Demo!", 13
    .byte "Ctrl+C to halt, Ctrl+R to reset, Ctrl+S to save snapshot", 13
    .byte "BASIC is at $E000", 13
    .byte "wozmon is at $FF00", 13
    .byte "Start BASIC with E000R", 13
    .byte "Warm-start BASIC with E2B3R", 13
    .byte 13
    .byte 0
