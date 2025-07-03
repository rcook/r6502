CC65DIR := $(shell dirname $$(dirname $$(readlink -f $$(which ld65))))
ifeq ($(OS),Windows_NT)
CC65DIR := $(shell echo $(CC65DIR) | sed -e 's/\/\(.\)\//\\1:\//g')
endif
ifeq ($(OS),Windows_NT)
CC65LIBDIR := $(CC65DIR)/lib
else
CC65LIBDIR := $(CC65DIR)/share/cc65/lib
endif

LIBEXT := lib
BINEXT := r6502

ifneq ($(notdir $(lastword $(MAKEFILE_LIST))), shared.mk)
$(error Could not determine path to shared.mk)
endif
SHAREDMKPATH := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECTDIR := $(dir $(SHAREDMKPATH))
PROJECTDIR := $(dir $(PROJECTDIR:/=))
PROJECTDIR := $(PROJECTDIR:/=)
ifeq ($(wildcard $(PROJECTDIR)/.gitignore),)
$(error Could not determine containing project directory)
endif
SHAREDLIBDIR := $(PROJECTDIR)/examples/lib
CONFIGDIR := $(PROJECTDIR)/config
ARTIFACTS :=

# Standard rules
%.o: %.s
	ca65 --include-dir $(SHAREDLIBDIR) -l $(@:o=lst) -o $@ $<

# ld65 rule
define ld65
$(1): prereqs $(2) $(1:.$(BINEXT)=.cfg) $(3)
	ld65 \
		-C $(1:.$(BINEXT)=.cfg) \
		-vm \
		-m $(1:.$(BINEXT)=.map) \
		-o $(1) \
		$(2) \
		$(patsubst %,--lib %,$(3))
ARTIFACTS := $$(ARTIFACTS) $(1) $(1:.$(BINEXT)=.map)
endef
