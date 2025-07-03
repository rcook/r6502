.macpack util
.import memcpy
.import num_to_str
.import print
.exportzp zword0, zword1, zword2

.segment "SIDEWAYSCODE"
.export main
.proc main
    print_buf hello
    print_int value
    print_buf line_break

    ; Demonstrate memcpy
    print_buf lorem_ipsum
    stzword0 lorem_ipsum
    stzword1 str
    stzword2 (lorem_ipsum_end - lorem_ipsum)
    jsr memcpy
    print_buf str

    print_buf goodbye
    rts
.endproc

.zeropage
zword0: .word $0000
zword1: .word $0000
zword2: .word $0000

.data
result: .dword $FFFFFFFF
str: .res 1024
str_end: .byte 0

.segment "SIDEWAYSDATA"
value_a: .word 25
value_b: .word 35
line_break: .byte 13, 10, 0
value: .dword 12345678
hello: .byte "Hello", 13, 10, 0
goodbye: .byte "Goodbye", 13, 10, 0
lorem_ipsum: .byte "abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz abcdefghijklmnopqrstuvwxyz Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", 13, 10, 0
lorem_ipsum_end:
