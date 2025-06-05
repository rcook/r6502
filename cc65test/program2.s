.export _main

_main:
    ; do your test
    ; return from main with error code in A
    lda #0
    rts

.import exit
    ; instead of returning from main, we can use exit to return the error code.
    lda #0
    jmp exit
