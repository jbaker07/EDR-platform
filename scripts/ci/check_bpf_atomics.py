#!/usr/bin/env python3
# scripts/ci/check_bpf_atomics.py
#
# Fails when any compiled BPF program contains an atomic instruction.
#
# Why: the kernel's verifier accepts an atomic add on a map value, but it does not
# check that the value is aligned. An LPM trie keeps each value right after a key
# of any length, so its values can sit on a 4 byte boundary, and on arm64 a 64 bit
# atomic on such an address faults in kernel mode. An early version of
# edr_shield.bpf.c did __sync_fetch_and_add on a hit counter in an LPM trie value,
# and the first live run panicked the kernel. The programs now count with per CPU
# arrays and plain adds, and this check keeps any atomic from coming back.
#
#   check_bpf_atomics.py <object.bpf.o or directory> ...
#
# It reads the ELF file itself (no LLVM tools needed), looks at every executable
# section, and reports each atomic instruction by section, function and offset.
# Standard library only.

import struct
import sys
from pathlib import Path

SHF_EXECINSTR = 0x4
SHT_SYMTAB = 2
STT_FUNC = 2
BPF_CLASS_MASK = 0x07
BPF_STX = 0x03
BPF_MODE_MASK = 0xE0
BPF_ATOMIC = 0xC0
SIZES = {0x00: "32 bit", 0x08: "16 bit", 0x10: "8 bit", 0x18: "64 bit"}


def find_atomics(code: bytes):
    """Offsets and opcodes of atomic instructions in raw BPF code (8 byte slots)."""
    out = []
    for off in range(0, len(code) - 7, 8):
        op = code[off]
        if op & BPF_CLASS_MASK == BPF_STX and op & BPF_MODE_MASK == BPF_ATOMIC:
            out.append((off, op))
    return out


def sections(data: bytes):
    """(name, flags, type, offset, size, link, entsize) for each section of a little endian ELF64."""
    if data[:4] != b"\x7fELF" or data[4] != 2 or data[5] != 1:
        raise ValueError("not a little endian ELF64 file")
    shoff = struct.unpack_from("<Q", data, 0x28)[0]
    shentsize, shnum, shstrndx = struct.unpack_from("<HHH", data, 0x3A)
    raw = []
    for i in range(shnum):
        b = shoff + i * shentsize
        name, typ, flags, _addr, off, size, link, _info, _align, entsize = struct.unpack_from("<IIQQQQIIQQ", data, b)
        raw.append((name, typ, flags, off, size, link, entsize))
    strtab = raw[shstrndx]
    def sname(n):
        start = strtab[3] + n
        return data[start:data.index(b"\0", start)].decode()
    return [(sname(n), flags, typ, off, size, link, entsize) for (n, typ, flags, off, size, link, entsize) in raw]


def functions(data: bytes, secs):
    """{section index: [(start, end, name)]} from the symbol table."""
    funcs = {}
    for name, _flags, typ, off, size, link, entsize in secs:
        if typ != SHT_SYMTAB:
            continue
        strs = secs[link]
        for i in range(size // entsize):
            st_name, st_info, _other, shndx, value, sz = struct.unpack_from("<IBBHQQ", data, off + i * entsize)
            if st_info & 0xF != STT_FUNC:
                continue
            start = strs[3] + st_name
            fname = data[start:data.index(b"\0", start)].decode()
            funcs.setdefault(shndx, []).append((value, value + sz, fname))
    return funcs


def check(path: Path):
    data = path.read_bytes()
    secs = sections(data)
    funcs = functions(data, secs)
    found = []
    for idx, (name, flags, _typ, off, size, _link, _ent) in enumerate(secs):
        if not flags & SHF_EXECINSTR:
            continue
        for at, op in find_atomics(data[off:off + size]):
            fn = next((f for s, e, f in funcs.get(idx, []) if s <= at < e), "?")
            found.append(f"{path.name}: {name} {fn}+{at:#x}: {SIZES.get(op & 0x18, '?')} atomic (opcode {op:#04x})")
    return found


def main(argv):
    files = []
    for a in argv or ["."]:
        p = Path(a)
        files.extend(sorted(p.glob("*.bpf.o")) if p.is_dir() else [p])
    if not files:
        sys.exit("no .bpf.o files given")
    bad = []
    for f in files:
        bad.extend(check(f))
    for line in bad:
        print(line)
    print(f"{len(files)} objects, {len(bad)} atomic instructions")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
