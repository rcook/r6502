.import BRKV
.importzp OSFAULT
.importzp OSINTA
.importzp OSXREG
.import STACKBASE

.segment "NMI"
    .addr interrupt

.segment "RESET"
    .addr $0000

.segment "IRQ"
    .addr interrupt

.segment "HALT"
.proc HALT
    brk
    nop
    rts
.endproc

.segment "ROCODE"
.proc interrupt
    ; For the time being, we'll just handle BRK: for everything else,
    ; we'll halt the system
    ; https://github.com/chelsea6502/BeebEater/blob/main/BeebEater.asm
    sta OSINTA          ; Save A for later
    pla                 ; Get the status register: IRQ/BRK puts it on the stack
    pha                 ; Keep the status register on the stack for later
    and #$10            ; Check if it's a BRK or an IRQ
    bne handle_brk      ; If it's BRK, that's an error: go to the BRK vector
    jmp HALT            ; Otherwise, halt the CPU for time being
.endproc

; https://github.com/chelsea6502/BeebEater/blob/main/BeebEater.asm
; Handler for interrupts that we know were called by the BRK instruction.
; This means an error was reported. The BBC MOS API defines the structure
; of an error message. To get the message, we need to store the location
; of the error message in addresses $FD and $FE.
.proc handle_brk
    txa
    pha                     ; Save X
    tsx                     ; Get the stack pointer value
    lda STACKBASE + 3, x    ; Get the low byte of the error message location, offset by the stack pointer
    sec
    sbc #$01                ; Subtract one since BRK stores BRK + 2 to the stack by default, rather than the BRK + 1 we need
    sta OSFAULT             ; Store the low byte into the fault handler
    lda STACKBASE + 4, x    ; Get the high byte of the error message location
    sbc #$00                ; Did subtracting 1 from the low byte cause the carry bit to set? Subtract 1 from the high byte too
    sta OSFAULT + 1         ; Store the high byte into the fault handler
    stx OSXREG              ; Store the location of the last break for the error handler
    pla
    txa                     ; Restore X
    lda OSINTA
    cli
    jmp (BRKV)              ; Jump to BBC BASIC's error handler routine, which takes it from there
.endproc
