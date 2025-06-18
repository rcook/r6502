.feature string_escapes

.code
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr main
    lda #$00
    jmp OSEXIT
.endproc

.proc main
    ldx #<welcome
    ldy #>welcome
    jsr print_str

    ldx #$00
    stx value
    stx value + 1
    stx value + 2
    stx value + 3
    txa
    pha

@outer:
    stx value

    lda #$80
    ldx #<value
    ldy #>value
    jsr num_to_str
    stx zp_address
    sty zp_address + 1
    tay
@loop:
    lda (zp_address),Y
    sta str,Y
    dey
    bpl @loop

    ldx #<str
    ldy #>str
    jsr print_str
    lda #10
    jsr OSWRCH

    pla
    tax
    inx
    cmp #127
    beq @done
    txa
    pha
    clc
    bcc @outer

@done:
    rts
.endproc

.zeropage
zp_address:
    .addr $0000

.data
str:
    .res 10
value:
    .dword 0

.rodata
welcome:
    .asciiz "Welcome to my program!\n"
