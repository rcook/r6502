.macpack r6502
.import __SIDEWAYSHEADER_LOAD__

r6502_system "ACRN", __SIDEWAYSHEADER_LOAD__

.segment "SIDEWAYSHEADER"
.incbin "bbc-basic-2.rom"

.segment "SIDEWAYSCODE"

.segment "SIDEWAYSDATA"
