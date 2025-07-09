include ../shared.mk

.DEFAULT_GOAL := default
.SUFFIXES:
.PHONY: clean copy debug default run user

ACORNLIB := acorn.$(LIBEXT)
ACORNLIBSOURCES := \
	ascii.s \
	cpu.s \
	misc.s \
	mos_routines.s \
	mos_vectors.s \
	mos_zp.s \
	os.s
ACORNLIBMAP := $(ACORNLIB:.$(LIBEXT)=.map)
$(ACORNLIB): $(ACORNLIBSOURCES:.s=.o)
	ar65 r $@ $^

$(SHAREDLIBDIR)/$(ACORNLIB): $(ACORNLIB)
	cp $^ $@

IMAGE := acorn.$(BINEXT)
IMAGESOURCES := \
	cpu_impl.s \
	header.s \
	init.s \
	mos_entrypoints.s \
	mos_routines_impl.s \
	os_impl.s \
	zp.s
mos_entrypoints.o: mos_entrypoints.s raw.mac $(SHAREDLIBDIR)/r6502.mac

BIN := acorn-test.$(BINEXT)
BINSOURCES := main.s
main.o: main.s raw.mac $(SHAREDLIBDIR)/r6502.mac

ALLSOURCES := $(ACORNLIBSOURCES) $(IMAGESOURCES) $(BINSOURCES)

$(eval $(call ld65v2,$(IMAGE),$(IMAGE:.$(BINEXT)=.cfg),$(IMAGESOURCES:.s=.o),$(SHAREDLIBDIR)/$(ACORNLIB)))
$(eval $(call ld65v2,$(BIN),$(SHAREDLIBDIR)/sideways.cfg,$(BINSOURCES:.s=.o),$(SHAREDLIBDIR)/$(ACORNLIB)))

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
