include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

BINSOURCES := acorn.s header.s main.s memcpy.s print.s
BIN := memcpy.$(BINEXT)
BINCFG := $(BIN:.$(BINEXT)=.cfg)
BINMAP := $(BIN:.$(BINEXT)=.map)
BINTARGETLIB := $(SHAREDLIBDIR)/std.lib

ALLSOURCES = $(BINSOURCES)

%.o: %.s
	ca65 --include-dir $(SHAREDLIBDIR) -U -l $(@:o=lst) -o $@ $<

BINLIBS = $(BINTARGETLIB)
$(BIN): $(BINSOURCES:.s=.o) $(BINCFG) $(BINLIBS)
	ld65 -C $(BINCFG) -vm -m $(BINMAP) -o $@ $(filter-out $(BINLIBS),$(filter-out $(BINCFG),$^)) $(patsubst %,--lib %, $(BINLIBS))

clean:
	$(RM) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(BIN) $(BINMAP)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
