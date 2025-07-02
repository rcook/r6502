CC65DIR := $(shell dirname $$(dirname $$(readlink -f $$(which ld65))))
ifeq ($(OS),Windows_NT)
CC65DIR := $(shell echo $(CC65DIR) | sed -e 's/\/\(.\)\//\\1:\//g')
endif
ifeq ($(OS),Windows_NT)
CC65LIBDIR := $(CC65DIR)/lib
else
CC65LIBDIR := $(CC65DIR)/share/cc65/lib
endif

SHAREDMKPATH := $(abspath $(lastword $(MAKEFILE_LIST)))
SHAREDLIBDIR := $(dir $(SHAREDMKPATH))lib

LIBEXT := lib
BINEXT := r6502
