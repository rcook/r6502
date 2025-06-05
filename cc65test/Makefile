SOURCES = program.s

PROGRAM = program.bin
TARGET = sim6502

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o)
	ld65 -t $(TARGET) -m $(PROGRAM).map -o $@ $^ sim6502.lib

clean:
	$(RM) $(SOURCES:.s=.o) $(PROGRAM) $(PROGRAM).map

run: $(PROGRAM)
	sim65 --verbose --cycles $(PROGRAM)
