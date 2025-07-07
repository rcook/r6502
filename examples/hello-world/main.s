.macpack r6502
.import OSWRCH
.import __SIDEWAYSHEADER_LOAD__

r6502_system "ACRN", __SIDEWAYSHEADER_LOAD__
sideways_rom_header entrypoint, , , , "hello-world", "1.0", "2025 Richard Cook"

.segment "SIDEWAYSCODE"
.proc entrypoint
    ldx #$00
@loop:
    lda str, x
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    syshalt
.endproc

.segment "SIDEWAYSDATA"
str: .byte "Hello World!", 13, 10, 0
