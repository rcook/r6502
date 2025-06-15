.export DSP
.export DSPCR
.export HALT

KBD = $D010     ; PIA.A keyboard input
KBDCR = $D011   ; PIA.A keyboard control register
DSP = $D012     ; PIA.B display output register
DSPCR = $D013   ; PIA.B display control register
HALT = $D014    ; JMP to this address to halt execution
