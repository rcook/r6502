.macpack r6502
.macpack util
.import copydata
.import print
.import __CODE_LOAD__
.exportzp zword0

r6502_header "ACRN", __CODE_LOAD__, startup

.code
.proc startup
    sysinit
    jsr copydata
    jsr test_div16
    syshalt
.endproc

.proc test_div16
        print_buf welcome
        ; Test 16-bit division
        ; Reference: https://www.llx.com/Neil/a2/mult.html
        ; word0 = $1235 (4661) (dividend)
        ; word1 = $000a (10) (divisor)
        ; Result should be:
        ; word0 = 0x01d2 (466) (quotient)
        ; word2 = 0x0001 (1) (remainder)
        ;
        ; Total cycle count = 958 (920 for div16 and JSR + 38 for checks)
        jsr     div16

        ; Quotient stored in word0
@check_quotient_lo:
        lda     word0      ; Load low byte of quotient
        cmp     #$d2            ; Must be $d2
        beq     @check_quotient_hi
        print_buf failed
        lda     #$01            ; Failure
        rts
@check_quotient_hi:
        lda     word0 + 1  ; Low high byte of quotient
        cmp     #$01            ; Must be $01
        beq     @check_remainder_lo
        print_buf failed
        lda     #$02            ; Failure
        rts

        ; Remainder stored in word2
@check_remainder_lo:
        lda     word2      ; Load low byte of remainder
        cmp     #$01            ; Must be $01
        beq     @check_remainder_hi
        print_buf failed
        lda     #$03            ; Failure
        rts
@check_remainder_hi:
        lda     word2 + 1  ; Low high byte of remainder
        cmp     #$00            ; Must be $01
        beq     @done
        print_buf failed
        lda     #$04            ; Failure
        rts

@done:
        print_buf succeeded
        lda     #$00            ; Success
        rts
.endproc

.proc div16
        lda     #0
        sta     word2
        sta     word2 + 1
        ldx     #16
@l1:
        asl     word0
        rol     word0 + 1
        rol     word2
        rol     word2 + 1
        lda     word2
        sec
        sbc     word1
        tay
        lda     word2 + 1
        sbc     word1 + 1
        bcc     @l2
        sta     word2 + 1
        sty     word2
        inc     word0
@l2:
        dex
        bne     @l1
        rts
.endproc

.zeropage
zword0: .word $0000

.data
welcome: .byte "DIV16 TEST", 13, 10, 0
succeeded: .byte "Test passed", 13, 10, 0
failed: .byte "Test failed", 13, 10, 0
word0: .word $1235
word1: .word $000a
word2: .word $ffff
