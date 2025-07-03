.macpack r6502
.import HALT
.import OSWRCH
.import __SIDEWAYSCODE_LOAD__

r6502_header "ACRN", __SIDEWAYSCODE_LOAD__, startup

.segment "SIDEWAYSCODE"
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr main
    jmp HALT
.endproc

.proc main
    jsr test_oswrch
    lda #$00
    rts
.endproc

.proc test_oswrch
    ldx #0
@loop:
    lda str, x
    beq @done
    jsr OSWRCH
    inx
    bne @loop
@done:
    rts
.endproc

.segment "SIDEWAYSDATA"
str: .byte "Testing OSWRCH", 13, 10, 0
