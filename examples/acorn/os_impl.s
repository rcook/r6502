.macpack raw
.macpack r6502
.import BRKV
.import IRQ1V
.importzp OSESC
.importzp OSFAULT
.importzp OSINTA
.importzp OSKBD1
.importzp OSKBD2
.importzp OSXREG
.import STACKBASE

.importzp ESC

.import KBD
.import KBDCR

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
    jsr keyboard_interrupt
    lda OSINTA
    rti
.endproc

; Inspired by https://github.com/chelsea6502/BeebEater/blob/main/BeebEater.asm
.proc keyboard_interrupt
    lda KBD
    cmp #ESC
    sta OSKBD1
    bne @keyboard_interrupt_done
    lda #$00
    sta OSKBD2
    lda #$FF
    sta OSESC
@keyboard_interrupt_done:
    rts
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
