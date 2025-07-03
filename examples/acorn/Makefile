include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean copy debug default prereq run user

BIN := acorn.$(BINEXT)
BINSOURCES := header.s main.s os.s

prereqs:

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(CC65LIBDIR)/none.lib))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

$(CONFIGDIR)/$(BIN): $(BIN)
	cp $^ $@

copy: $(CONFIGDIR)/$(BIN)

run: copy
	cargo run -- run $(BIN) --trace

debug: copy
	cargo run -- debug $(BIN)

user: copy
	cargo run -- run $(PROJECTDIR)/examples/hello-world/hello-world.r6502 --trace

default: copy
