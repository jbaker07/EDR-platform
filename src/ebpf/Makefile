# ebpf/Makefile — build CO-RE .o files

ARCH := $(shell uname -m)
TARGET := bpf

# Map uname -m to __TARGET_ARCH_* for clang
ifeq ($(ARCH),x86_64)
BPF_ARCH := x86
else ifeq ($(ARCH),aarch64)
BPF_ARCH := arm64
else ifeq ($(ARCH),arm64)
BPF_ARCH := arm64
else
BPF_ARCH := x86
endif

CFLAGS = -O2 -g -target $(TARGET) -D__TARGET_ARCH_$(BPF_ARCH) -Wall -Wno-unused-value -Wno-pointer-sign -Wno-compare-distinct-pointer-types -Wno-unknown-warning-option

INCS = -I.   # vmlinux.h lives here

SRC = syscall_tracer.bpf.c net_flow.bpf.c wx_exec.bpf.c
OBJ = $(SRC:.c=.o)

all: $(OBJ)

%.o: %.c vmlinux.h
	clang $(CFLAGS) $(INCS) -c $< -o $@

clean:
	rm -f $(OBJ)

.PHONY: all clean
