oswrch = &FFEE
print_strs_arg = &80

ORG &2000         ; code origin (like P%=&2000)

.start
    LDA #LO(array)
    STA print_strs_arg
    LDA #HI(array)
    STA print_strs_arg + 1
    JSR print_strs
    RTS
.print_strs
    LDY #$00
    LDA (print_strs_arg), Y
    RTS
.str0
    EQUS "String0", 10, 0
.str1
    EQUS "String1", 10, 0
.array
    EQUB 2
    EQUW str0, str1
.end

SAVE "strings.bin", start, end
