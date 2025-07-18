;.macpack raw
;.macpack r6502

.importzp ESC
.importzp OSESC
.importzp OSFAULT
.importzp OSINTA
.importzp OSKBD1
.importzp OSKBD2

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
    sta OSKBD1
    cmp #ESC
    bne @not_esc
    lda #$FF
    sta OSESC
    rts
@not_esc:
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
