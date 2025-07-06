.macpack r6502
.import __SIDEWAYSCODE_LOAD__

r6502_system "ACRN", __SIDEWAYSCODE_LOAD__

.segment "SIDEWAYSCODE"
.incbin "bbc-basic-2.rom"

.segment "SIDEWAYSDATA"
