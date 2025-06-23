.DEFAULT_GOAL := default
.SUFFIXES:

.PHONY: clean default run

LIBEXT := lib
LIBSOURCES := console.s num_to_str.s os.s str_to_num.s
LIB := stdlib.$(LIBEXT)
$(LIB): $(LIBSOURCES:.s=.o)
	ar65 r $@ $(LIBSOURCES:.s=.o)

BINEXT := r6502
BINSOURCES := header.s test.s
BIN := test.$(BINEXT)
BINTARGETLIB := none.lib

ALLSOURCES = $(LIBSOURCES) $(BINSOURCES)

%.o: %.s
	ca65 --include-dir ../lib -U -l $(@:o=lst) -o $@ $<

LIBS := $(LIB) none.lib
$(BIN): $(BINSOURCES:.s=.o) $(BINCFG) $(LIB)
	ld65 -C $(BIN:.$(BINEXT)=.cfg) -vm -m $(BIN:.$(BINEXT)=.map) -o $@ $(filter-out $(LIBS),$(filter-out $(BINCFG),$^)) $(patsubst %,--lib %, $(LIBS))

clean:
	$(RM) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(LIB) $(LIB:.$(LIBEXT)=.map) $(BIN) $(BIN:.$(BINEXT)=.map)

default: $(BIN)

run: $(BIN)
	cargo run -- run $(BIN) --trace
