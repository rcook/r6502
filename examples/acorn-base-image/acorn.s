.export STARTUP

; r6502 executable header
.segment "HEADER"
.dbyt $6502
.byte "ACRN"
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
