.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := r6502
PROGRAM := strings.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean debug default run

SOURCES := strings.s
CFG := $(PROGRAM:.$(BINEXT)=.cfg)
TARGET := -C $(PROGRAM:.$(BINEXT)=.cfg)
LIB :=

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(PROGRAM:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.$(BINEXT)=.map)

run: $(PROGRAM)
	cargo run -- run $(PROGRAM)

debug: $(PROGRAM)
	cargo run -- debug $(PROGRAM)
