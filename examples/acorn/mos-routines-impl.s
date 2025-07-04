.importzp CR
.importzp LF

.segment "OSCLI"
.import CLIV
    jmp (CLIV)

.segment "OSBYTE"
.import BYTEV
    jmp (BYTEV)

.segment "OSWORD"
.import WORDV
    jmp (WORDV)

; Start OSASCI, OSNEWL, OSWRCH

.segment "OSASCI"
.proc osasci
    cmp #CR
    bne oswrch
.endproc
.assert * = osnewl, error, "OSNEWL must immediately follow OSASCI"

.proc osnewl
    lda #LF
    jsr oswrch
    lda #CR
.endproc
.assert * = oswrch, error, "OSWRCH must immediately follow OSNEWL"

.import WRCHV
.proc oswrch
    jmp (WRCHV)
.endproc

.segment "OSNEWL"
.segment "OSWRCH"

; End of OSASCI, OSNEWL, OSWRCH

.segment "OSRDCH"
.import RDCHV
    jmp (RDCHV)

.segment "OSFILE"
.import FILEV
    jmp (FILEV)

.segment "OSARGS"
.import ARGSV
    jmp (ARGSV)

.segment "OSBGET"
.import BGETV
    jmp (BGETV)

.segment "OSBPUT"
.import BPUTV
    jmp (BPUTV)

.segment "OSGBPB"
.import GBPBV
    jmp (GBPBV)

.segment "OSFIND"
.import FINDV
    jmp (FINDV)
