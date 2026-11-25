UNAME_M := $(shell uname -m)
ifeq ($(UNAME_M),x86_64)
  TARGET_ARCH := x86
else ifeq ($(UNAME_M),aarch64)
  TARGET_ARCH := arm64
else ifeq ($(UNAME_M),arm64)
  TARGET_ARCH := arm64
else
  $(error Unsupported arch $(UNAME_M))
endif

# Cross builds name the target and its kernel types:
#   make TARGET_ARCH=x86 VMLINUX_H=include/x86_64/vmlinux.h
# The sources include "vmlinux.h" from their own directory, where the build host's
# header sits, so a build with VMLINUX_H compiles copies of the sources in
# build/<arch>/ next to the target's header. The objects land there too.

CLANG ?= clang

BPF_SRCS := $(wildcard *.bpf.c)
BPF_OBJS := $(BPF_SRCS:.bpf.c=.bpf.o)

# BPF compile flags: kernel-only headers
CFLAGS := -O2 -g -Wall -Werror -target bpf -D__TARGET_ARCH_$(TARGET_ARCH)
INCLUDES := -I. -I/usr/include/bpf
# hard drop any generic user includes from BPF path
CFLAGS := $(filter-out -I/usr/include,$(CFLAGS))

ifdef VMLINUX_H
OUT := build/$(TARGET_ARCH)
all: $(addprefix $(OUT)/,$(BPF_OBJS))

$(OUT)/headers.stamp: $(VMLINUX_H) $(filter-out vmlinux.h,$(wildcard *.h))
	mkdir -p $(OUT)
	cp $(filter-out vmlinux.h,$(wildcard *.h)) $(OUT)/
	cp $(VMLINUX_H) $(OUT)/vmlinux.h
	touch $@

$(OUT)/%.bpf.o: %.bpf.c $(OUT)/headers.stamp
	cp $< $(OUT)/$<
	$(CLANG) $(CFLAGS) $(INCLUDES) -c $(OUT)/$< -o $@
else
all: $(BPF_OBJS)

%.bpf.o: %.bpf.c
	$(CLANG) $(CFLAGS) $(INCLUDES) -c $< -o $@
endif

clean:
	rm -f *.bpf.o
	rm -rf build
