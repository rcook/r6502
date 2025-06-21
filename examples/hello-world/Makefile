.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BINEXT := r6502
BIN := hello-world.$(BINEXT)
BINSOURCES := acorn.s header.s hello-world.s
CFG := $(BIN:.$(BINEXT)=.cfg)
TARGET := -C $(CFG)
LIB := none.lib

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(BIN): $(BINSOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(BIN:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(BIN) $(BIN:.$(BINEXT)=.map)

run: $(BIN)
	cargo run -- run $(BIN) --trace

default: $(BIN)
