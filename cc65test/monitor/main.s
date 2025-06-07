DSP             = $D012         ;  PIA.B display output register
DSPCR           = $D013         ;  PIA.B display control register

        .segment "EXEHDR"
        .export __EXEHDR__
__EXEHDR__:
        .byte   "sim65"	; magic number
        .byte   2	; simulator version: 2 = current
        .byte   0	; CPU version: 0 = 6502, 1 = 65c02
        .byte   $FF	; initial SP
        .addr   $0000   ; load address
        .addr   _main   ; start address (these are the same if _main is first in STARTUP)

        .segment "STARTUP"
        .export _main
_main:
        ldx #$00
loop:
        lda message, x
        beq done
        clc
        adc #$80
        jsr echo
        inx
        bne loop
done:
        jmp $ff1f       ; https://www.sbprojects.net/projects/apple1/wozmon.php
echo:
        bit DSP         ; DA bit (B7) cleared yet?
        bmi echo        ; No, wait for display.
        sta DSP         ; Output character. Sets DA.
        rts             ; Return.
message:
        .byte "Hello World"
        .byte $00
