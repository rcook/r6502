.import __DATA_LOAD__
.segment "HEADER"
.dbyt $6502
.byte $00
.byte "ACRN"
.addr __DATA_LOAD__
.addr startup
