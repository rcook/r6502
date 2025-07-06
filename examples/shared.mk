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

ifdef DEBUG
	DEBUGARG := -DDEBUG
else
	DEBUGARG :=
endif

# Standard rules
%.o: %.s
	ca65 $(DEBUGARG) -I $(SHAREDLIBDIR) -l $(@:o=lst) -o $@ $<

# ld65 rule
# $(1): output
# $(2): object files
# $(3): libraries
# $(4): additional dependencies
define ld65
$(1): $(4) prereqs $(1:.$(BINEXT)=.cfg) $(2) $(3) $(4)
	ld65 \
		-C $(1:.$(BINEXT)=.cfg) \
		-vm \
		-m $(1:.$(BINEXT)=.map) \
		-o $(1) \
		$(2) \
		$(patsubst %,--lib %,$(3))
ARTIFACTS := $$(ARTIFACTS) $(1) $(1:.$(BINEXT)=.map)
endef

# ld65v2 rule
# $(1): output
# $(2): configuration file
# $(3): object files
# $(4): libraries
# $(5): additional dependencies
define ld65v2
$(1): $(2) $(3) $(4) $(5)
	ld65 \
		-C $(2) \
		-vm \
		-m $(1:.$(BINEXT)=.map) \
		-o $(1) \
		$(3) \
		$(patsubst %,--lib %,$(4))
ARTIFACTS := $$(ARTIFACTS) $(1) $(1:.$(BINEXT)=.map)
endef
