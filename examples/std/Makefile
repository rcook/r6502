include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default prereqs run

CC65OBJS := copydata.o zeropage.o
$(CC65OBJS): $(CC65LIBDIR)/none.lib
	ar65 x $^ $@
	touch $@

STDLIB := std.$(LIBEXT)
STDLIBSOURCES := memcpy.s num_to_str.s print.s os.s str_to_num.s
STDLIBMAP := $(STDLIB:.$(LIBEXT)=.map)
$(STDLIB): $(STDLIBSOURCES:.s=.o) $(CC65OBJS)
	ar65 r $@ $^

BIN := test.$(BINEXT)
BINSOURCES := header.s main.s

ALLSOURCES := $(STDLIBSOURCES) $(BINSOURCES)

prereqs:

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(STDLIB)))

clean:
	$(RM) $(CC65OBJS) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(STDLIB) $(STDLIBMAP) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

$(SHAREDLIBDIR)/$(STDLIB): $(STDLIB)
	cp $^ $@

copy: $(SHAREDLIBDIR)/$(STDLIB)

default: $(BIN) copy
