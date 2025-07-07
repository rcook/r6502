include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean default prereqs run trace

BIN := tests.$(BINEXT)
BINSOURCES := \
	main.s \
	test_add16.s \
	test_div16.s \
	test_num_to_str.s \
	test_preserve_stack.s \
	test_str_to_num.s
main.o: main.s $(SHAREDLIBDIR)/r6502.mac
test_add16.o: test_add16.s $(SHAREDLIBDIR)/r6502.mac $(SHAREDLIBDIR)/util.mac
test_div16.o: test_div16.s $(SHAREDLIBDIR)/r6502.mac $(SHAREDLIBDIR)/util.mac
test_num_to_str.o: test_num_to_str.s $(SHAREDLIBDIR)/util.mac
test_preserve_stack.o: test_preserve_stack.s $(SHAREDLIBDIR)/util.mac
test_str_to_num.o: test_str_to_num.s $(SHAREDLIBDIR)/util.mac

prereqs:
	make -C $(PROJECTDIR)/examples/acorn
	make -C $(PROJECTDIR)/examples/std

$(eval $(call ld65v2,$(BIN),$(SHAREDLIBDIR)/sideways-with-code.cfg,$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/std.lib $(SHAREDLIBDIR)/acorn.lib,prereqs $(CONFIGDIR)/acorn.r6502))

clean:
	$(RM) $(BINSOURCES:.s=.o) $(BINSOURCES:.s=.lst) $(ARTIFACTS)

run: $(BIN)
	cargo run -- run $(BIN)

trace: $(BIN)
	cargo run -- run $(BIN) --trace

default: $(BIN)
