SOURCES = main.s wozmon.s
PROGRAM = monitor.sim65

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o) monitor.cfg
	ld65 -C monitor.cfg -m $(PROGRAM:.sim65=.map) -o $@ $(filter-out monitor.cfg,$^)

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.sim65=.map)

run: $(PROGRAM)
	#sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
	cargo run -p r6502 run $(PROGRAM) --reset --cycles --emu apple1
