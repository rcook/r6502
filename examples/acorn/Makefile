include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean copy debug default prereqs run user

ACORNLIB := acorn.$(LIBEXT)
ACORNLIBSOURCES := os.s
ACORNLIBMAP := $(ACORNLIB:.$(LIBEXT)=.map)
$(ACORNLIB): $(ACORNLIBSOURCES:.s=.o)
	ar65 r $@ $^

$(SHAREDLIBDIR)/$(ACORNLIB): $(ACORNLIB)
	cp $^ $@

IMAGE := acorn.$(BINEXT)
IMAGESOURCES := header.s os-impl.s

BIN := acorn-test.$(BINEXT)
BINSOURCES := main.s

ALLSOURCES := $(ACORNLIBSOURCES) $(IMAGESOURCES) $(BINSOURCES)

prereqs:

$(eval $(call ld65,$(IMAGE),$(IMAGESOURCES:.s=.o),$(SHAREDLIBDIR)/$(ACORNLIB)))

$(eval $(call ld65,$(BIN),$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/$(ACORNLIB)))

$(CONFIGDIR)/$(IMAGE): $(IMAGE)
	cp $^ $@

run: $(CONFIGDIR)/$(IMAGE) $(BIN)
	cargo run -- run $(BIN) --trace

debug: $(CONFIGDIR)/$(IMAGE) $(BIN)
	cargo run -- debug $(BIN)

user: $(CONFIGDIR)/$(IMAGE)
	cargo run -- run $(PROJECTDIR)/examples/hello-world/hello-world.r6502 --trace

clean:
	$(RM) $(ALLSOURCES:.s=.o) $(ALLSOURCES:.s=.lst) $(ACORNLIB) $(ACORNLIBMAP) $(ARTIFACTS)

default: $(CONFIGDIR)/$(IMAGE) $(BIN)
