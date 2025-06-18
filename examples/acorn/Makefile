.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean copy debug default run user

BINEXT := r6502
BIN := acorn.$(BINEXT)
BINSOURCES := header.s main.s os.s
CFG := $(BIN:.$(BINEXT)=.cfg)
TARGET := -C $(CFG)
LIB := none.lib

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(BIN): $(BINSOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(BIN:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(BIN) $(BIN:.$(BINEXT)=.map)

../../config/$(BIN): $(BIN)
	cp $(BIN) ../../config/$(BIN)

copy: ../../config/$(BIN)

run: copy
	cargo run -- run $(BIN) --trace

debug: copy
	cargo run -- debug $(BIN)

user: copy
	cargo run -- run ../hello-world/hello-world.r6502 --trace

default: copy
