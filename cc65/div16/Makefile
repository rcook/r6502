NAME = div16
SOURCES = $(NAME).s
PROGRAM = $(NAME).bin
TARGET = sim6502

.SUFFIXES:
.PHONY: all clean run
all: $(PROGRAM)

%.o: %.s
	ca65 -U -l $(@:o=lst) -o $@ $<

$(PROGRAM): $(SOURCES:.s=.o)
	ld65 -t $(TARGET) -m $(PROGRAM:bin=map) -o $@ $^ sim6502.lib

clean:
	$(RM) $(SOURCES:.s=.o) $(PROGRAM) $(PROGRAM:bin=lst) $(PROGRAM:bin=map)

run: $(PROGRAM)
	sim65 --verbose --cycles $(PROGRAM) && echo "Program succeeded with exit code $$?" || { echo "Program failed with exit code $$?"; exit 1; }
