include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default prereqs run

BIN := memcpy-demo.$(BINEXT)
BINSOURCES := main.s
main.o: main.s $(SHAREDLIBDIR)/r6502.mac $(SHAREDLIBDIR)/util.mac

prereqs:
	make -C $(PROJECTDIR)/examples/std

$(eval $(call ld65v2,$(BIN),$(SHAREDLIBDIR)/sideways-with-code.cfg,$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib $(SHAREDLIBDIR)/acorn.lib,prereqs $(CONFIGDIR)/acorn.r6502))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
