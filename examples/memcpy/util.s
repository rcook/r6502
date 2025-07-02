.export print, memcpy

.importzp zptr0, zptr1

; print_impl
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


; memcpy
; Copy block of memory (must not overlap)
; params:
;   zptr0, zptr0 + 1: address of source buffer
;   zptr1, zptr1 + 1: address of target buffer
;   zptr2, zptr2 + 1: number of bytes to copy
; comments:
;   Destroys P, A, Y, zptr0 + 1, zptr1 + 1
.proc memcpy
@copy_whole_pages:
    ldx zptr2 + 1
    beq @copy_partial_page
    ldy #0
@loop1:
    lda (zptr0), y
    sta (zptr1), y
    iny
    bne @loop1
    dex
    beq @loop1_done
    txa
    inc zptr0 + 1
    inc zptr1 + 1
    tax
    bne @loop1
@loop1_done:
    inc zptr0 + 1
    inc zptr1 + 1

@copy_partial_page:
    ldy #$FF
@loop2:
    iny
    cpy zptr2
    beq @loop2_done
    lda (zptr0), y
    sta (zptr1), y
    lda #0
    beq @loop2
@loop2_done:
    rts
.endproc
