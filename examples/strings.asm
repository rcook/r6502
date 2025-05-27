.alias oswrch $FFEE
.alias print_strs_arg $80
.alias print_str_arg $82

.org $0e00

start:
    LDA #<array
    STA print_strs_arg
    LDA #>array
    STA print_strs_arg + 1
    JSR print_strs
    RTS
print_strs:
    LDY #$00
    LDA (print_strs_arg), Y
    TAX                             ; X tracks number of elements remaining in array
print_strs_loop:
    CPX #$00
    BEQ print_strs_end
    INY
    LDA (print_strs_arg), Y
    STA print_str_arg
    INY
    LDA (print_strs_arg), Y
    STA print_str_arg + 1
    TYA
    PHA
    JSR print_str
    PLA
    TAY
    DEX
    JMP print_strs_loop
print_strs_end:
    RTS
print_str:
    LDY #$00
print_str_loop:
    LDA (print_str_arg), Y
    CMP #$00
    BEQ print_str_end
    JSR oswrch
    INY
    JMP print_str_loop
print_str_end:
    RTS
str0:
    .byte "String0", 10, 0
str1:
    .byte "String1", 10, 0
array:
    .byte 2
    .word str0, str1
