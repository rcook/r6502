PROGRAM = main

.PHONY: all clean

all:
	ca65 -o $(PROGRAM).o $(PROGRAM).s
	ld65 -C $(PROGRAM).cfg -o $(PROGRAM).bin $(PROGRAM).o

clean:
	rm -f $(PROGRAM).bin $(PROGRAM).o
