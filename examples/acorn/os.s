; Debugging and host communication hooks
.export HALT = $FFA0
.export HOSTHOOK = $FFA2
.exportzp CLIVHOSTHOOK = 100
.exportzp FILEVHOSTHOOK = 101

.export SIDEWAYS_ROM_START = $8000
.export SIDEWAYS_ROM_OFFSET = $8007
.export SIDEWAYS_ROM_TITLE = $8009
