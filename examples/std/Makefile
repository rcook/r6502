.DEFAULT_GOAL := default
.SUFFIXES:

.PHONY: clean default run

# Probably won't work on Windows
CC65DIR := $(shell dirname $$(dirname $$(readlink -f $$(which ld65))))
CC65LIBDIR := $(CC65DIR)/share/cc65/lib
CC65OBJS := copydata.o zeropage.o
$(CC65OBJS): $(CC65LIBDIR)/none.lib
	ar65 x $^ $@

LIBEXT := lib
LIBSOURCES := console.s num_to_str.s os.s str_to_num.s
LIB := std.$(LIBEXT)
$(LIB): $(LIBSOURCES:.s=.o) $(CC65OBJS)
	ar65 r $@ $^

BINEXT := r6502
BINSOURCES := header.s test.s
BIN := test.$(BINEXT)
BINTARGETLIB := none.lib

ALLSOURCES = $(LIBSOURCES) $(BINSOURCES)

%.o: %.s
	ca65 --include-dir ../lib -U -l $(@:o=lst) -o $@ $<

LIBS := $(LIB)
$(BIN): $(BINSOURCES:.s=.o) $(BINCFG) $(LIB)
	ld65 -C $(BIN:.$(BINEXT)=.cfg) -vm -m $(BIN:.$(BINEXT)=.map) -o $@ $(filter-out $(LIBS),$(filter-out $(BINCFG),$^)) $(patsubst %,--lib %, $(LIBS))

clean:
	$(RM) $(CC65OBJS) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(LIB) $(LIB:.$(LIBEXT)=.map) $(BIN) $(BIN:.$(BINEXT)=.map)

run: $(BIN)
	cargo run -- run $(BIN) --trace

../lib/$(LIB): $(LIB)
	cp $^ $@

copy: ../lib/$(LIB)

default: copy
