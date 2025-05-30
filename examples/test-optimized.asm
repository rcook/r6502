.alias oswrch $FFEE

.org $2000

start:
    LDX #0
letter:
    LDA message, X
    BEQ finished
    JSR oswrch
    INX
    JMP letter
finished:
    RTS
message:
    .byte "Hello, world", 13, 10, 0
