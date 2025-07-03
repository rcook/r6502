.macpack r6502
.import startup
.import __DATA_LOAD__

r6502_header "ACRN", __DATA_LOAD__, startup

.export HALT = $FFC0
.export OSWRCH = $FFEE
