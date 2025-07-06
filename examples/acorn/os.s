; Debugging and host communication hooks
.export HALT = $FFA0
.export HOSTHOOK = $FFA2
.exportzp CLIVHOSTHOOK = 100
.exportzp FILEVHOSTHOOK = 101

.export LANGUAGE_ROM_START = $8000
.export LANGUAGE_ROM_TITLE = $8009
