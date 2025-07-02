include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

BINEXT := r6502
BINSOURCES := acorn.s header.s memcpy.s util.s
BIN := memcpy.$(BINEXT)
BINCFG := $(BIN:.$(BINEXT)=.cfg)
BINMAP := $(BIN:.$(BINEXT)=.map)
BINTARGETLIB := $(SHAREDLIBDIR)/std.lib

%.o: %.s
	ca65 --include-dir ../lib -U -l $(@:o=lst) -o $@ $<

BINLIBS = $(BINTARGETLIB)
$(BIN): $(BINSOURCES:.s=.o) $(BINCFG) $(BINLIBS)
	ld65 -C $(BINCFG) -m $(BINMAP) -o $@ $(filter-out $(BINLIBS),$(filter-out $(BINCFG),$^)) $(patsubst %,--lib %, $(BINLIBS))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(BIN) $(BINMAP)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
