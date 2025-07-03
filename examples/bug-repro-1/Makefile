include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BIN := main.$(BINEXT)
BINSOURCES := main.s

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/none.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

default: $(BIN)
