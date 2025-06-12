SOURCES = constants.s main.s simple.s vectors.s
PROGRAM = simple.sim65
TARGET = none

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(PROGRAM:.sim65=.cfg)
	ld65 -C $(PROGRAM:.sim65=.cfg) -m $(PROGRAM:.sim65=.map) -o $@ $(filter-out $(PROGRAM:.sim65=.cfg),$^) $(TARGET).lib

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.sim65=.map)

run: $(PROGRAM)
	cargo run -p r6502 run $(PROGRAM) --reset --cycles --emu acorn --trace
