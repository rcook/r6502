include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean copy debug default prereq run user

BINSOURCES := header.s main.s os.s
BIN := acorn.$(BINEXT)
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

../../config/$(BIN): $(BIN)
	cp $^ $@

copy: ../../config/$(BIN)

run: copy
	cargo run -- run $(BIN) --trace

debug: copy
	cargo run -- debug $(BIN)

user: copy
	cargo run -- run ../hello-world/hello-world.r6502 --trace

default: copy
