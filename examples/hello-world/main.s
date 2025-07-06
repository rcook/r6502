.macpack r6502
.import OSWRCH
.import __SIDEWAYSCODE_LOAD__

r6502_system "ACRN", __SIDEWAYSCODE_LOAD__

.segment "SIDEWAYSCODE"
.proc entrypoint
    sideways_rom_header @go, , , , "hello-world", "1.0", "2025 Richard Cook"
@go:
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
