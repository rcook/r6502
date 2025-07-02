.macpack util
.exportzp ptr

.code
.export MAIN
.proc MAIN
    print_imm hello
    print_imm lorem_ipsum
    print_imm goodbye

    lda #$00
    ldx #<value
    ldy #>value
    ora #%10000000
    jsr num_to_str
    stx ptr
    sty ptr + 1
    jsr print
    print_imm line_break

    rts
.endproc

.zeropage
ptr: .word 0

.rodata
line_break: .byte 13, 10, 0
value: .dword 12345678
hello: .byte "Hello", 13, 10, 0
goodbye: .byte "Goodbye", 13, 10, 0
lorem_ipsum: .byte "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", 13, 10, 0
lorem_ipsum_end:
str: .res 512

.data
temp: .byte "TEMP", 13, 10, 0
