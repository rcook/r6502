.import BRKV
.import IRQ1V
.importzp OSFAULT
.importzp OSINTA
.importzp OSXREG
.import STACKBASE

.segment "HALT"
.proc HALT
    brk
    nop
    rts
.endproc

; https://tobylobster.github.io/mos/os120_acme.a (.irqEntryPoint)
.segment "MOS"
.export MOS_IRQ_ENTRYPOINT
.proc MOS_IRQ_ENTRYPOINT
    sta OSINTA
    pla
    pha
    and #$10
    bne handle_brk
    jmp (IRQ1V)
.endproc

; https://tobylobster.github.io/mos/os120_acme.a (.brkRoutine)
.proc handle_brk
    txa
    pha
    tsx
    lda STACKBASE + 3, x
    cld
    sec
    sbc #$01
    sta OSFAULT
    lda STACKBASE + 4, x
    sbc #$00
    sta OSFAULT + 1
    ; lda .currentlySelectedROM
    ; sta .romNumberActiveLastBRK
    ; stx .stackPointerLastBRK
    ; ldx #.romServiceCallBreakInstruction
    ; jsr .osbyte143EntryPoint
    ; ldx .languageROMNumber
    ; jsr .selectROM
    pla
    txa
    lda OSINTA
    cli
    jmp (BRKV)
.endproc
