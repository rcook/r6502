.export STARTUP

; sim65 executable header
.segment "HEADER"
.byte "sim65"
.byte 2
.byte 0
.byte $FF
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
