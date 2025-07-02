.export print

.importzp zword0

; print
; Prints zero-terminated string
; params:
;   zword0, zword0 + 1: address of string
; comments:
;   Destroys P, A, Y, zword0, zword0 + 1
.code
.proc print
    ldy #$00
@loop:
    lda (zword0), y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
    inc zword0 + 1
    lda #0
    beq @loop
@done:
    rts
.endproc
