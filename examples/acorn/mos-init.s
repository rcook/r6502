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
.proc MOSINIT
    init_vectors
    rts
.endproc
