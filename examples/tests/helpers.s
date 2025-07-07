.importzp OSAREG
.import OSWRCH

.zeropage
.exportzp za
za: .byte 0
.exportzp zx
zx: .byte 0
.exportzp zy
zy: .byte 0

.segment "SIDEWAYSCODE"
.export print_str
.proc print_str
    ldy #$00
@loop:
    lda (OSAREG),Y
    beq @done
    jsr OSWRCH
    iny
    bne @loop
@done:
    rts
.endproc
