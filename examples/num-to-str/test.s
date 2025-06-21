.exportzp MAX_STR_LEN = 33

ZPPTR = $80

.code
.export startup
startup:
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr test_binstr
    jmp OSHALT

.proc test_binstr
    lda #$00
    ldx #<value
    ldy #>value
    ora #%10000000
    jsr binstr
    sta result_str_len
    stx ZPPTR
    sty ZPPTR + 1
    tay
@loop:
    lda (ZPPTR),Y
    sta result_str,Y
    dey
    bpl @loop

.proc check_result_str
    ldx result_str_len
    cpx expected_str_len
    bne @failed
@loop:
    dex
    lda result_str,X
    cmp expected_str,X
    bne @failed
    cpx #$00
    bne @loop
@success:
    lda #<success_str
    sta ZPPTR
    lda #>success_str
    sta ZPPTR + 1
    jsr print_str
    lda #$00
    rts
@failed:
    lda #<failure_str
    sta ZPPTR
    lda #>failure_str
    sta ZPPTR + 1
    jsr print_str
    lda #$01
    rts
.endproc
.endproc

.proc print_str
    ldy #$00
@loop:
    lda (ZPPTR),Y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc

.data
value:
    .dword $12345678

result_str_len:
    .byte 0
result_str:
    .res MAX_STR_LEN

expected_str_len:
    .byte 9
expected_str:
    .byte "305419896", 0

success_str:
    .byte "binstr returned expected string", 13, 10, 0

failure_str:
    .byte "binstr did not return expected string", 13, 10, 0
