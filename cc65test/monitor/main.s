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
        LDA #$23
        BRK
