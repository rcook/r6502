.DEFAULT_GOAL := default
.SUFFIXES:
default: $(PROGRAM)

MAKEFILES := $(wildcard */Makefile)
ALL_SUBDIRS := $(MAKEFILES:/Makefile=)
SUBDIRS := $(patsubst bug%,,$(ALL_SUBDIRS))
.PHONY: clean default $(SUBDIRS)

$(SUBDIRS):
	@echo '[$@]'
	@$(MAKE) -C $@
	@echo '[/$@]'

default: $(SUBDIRS)
