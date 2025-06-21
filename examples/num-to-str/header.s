; r6502 executable header
.segment "HEADER"
.dbyt $6502
.byte $00
.byte "ACRN"
.import __DATA_LOAD__
.addr __DATA_LOAD__
.addr startup

.export OSHALT = $FFC0
.export OSWRCH = $FFEE
