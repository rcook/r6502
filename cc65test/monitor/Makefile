SOURCES = main.s wozmon.s
PROGRAM = monitor.sim65

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o)
	ld65 -C monitor.cfg -m $(PROGRAM:.sim65=.map) -o $@ $^

clean:
	$(RM) $(SOURCES:.s=.o) $(SOURCES:.s=.lst) $(PROGRAM) $(PROGRAM:.sim65=.map)

run: $(PROGRAM)
	#sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
