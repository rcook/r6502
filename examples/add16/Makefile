include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default prereqs run

BIN := add16.$(BINEXT)
BINSOURCES := main.s

prereqs:
	make -C $(PROJECTDIR)/examples/std

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib $(SHAREDLIBDIR)/acorn.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace --cycles

default: $(BIN)
