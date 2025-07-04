.macpack r6502
.macpack util
.import OSWRCH
.import copydata
.import print
.import num_to_str
.exportzp zword0

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    sysinit
    jsr copydata
    jsr main
    lda #$00
    syshalt
.endproc

.proc main
    print_buf welcome
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
    stx zptr
    sty zptr + 1
    tay
@loop:
    lda (zptr),Y
    sta str,Y
    dey
    bpl @loop

    print_buf str
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
zword0: .word $0000
zptr: .addr $0000

.data
str: .res $80
value: .dword 0

.segment "SIDEWAYSDATA"
welcome: .byte "Welcome to my program!", 13, 10, 0
