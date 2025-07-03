include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default prereqs run

BIN := sim65-div16.$(BINEXT)
BINSOURCES := main.s

prereqs:


$(BIN): prereqs $(BINSOURCES:.s=.o) $(1:.$(BINEXT)=.cfg) $(CC65LIBDIR)/sim6502.lib
	ld65 \
		-t sim6502 \
		-vm \
		-m $(BIN:.$(BINEXT)=.map) \
		-o $(BIN) \
		$(BINSOURCES:.s=.o) \
		$(patsubst %,--lib %,$(CC65LIBDIR)/sim6502.lib)
ARTIFACTS := $(ARTIFACTS) $(BIN) $(BIN:.$(BINEXT)=.map)

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	sim65 --verbose --cycles $(BIN) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }

default: $(BIN)
