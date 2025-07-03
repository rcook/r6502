include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BIN := div16.$(BINEXT)
BINSOURCES := main.s

prereqs:

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(CC65LIBDIR)/none.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

default: $(BIN)
