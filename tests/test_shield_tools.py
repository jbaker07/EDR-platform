"""Tests for the shield tools: scripts/shieldctl.py writes shield documents, and
scripts/ci/check_bpf_atomics.py keeps atomic instructions out of the BPF programs.

edr-agent/tests/fixtures/shield_doc.json is the document format. The agent's Rust
test parses that file, and the test here checks that shieldctl writes the same keys,
so the two sides cannot drift apart.
"""

from __future__ import annotations

import importlib.util
import json
import os
import struct
from pathlib import Path
from types import SimpleNamespace

import pytest

_REPO = Path(__file__).resolve().parent.parent


def _load(name: str, rel: str):
    spec = importlib.util.spec_from_file_location(name, _REPO / rel)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


shieldctl = _load("shieldctl", "scripts/shieldctl.py")
atomics = _load("check_bpf_atomics", "scripts/ci/check_bpf_atomics.py")
FIXTURE = json.loads((_REPO / "edr-agent" / "tests" / "fixtures" / "shield_doc.json").read_text())


def _args(**kw):
    base = dict(kind="connect_deny", path=None, cidr=None, port=None, cgroup=None, ttl=3600,
                mode="enforce", id=None, serial=None, reason=None, incident=None)
    base.update(kw)
    return SimpleNamespace(**base)


def test_shieldctl_writes_the_fixture_shape_for_connect_deny():
    doc = shieldctl.new_doc(_args(cidr="203.0.113.10", port=4444, cgroup="/sys/fs/cgroup/x", reason="c2"))["shield"]
    want = FIXTURE["shield"]
    assert set(doc) == set(want) - {"incident_id"}
    assert set(doc["match"]) == set(want["match"])
    assert doc["expires_at"] - doc["not_before"] == 3600
    assert doc["serial"] == doc["not_before"], "the serial defaults to the time, so it only moves up"


def test_shieldctl_names_a_file_by_device_and_inode(tmp_path):
    f = tmp_path / "payload"
    f.write_bytes(b"#!/bin/sh\n")
    doc = shieldctl.new_doc(_args(kind="exec_deny", path=str(f), incident="inc-1"))["shield"]
    st = os.stat(f)
    assert doc["match"] == {"path": str(f), "dev": st.st_dev, "ino": st.st_ino}
    assert doc["incident_id"] == "inc-1" and "reason" not in doc


def test_shieldctl_refuses_a_document_without_a_reason():
    with pytest.raises(SystemExit):
        shieldctl.new_doc(_args(cidr="192.0.2.1"))


def _insn(op: int) -> bytes:
    return bytes([op, 0x21, 0, 0, 0, 0, 0, 0])


def test_atomic_instructions_are_found_and_plain_stores_are_not():
    code = b"".join([
        _insn(0x7B),  # stx dw, a plain store
        _insn(0xDB),  # atomic 64 bit, what __sync_fetch_and_add on a u64 compiles to
        _insn(0x18), _insn(0x00),  # ld_imm64 takes two slots
        _insn(0xC3),  # atomic 32 bit
        _insn(0x95),  # exit
    ])
    assert atomics.find_atomics(code) == [(8, 0xDB), (32, 0xC3)]
    assert atomics.find_atomics(_insn(0x7B) + _insn(0x95)) == []


def _tiny_elf(code: bytes) -> bytes:
    """A little endian ELF64 with a null section, one executable section and .shstrtab."""
    shstr = b"\0prog\0.shstrtab\0"
    code_off = 64
    str_off = code_off + len(code)
    sh_off = (str_off + len(shstr) + 7) // 8 * 8
    hdr = b"\x7fELF" + bytes([2, 1, 1]) + bytes(9)
    hdr += struct.pack("<HHIQQQIHHHHHH", 1, 247, 1, 0, 0, sh_off, 0, 64, 0, 0, 64, 3, 2)
    out = bytearray(hdr) + code + shstr
    out += bytes(sh_off - len(out))
    out += bytes(64)
    out += struct.pack("<IIQQQQIIQQ", 1, 1, 0x6, 0, code_off, len(code), 0, 0, 8, 0)
    out += struct.pack("<IIQQQQIIQQ", 6, 3, 0, 0, str_off, len(shstr), 0, 0, 1, 0)
    return bytes(out)


def test_the_checker_reads_an_elf_object(tmp_path):
    bad = tmp_path / "bad.bpf.o"
    bad.write_bytes(_tiny_elf(_insn(0x7B) + _insn(0xDB) + _insn(0x95)))
    good = tmp_path / "good.bpf.o"
    good.write_bytes(_tiny_elf(_insn(0x7B) + _insn(0x95)))
    found = atomics.check(bad)
    assert len(found) == 1 and "prog" in found[0] and "64 bit atomic" in found[0]
    assert atomics.check(good) == []
    assert atomics.main([str(tmp_path)]) == 1


@pytest.mark.skipif(not os.environ.get("BPF_OBJ_DIR"), reason="set BPF_OBJ_DIR to the built .bpf.o files")
def test_the_built_programs_have_no_atomics():
    assert atomics.main([os.environ["BPF_OBJ_DIR"]]) == 0
