include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

BIN := brk.$(BINEXT)
BINSOURCES := header.s brk.s
BINCFG := $(BIN:.$(BINEXT)=.cfg)
BINMAP := $(BIN:.$(BINEXT)=.map)
BINTARGETLIB := $(CC65LIBDIR)/none.lib

ALLSOURCES := $(BINSOURCES)

prereqs:
	# None

%.o: %.s
	ca65 --include-dir $(SHAREDLIBDIR) -l $(@:o=lst) -o $@ $<

BINLIBS := $(BINTARGETLIB)
$(BIN): prereqs $(BINSOURCES:.s=.o) $(BINCFG)
	ld65 -C $(BINCFG) -vm -m $(BINMAP) -o $@ $(filter-out prereqs,$(filter-out $(BINLIBS),$(filter-out $(BINCFG),$^))) $(patsubst %,--lib %, $(BINLIBS))

clean:
	$(RM) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(BIN) $(BINMAP)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
