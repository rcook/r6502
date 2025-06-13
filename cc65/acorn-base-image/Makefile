SOURCES = acorn.s os.s main.s vectors.s
PROGRAM = acorn.r6502
TARGET = none

.SUFFIXES:
.PHONY: all clean copy run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(PROGRAM:.r6502=.cfg)
	ld65 -C $(PROGRAM:.r6502=.cfg) -m $(PROGRAM:.r6502=.map) -o $@ $(filter-out $(PROGRAM:.r6502=.cfg),$^) $(TARGET).lib

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.r6502=.map)

copy: $(PROGRAM)
	cp $(PROGRAM) ../../config/$(PROGRAM)

run: copy
	cargo run -p r6502 run ../../examples/hello-world.r6502 -m Acorn
