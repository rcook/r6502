.macpack util
.import print
.exportzp zword0

.segment "SIDEWAYSCODE"
.export main
.proc main
    print_buf message
    brk
.endproc

.zeropage
zword0: .word $0000

.segment "SIDEWAYSDATA"
message: .byte "One break coming up...", 13, 10, 0
