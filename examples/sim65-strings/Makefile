include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default run

BIN := sim65-strings.$(BINEXT)
BINSOURCES := main.s

prereqs:

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(CC65LIBDIR)/none.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	@#sim65 --verbose --cycles $(BIN) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
	cargo run -- run $(BIN) -m Acorn

default: $(BIN)
