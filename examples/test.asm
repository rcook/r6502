oswrch = &FFEE

ORG &2000         ; code origin (like P%=&2000)

.start
    LDX #0
.letter
    LDA message, X
    CMP #0
    BEQ finished
    JSR oswrch
    INX
    JMP letter  
.finished
    RTS
.message
    EQUS "Hello, world", 13, 10, 0
.end

SAVE "test.bin", start, end
