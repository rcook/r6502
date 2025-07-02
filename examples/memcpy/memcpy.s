.macpack util
.exportzp ptr

.code
.export MAIN
.proc MAIN
    print_const hello
    print_const lorem_ipsum

    print_int value
    print_const line_break

    add16 value_a, value_b, result
    lda #0
    sta result + 2
    sta result + 3

    ; There appears to be a bug in num_to_str...
    print_int result
    print_const line_break

    print_const goodbye
    rts
.endproc

.zeropage
ptr: .word 0
result: .dword $FFFFFFFF

.rodata
value_a: .word 25
value_b: .word 35
line_break: .byte 13, 10, 0
value: .dword 12345678
hello: .byte "Hello", 13, 10, 0
goodbye: .byte "Goodbye", 13, 10, 0
lorem_ipsum: .byte "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", 13, 10, 0
lorem_ipsum_end:
str: .res 512

.data
temp: .byte "TEMP", 13, 10, 0
