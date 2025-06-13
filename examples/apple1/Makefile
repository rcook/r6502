.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := sim65
PROGRAM := apple1.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean default run

SOURCES := main.s wozmon.s a1basic.s
CFG := $(PROGRAM:.$(BINEXT)=.cfg)
TARGET := -C $(CFG)
LIB :=

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(PROGRAM:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.$(BINEXT)=.map)

run: $(PROGRAM)
	cargo run -- run $(PROGRAM) --reset --cycles -m Apple1
