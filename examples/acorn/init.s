.import HALT
.import SIDEWAYS_ROM_OFFSET
.import SIDEWAYS_ROM_START
.import SIDEWAYS_ROM_TITLE
.importzp OSAREG
.import OSNEWL
.import OSWRCH

.macro init_vector vec, entrypoint
    .import vec
    .import entrypoint
    lda #<entrypoint
    sta vec
    lda #>entrypoint
    sta vec + 1
.endmacro

.macro init_vectors
    init_vector USERV, userv_entrypoint
    init_vector BRKV, brkv_entrypoint
    init_vector IRQ1V, irq1v_entrypoint
    init_vector IRQ2V, irq2v_entrypoint
    init_vector CLIV, cliv_entrypoint
    init_vector BYTEV, bytev_entrypoint
    init_vector WORDV, wordv_entrypoint
    init_vector WRCHV, wrchv_entrypoint
    init_vector RDCHV, rdchv_entrypoint
    init_vector FILEV, filev_entrypoint
    init_vector ARGSV, argsv_entrypoint
    init_vector BGETV, bgetv_entrypoint
    init_vector BPUTV, bputv_entrypoint
    init_vector GBPBV, gbpbv_entrypoint
    init_vector FINDV, findv_entrypoint
    init_vector FSCV, fscv_entrypoint
    init_vector EVNTV, evntv_entrypoint
    init_vector UPTV, uptv_entrypoint
    init_vector NETV, netv_entrypoint
    init_vector VDUV, vduv_entrypoint
    init_vector KEYV, keyv_entrypoint
    init_vector INSV, insv_entrypoint
    init_vector REMV, remv_entrypoint
    init_vector CNPV, cnpv_entrypoint
    init_vector IND1V, ind1v_entrypoint
    init_vector IND2V, ind2v_entrypoint
    init_vector IND3V, ind3v_entrypoint
.endmacro

.segment "MOSINIT"
.export MOS_INIT
.proc MOS_INIT
    init_vectors

    ; Display startup banner
    lda #<mos_banner
    sta OSAREG
    lda #>mos_banner
    sta OSAREG + 1
    jsr print_banner
    jsr OSNEWL
    jsr OSNEWL

    ; Annoying beep
    lda #$07
    jsr OSWRCH

    ; Check that we have a language ROM
    lda #<SIDEWAYS_ROM_START
    sta OSAREG
    lda #>SIDEWAYS_ROM_START
    sta OSAREG + 1
    ldy #$00
    lda (OSAREG), y
    cmp #$4C ; opcode for JMP direct (well-formed language ROMS start with this)
    beq @continue
    cmp #$C9 ; opcode for CMP immediate (BBC BASIC 2 starts with this)
    beq @continue

@no_language:
    lda #<no_language
    sta OSAREG
    lda #>no_language
    sta OSAREG + 1
    jsr print_banner
    lda #$01
    jmp HALT

@continue:
    ; Check copyright is valid
    lda #<SIDEWAYS_ROM_START
    clc
    adc SIDEWAYS_ROM_OFFSET
    sta OSAREG
    lda #>SIDEWAYS_ROM_START
    sta OSAREG + 1

    ldy #$00
    lda (OSAREG), y
    bne @no_language

    iny
    lda (OSAREG), y
    cmp #'('
    bne @no_language

    iny
    lda (OSAREG), y
    cmp #'C'
    bne @no_language

    iny
    lda (OSAREG), y
    cmp #')'
    bne @no_language

    ; Display sideways ROM title
    lda #<SIDEWAYS_ROM_TITLE
    sta OSAREG
    lda #>SIDEWAYS_ROM_TITLE
    sta OSAREG + 1
    jsr print_banner
    lda #' '
    jsr OSWRCH

    ; Display sideways ROM copyright
    lda #<SIDEWAYS_ROM_START
    clc
    adc SIDEWAYS_ROM_OFFSET
    adc #$01
    sta OSAREG
    lda #>SIDEWAYS_ROM_START
    sta OSAREG + 1
    jsr print_banner
    jsr OSNEWL

    ; Initialize stack pointer and set binary mode
    ldx #$ff
    txs
    cld

    ; Jump to language entrypoint $8000 with A=1
    lda #$01
    jmp SIDEWAYS_ROM_START
.endproc

.proc print_banner
    ldy #$00
@banner_loop:
    lda (OSAREG), y
    beq @banner_loop_done
    jsr OSWRCH
    iny
    bne @banner_loop
@banner_loop_done:
    rts
.endproc

.segment "MOSDATA"
mos_banner: .byte "r6502 Emulator 32K", 0
no_language: .byte "Language?", 0
