.export memcpy

.importzp zword0, zword1, zword2

; memcpy
; Copy block of memory (must not overlap)
; params:
;   zword0, zword0 + 1: address of source buffer
;   zword1, zword1 + 1: address of target buffer
;   zword2, zword2 + 1: number of bytes to copy
; comments:
;   Destroys P, A, Y, zword0 + 1, zword1 + 1
.proc memcpy
@copy_whole_pages:
    ldx zword2 + 1
    beq @copy_partial_page
    ldy #0
@loop1:
    lda (zword0), y
    sta (zword1), y
    iny
    bne @loop1
    dex
    beq @loop1_done
    txa
    inc zword0 + 1
    inc zword1 + 1
    tax
    bne @loop1
@loop1_done:
    inc zword0 + 1
    inc zword1 + 1

@copy_partial_page:
    ldy #$FF
@loop2:
    iny
    cpy zword2
    beq @loop2_done
    lda (zword0), y
    sta (zword1), y
    lda #0
    beq @loop2
@loop2_done:
    rts
.endproc
