.DEFAULT_GOAL := default
.SUFFIXES:
default: $(PROGRAM)

MAKEFILES := $(wildcard */Makefile)
ALLSUBDIRS := $(MAKEFILES:/Makefile=)
SUBDIRS := $(patsubst bug%,,$(ALLSUBDIRS))
.PHONY: clean default $(SUBDIRS)

$(SUBDIRS):
	@echo '[$@]'
	@$(MAKE) -C $@
	@echo '[/$@]'

default: $(SUBDIRS)
