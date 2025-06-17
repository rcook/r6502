.import __DATA_LOAD__
.segment "HEADER"
.dbyt $6502
.byte $00
.byte "ACRN"
.addr __DATA_LOAD__
.addr startup

.feature string_escapes

EXIT = $FFC0
OSWRCH = $FFEE

.code
startup:
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr hello_world
    jmp EXIT

hello_world:
    ldx #$00
hello_world_loop:
    lda hello_world_string, X
    beq hello_world_done
    jsr OSWRCH
    inx
    bne hello_world_loop
hello_world_done:
    rts

.data
hello_world_string:
    .asciiz "HELLO, WORLD!\n"
