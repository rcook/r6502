; Eventually this will implement a tiny subset of Acorn MOS 1.00/1.20 functionality
; to support character I/O: so, most likely, just OSRDCH and OSWRCH
; Sources:
;   https://github.com/raybellis/mos120
;   https://mdfs.net/Docs/Comp/BBC/OS1-20/
;   https://tobylobster.github.io/mos/mos/index.html
;   https://www.pa3byb.nl/wp-content/uploads/2017/12/Advance_user_guide_Acorn_electron.pdf
;   https://archive.org/details/bbc-micro-advanced-user-guide/page/4/mode/2up

.macpack r6502
.import HALT
.import copydata
.import main
.import __OSRODATA_LOAD__

r6502_header "ACRN", __OSRODATA_LOAD__, startup

.code
.export startup
.proc startup
    ldx #$ff
    txs
    cld
    jsr copydata
    jsr main
    jmp HALT
.endproc
