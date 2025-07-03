.import exit

        .segment "EXEHDR"
        .export __EXEHDR__
__EXEHDR__:
        .byte   "sim65"	; magic number
        .byte   2	; simulator version: 2 = current
        .byte   0	; CPU version: 0 = 6502, 1 = 65c02
        .byte   $FF	; initial SP
        .addr   _main   ; load address
        .addr   _main   ; start address (these are the same if _main is first in STARTUP)

        .segment "STARTUP"
        .export _main
.org $1000
_main:
        ; Test 16-bit division
        ; Reference: https://www.llx.com/Neil/a2/mult.html
        ; WORD_REG_0 = $1235 (4661) (dividend)
        ; WORD_REG_1 = $000a (10) (divisor)
        ; Result should be:
        ; WORD_REG_0 = 0x01d2 (466) (quotient)
        ; WORD_REG_2 = 0x0001 (1) (remainder)
        ;
        ; Total cycle count = 958 (920 for div16 and JSR + 38 for checks)
        jsr     div16

        ; Quotient stored in WORD_REG_0
check_quotient_lo:
        lda     WORD_REG_0      ; Load low byte of quotient
        cmp     #$d2            ; Must be $d2
        beq     check_quotient_hi
        lda     #$01            ; Failure
        jmp     exit
check_quotient_hi:
        lda     WORD_REG_0 + 1  ; Low high byte of quotient
        cmp     #$01            ; Must be $01
        beq     check_remainder_lo
        lda     #$02            ; Failure
        jmp     exit

        ; Remainder stored in WORD_REG_2
check_remainder_lo:
        lda     WORD_REG_2      ; Load low byte of remainder
        cmp     #$01            ; Must be $01
        beq     check_remainder_hi
        lda     #$03            ; Failure
        jmp     exit
check_remainder_hi:
        lda     WORD_REG_2 + 1  ; Low high byte of remainder
        cmp     #$00            ; Must be $01
        beq     done
        lda     #$04            ; Failure
        jmp     exit

done:
        lda     #$00            ; Success
        jmp     exit

div16:
        lda     #0
        sta     WORD_REG_2
        sta     WORD_REG_2 + 1
        ldx     #16
l1:
        asl     WORD_REG_0
        rol     WORD_REG_0 + 1
        rol     WORD_REG_2
        rol     WORD_REG_2 + 1
        lda     WORD_REG_2
        sec
        sbc     WORD_REG_1
        tay
        lda     WORD_REG_2 + 1
        sbc     WORD_REG_1 + 1
        bcc     l2
        sta     WORD_REG_2 + 1
        sty     WORD_REG_2
        inc     WORD_REG_0
l2:
        dex
        bne     l1
        rts

        .segment "RODATA"
        .export status
status:
        .byte   25
WORD_REG_0:
        .word   $1235
WORD_REG_1:
        .word   $000a
WORD_REG_2:
        .word   $ffff
