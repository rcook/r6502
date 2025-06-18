.zeropage
print_str_ptr:
    .res 2

.code
.export print_str
.proc print_str
    ; Arguments:
    ;   X = LSB of address of string
    ;   Y = MSB of address of string
    ; On return:
    ;   A destroyed
    ;   Y destroyed
    ;   P destroyed
    stx print_str_ptr
    sty print_str_ptr + 1
    ldy #$00
@loop:
    lda (print_str_ptr),y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc
