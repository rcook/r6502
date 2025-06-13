.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := sim65
PROGRAM := div16.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean default run

SOURCES := div16.s
CFG :=
TARGET := -t sim6502
LIB := sim6502.lib

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(PROGRAM:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.$(BINEXT)=.map)

run: $(PROGRAM)
	sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
