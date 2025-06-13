.DEFAULT_GOAL := default
.SUFFIXES:
BINEXT := r6502
PROGRAM := acorn.$(BINEXT)
default: copy

.PHONY: clean copy default run

SOURCES := acorn.s os.s main.s vectors.s
CFG := $(PROGRAM:.$(BINEXT)=.cfg)
TARGET := -C $(CFG)
LIB := none.lib

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(CFG)
	ld65 $(TARGET) -m $(PROGRAM:.$(BINEXT)=.map) -o $@ $(filter-out $(CFG),$^) $(LIB)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.$(BINEXT)=.map)

copy: $(PROGRAM)
	cp $(PROGRAM) ../../config/$(PROGRAM)

run: copy
	make -C ../hello-world
	cargo run -p r6502 run ../hello-world/hello-world.r6502
