.org $0e00

result:
    .byte $00

start:
    CLC
    LDA left_operand
    ADC right_operand
    STA result
    BRK

left_operand:
    .byte $12

right_operand:
    .byte $34
