include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

CC65OBJS := copydata.o zeropage.o
$(CC65OBJS): $(CC65LIBDIR)/none.lib
	ar65 x $^ $@
	touch $@

STDLIBSOURCES := console.s num_to_str.s os.s str_to_num.s
STDLIB := std.$(LIBEXT)
STDLIBMAP := $(STDLIB:.$(LIBEXT)=.map)
$(STDLIB): $(STDLIBSOURCES:.s=.o) $(CC65OBJS)
	ar65 r $@ $^

BINSOURCES := header.s test.s
BIN := test.$(BINEXT)
BINCFG := $(BIN:.$(BINEXT)=.cfg)
BINMAP := $(BIN:.$(BINEXT)=.map)
BINTARGETLIB :=

ALLSOURCES = $(STDLIBSOURCES) $(BINSOURCES)

%.o: %.s
	ca65 --include-dir $(SHAREDLIBDIR) -U -l $(@:o=lst) -o $@ $<

BINLIBS := $(BINTARGETLIB) $(STDLIB)
$(BIN): $(BINSOURCES:.s=.o) $(BINCFG) $(BINLIBS)
	ld65 -C $(BINCFG) -vm -m $(BINMAP) -o $@ $(filter-out $(BINLIBS),$(filter-out $(BINCFG),$^)) $(patsubst %,--lib %, $(BINLIBS))

clean:
	$(RM) $(CC65OBJS) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(STDLIB) $(STDLIBMAP) $(BIN) $(BINMAP)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

$(SHAREDLIBDIR)/$(STDLIB): $(STDLIB)
	cp $^ $@

copy: $(SHAREDLIBDIR)/$(STDLIB)

default: copy
