
.segment "OSWRCH"
.export OSWRCH
OSWRCH:
    ora #$80            ; Set high bit
oswrch_loop:
    bit $fc02
    bmi oswrch_loop
    sta $fc02
    rts
