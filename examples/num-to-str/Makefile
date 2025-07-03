include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BINSOURCES := header.s num-to-str.s test.s
BIN := num-to-str.$(BINEXT)
BINCFG := $(BIN:.$(BINEXT)=.cfg)
BINMAP := $(BIN:.$(BINEXT)=.map)
BINTARGETLIB := $(CC65LIBDIR)/none.lib

ALLSOURCES := $(BINSOURCES)

prereqs:
	# None

%.o: %.s
	ca65 --include-dir $(SHAREDLIBDIR) -l $(@:o=lst) -o $@ $<

BINLIBS := $(BINTARGETLIB)
$(BIN): prereqs $(BINSOURCES:.s=.o) $(BINCFG) $(BINLIBS)
	ld65 -C $(BINCFG) -vm -m $(BINMAP) -o $@ $(filter-out prereqs,$(filter-out $(BINLIBS),$(filter-out $(BINCFG),$^))) $(patsubst %,--lib %, $(BINLIBS))

clean:
	$(RM) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(BIN) $(BINMAP)

run: $(BIN)
	cargo run -- run $(BIN) --trace

default: $(BIN)
