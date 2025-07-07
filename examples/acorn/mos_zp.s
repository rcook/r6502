; Acorn MOS zero-page workspace
; https://mdfs.net/Docs/Comp/BBC/AllMem
.exportzp OSAREG = $EF
.exportzp OSXREG = $F0
.exportzp OSYREG = $F1
.exportzp OSINTA = $FC
.exportzp OSFAULT = $FD
.exportzp OSESC = $FF
