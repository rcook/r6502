.import LANGUAGE_ROM_TITLE
.import LANGUAGE_ROM_START
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

    ; Annoying beep
    lda #$07
    jsr OSWRCH

    ; Display name of language ROM
    lda #<LANGUAGE_ROM_TITLE
    sta OSAREG
    lda #>LANGUAGE_ROM_TITLE
    sta OSAREG + 1
    jsr print_banner

    ; Initialize stack pointer and set binary mode
    ldx #$ff
    txs
    cld

    ; Jump to language entrypoint $8000 with A=1
    lda #$01
    jmp LANGUAGE_ROM_START
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
    jsr OSNEWL
    jsr OSNEWL
    rts
.endproc

.segment "MOSDATA"
mos_banner: .byte "r6502 Emulator 32K", 0
