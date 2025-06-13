.org $0e00

result:
    .word $0000

start:
    CLC
    LDA left_operand
    ADC right_operand
    STA result
    LDA left_operand + 1
    ADC right_operand + 1
    STA result + 1
    BRK

left_operand:
    .word $3412

right_operand:
    .word $7856
