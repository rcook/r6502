.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := sim65
PROGRAM := strings.$(BINEXT)
default: $(PROGRAM)

.PHONY: clean default run

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
	@#sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
	cargo run -- run $(PROGRAM) -m Acorn
