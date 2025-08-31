# Simple eBPF build for this folder
UNAME_M := $(shell uname -m)
ifeq ($(UNAME_M),x86_64)
  TARGET_ARCH := x86
else ifeq ($(UNAME_M),aarch64)
  TARGET_ARCH := arm64
else
  $(error Unsupported arch $(UNAME_M))
endif

CLANG ?= clang
CFLAGS := -O2 -g -Wall -Werror -target bpf -D__TARGET_ARCH_$(TARGET_ARCH) \
          -I. -I/usr/include -I/usr/include/bpf

SOURCES := $(filter %.c %.bpf.c,$(wildcard *.c *.bpf.c))
SOURCES := $(filter-out lkm.c lkm.mod.c module-common.c Module.symvers,$(SOURCES))

OBJECTS := $(SOURCES:.c=.bpf.o)
OBJECTS := $(OBJECTS:.bpf.bpf.o=.bpf.o)

.PHONY: all clean list

all: $(OBJECTS)

list:
	@echo "Sources: $(SOURCES)"
	@echo "Objects: $(OBJECTS)"

%.bpf.o: %.bpf.c
	$(CLANG) $(CFLAGS) -c $< -o $@

%.bpf.o: %.c
	$(CLANG) $(CFLAGS) -c $< -o $@

clean:
	rm -f *.bpf.o
