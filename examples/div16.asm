.org $0e00

; 16-bit division: dividend / divisor = quotient
; Uses shift and subtract method (similar to long division)
; Input:  dividend_lo, dividend_hi, divisor_lo, divisor_hi
; Output: quotient_lo, quotient_hi, remainder_lo, remainder_hi

quotient_lo:
    .byte $00
quotient_hi:
    .byte $00
remainder_lo:
    .byte $00
remainder_hi:
    .byte $00

start:
    ; Initialize remainder to 0
    LDA #$00
    STA remainder_lo
    STA remainder_hi
    STA quotient_lo
    STA quotient_hi

    ; Set up counter for 16 bits
    LDX #16
div_loop:
    ; Shift dividend left, shifting in 0
    ASL dividend_lo
    ROL dividend_hi

    ; Shift remainder left, shifting in bit from dividend
    ROL remainder_lo
    ROL remainder_hi

    ; Try to subtract divisor from remainder
    SEC
    LDA remainder_lo
    SBC divisor_lo
    TAY         ; Save low byte of subtraction result
    LDA remainder_hi
    SBC divisor_hi
    BCC skip_subtract  ; If carry clear, subtraction would be negative

    ; If we get here, subtraction was successful
    ; Store new remainder
    STY remainder_lo
    STA remainder_hi

    ; Set quotient bit to 1
    LDA quotient_lo
    ORA #$01
    STA quotient_lo

skip_subtract:
    ; Shift quotient left
    ASL quotient_lo
    ROL quotient_hi

    DEX
    BNE div_loop

    RTS

; Test values
dividend_lo:
    .byte $34    ; Low byte of dividend (1234)
dividend_hi:
    .byte $12    ; High byte of dividend
divisor_lo:
    .byte $0A    ; Low byte of divisor (10)
divisor_hi:
    .byte $00    ; High byte of divisor
