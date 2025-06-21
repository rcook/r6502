.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BINEXT := r6502
BIN := num-to-str.$(BINEXT)
BINSOURCES := header.s num-to-str.s test.s
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
