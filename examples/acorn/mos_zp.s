; Acorn MOS zero-page workspace
; https://mdfs.net/Docs/Comp/BBC/AllMem
; https://tobylobster.github.io/mos/os120_acme.a
.exportzp OSVDU  = $D0      ; a.k.a. .vduStatusByte
.exportzp OSKBD1 = $EC      ; a.k.a. .keyPressedInternalTable/.lastKeyPressedInternal
.exportzp OSKBD2 = $ED      ; a.k.a. .firstKeyPressedInternal
.exportzp OSKBD3 = $EE      ; a.k.a. .keyToIgnoreWhenScanningWithOSBYTE121or122
.exportzp OSAREG = $EF      ; a.k.a. .osbyteA/.oswordA
.exportzp OSXREG = $F0      ; a.k.a. .osbyteX/.oswordX/.stackPointerLastBRK
.exportzp OSYREG = $F1      ; a.k.a. .osbyteY/.oswordY
.exportzp OSINTA = $FC      ; a.k.a. .interruptAccumulator
.exportzp OSFAULT = $FD     ; a.k.a. .brkAddressLow
.exportzp OSESC = $FF       ; a.k.a. .escapeFlag
