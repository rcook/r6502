SOURCES = strings.s
PROGRAM = strings.sim65

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) $(PROGRAM:.sim65=.cfg)
	ld65 -C $(PROGRAM:.sim65=.cfg) -m $(PROGRAM:.sim65=.map) -o $@ $(filter-out $(PROGRAM:.sim65=.cfg),$^)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.sim65=.map)

run: $(PROGRAM)
	#sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
	cargo run -p r6502 run $(PROGRAM) --cycles --emu custom
