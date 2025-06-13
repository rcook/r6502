.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := sim65
PROGRAM := simple.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean default run

SOURCES := constants.s main.s simple.s vectors.s
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
	cargo run -p r6502 run $(PROGRAM) --reset --cycles -m ACRN --trace
