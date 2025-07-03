include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default prereqs run

BIN := preserve-stack.$(BINEXT)
BINSOURCES := header.s main.s

prereqs:
	make -C $(PROJECTDIR)/examples/std

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
