include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean debug default run

BIN := integer-basic.$(BINEXT)
BINSOURCES := a1basic.s constants.s main.s wozmon.s
main.o: main.s $(SHAREDLIBDIR)/r6502.mac

$(eval $(call ld65v2,$(BIN),$(BIN:.$(BINEXT)=.cfg),$(BINSOURCES:.s=.o),$(CC65LIBDIR)/none.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(BIN)
	cargo run -- debug $(BIN)

default: $(BIN)
