.export STARTUP

; r6502 executable header
.segment "HEADER"
.dbyt $6502
.byte $00
.byte "ACRN"
.import __DATA_LOAD__
.addr __DATA_LOAD__
.addr STARTUP

OSHALT = $FFC0

; Standard startup code
.code
STARTUP:
    jsr copydata
    jsr MAIN
    jmp OSHALT
