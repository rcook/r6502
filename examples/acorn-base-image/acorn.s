; Eventually this will implement a tiny subset of Acorn MOS 1.00/1.20 functionality
; to support character I/O: so, most likely, just OSRDCH and OSWRCH
; Sources:
;   https://github.com/raybellis/mos120
;   https://mdfs.net/Docs/Comp/BBC/OS1-20/
;   https://tobylobster.github.io/mos/mos/index.html

.export STARTUP

; r6502 executable header
.segment "HEADER"
.dbyt $6502
.byte $00
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
