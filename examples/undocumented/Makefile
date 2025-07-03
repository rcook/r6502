include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

BIN := undocumented.$(BINEXT)
BINSOURCES := header.s main.s

prereqs:

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib $(SHAREDLIBDIR)/acorn.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
