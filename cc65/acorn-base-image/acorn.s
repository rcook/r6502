.export STARTUP

; r6502 executable header
.segment "HEADER"
.byte $65
.byte $02
.addr $8000
.addr RESET

; Standard startup code
.code
STARTUP:
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr MAIN
    jmp OSHALT
