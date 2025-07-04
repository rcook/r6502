include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default prereqs run

BIN := bbc-basic.$(BINEXT)
BINSOURCES := header.s main.s
main.o: main.s $(CONFIGDIR)/acorn.r6502

prereqs:
	make -C $(PROJECTDIR)/examples/acorn
	make -C $(PROJECTDIR)/examples/std

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib $(SHAREDLIBDIR)/acorn.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
