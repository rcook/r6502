.macpack r6502
.import OSWRCH

.segment "SIDEWAYSCODE"
.incbin "bbc-basic-2.rom"

HEADER_SIZE = 11
ROM_SIZE = $4000
MOS0_START = $C000
MOS0_SIZE = $3000
MOS1_START = $FC04
MOS1_SIZE = $010000 - MOS1_START
VECTORS_SIZE = 6

; Patch in parts of OS image
.segment "MOS0"
.incbin "../../config/acorn.r6502", HEADER_SIZE, MOS0_SIZE

; Patch in parts of OS image
.segment "MOS1"
.incbin "../../config/acorn.r6502", HEADER_SIZE + ROM_SIZE - MOS1_SIZE, MOS1_SIZE - VECTORS_SIZE

.zeropage
zptr: .addr $0000

.segment "STARTUPCODE"
.export main
.proc main
    sysinit

    lda #<banner
    sta zptr
    lda #>banner
    sta zptr + 1
    ldy #$00
@banner_loop:
    lda (zptr), y
    beq @banner_loop_done
    jsr OSWRCH
    iny
    bne @banner_loop
@banner_loop_done:
    jsr $FFE7           ; OSNEWL
    jsr $FFE7           ; OSNEWL
    lda #$07
    jsr OSWRCH          ; BEL

    ; Jump to BBC BASIC entrypoint $8000 with A = 1
    lda #$01
    jmp $8000

    syshalt
.endproc

.segment "STARTUPDATA"
banner: .byte "r6502 Microcomputer 32K"
