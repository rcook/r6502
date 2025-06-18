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

    jsr count_up
    jsr dword_inc

    rts
.endproc

.proc count_up
    ldx #$00
    stx value
    stx value + 1
    stx value + 2
    stx value + 3
    txa
    pha

@outer:
    stx value

    jsr print_value

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

.proc print_value
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
    rts
.endproc

.proc dword_inc
    ldx #$ff
    stx value
    stx value + 1
    stx value + 2
    stx value + 3

    jsr print_value

    ldx value
    inx
    stx value
    bne @skip

    ldx value + 1
    inx
    stx value + 1
    bne @skip

    ldx value + 2
    inx
    stx value + 2
    bne @skip

    ldx value + 3
    inx
    stx value + 3
    bne @skip

    ldx #$00
    stx value
    stx value + 1
    stx value + 2
    stx value + 3

@skip:
    jsr print_value
    rts
.endproc

.zeropage
zp_address:
    .addr $0000

.data
str:
    .res $80
value:
    .dword 0

.rodata
welcome:
    .asciiz "Welcome to my program!\n"
