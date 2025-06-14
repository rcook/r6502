.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := r6502
PROGRAM := hello-world.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean default run

SOURCES := hello-world.s
CFG := $(PROGRAM:.$(BINEXT)=.cfg)
TARGET := -C $(CFG)
LIB := none.lib

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(PROGRAM:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.$(BINEXT)=.map)

run: $(PROGRAM)
	cargo run -- run $(PROGRAM) --cycles
