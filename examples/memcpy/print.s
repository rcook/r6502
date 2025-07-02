.export print

.importzp zptr0

; print
; Prints zero-terminated string
; params:
;   zptr0, zptr0 + 1: address of string
; comments:
;   Destroys P, A, Y, zptr0, zptr0 + 1
.code
.proc print
    ldy #$00
@loop:
    lda (zptr0), y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
    inc zptr0 + 1
    lda #0
    beq @loop
@done:
    rts
.endproc
