.export print

.importzp ptr

; print_impl
; Prints zero-terminated string
; params:
;   ptr, ptr + 1:   address of string
; returns:
;   (nothing)
; comments:
;   Destroys P, A, Y, ptr, ptr + 1
.code
.proc print
    ldy #$00
@loop:
    lda (ptr), y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
    inc ptr + 1
    lda #0
    beq @loop
@done:
    rts
.endproc
