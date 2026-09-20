"""A small JVM class-file reader (JVMS chapter 4), used instead of javap text.

Why: javap output is for people. Its annotation rendering omits marker
annotations' parentheses, its member lines drop descriptors unless asked, and a
regex over it cannot tell a Fieldref read from a write. Everything the atlas
needs -- annotations with nested values, member descriptors, instruction
opcodes with offsets, and constant-pool references -- is explicit in the class
file itself. This module reads that and nothing else.

What it does NOT do: verify bytecode, resolve types, or interpret anything.
Unsupported or malformed input raises ``ClassFileError`` with the position, so
an extraction failure is recorded rather than silently skipped.
"""
from __future__ import annotations

import struct
import zipfile
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterator


class ClassFileError(Exception):
    pass


# --- constant pool -------------------------------------------------------------
CONSTANT_Utf8, CONSTANT_Integer, CONSTANT_Float, CONSTANT_Long, CONSTANT_Double = 1, 3, 4, 5, 6
CONSTANT_Class, CONSTANT_String, CONSTANT_Fieldref, CONSTANT_Methodref = 7, 8, 9, 10
CONSTANT_InterfaceMethodref, CONSTANT_NameAndType, CONSTANT_MethodHandle = 11, 12, 15
CONSTANT_MethodType, CONSTANT_Dynamic, CONSTANT_InvokeDynamic, CONSTANT_Module, CONSTANT_Package = 16, 17, 18, 19, 20

# opcode -> (mnemonic, operand byte count); -1 marks the variable-length ones
OPCODES: dict[int, tuple[str, int]] = {}
_simple = {
    0x00: "nop", 0x01: "aconst_null", 0x02: "iconst_m1", 0x03: "iconst_0", 0x04: "iconst_1", 0x05: "iconst_2",
    0x06: "iconst_3", 0x07: "iconst_4", 0x08: "iconst_5", 0x09: "lconst_0", 0x0a: "lconst_1", 0x0b: "fconst_0",
    0x0c: "fconst_1", 0x0d: "fconst_2", 0x0e: "dconst_0", 0x0f: "dconst_1", 0x1a: "iload_0", 0x1b: "iload_1",
    0x1c: "iload_2", 0x1d: "iload_3", 0x1e: "lload_0", 0x1f: "lload_1", 0x20: "lload_2", 0x21: "lload_3",
    0x22: "fload_0", 0x23: "fload_1", 0x24: "fload_2", 0x25: "fload_3", 0x26: "dload_0", 0x27: "dload_1",
    0x28: "dload_2", 0x29: "dload_3", 0x2a: "aload_0", 0x2b: "aload_1", 0x2c: "aload_2", 0x2d: "aload_3",
    0x2e: "iaload", 0x2f: "laload", 0x30: "faload", 0x31: "daload", 0x32: "aaload", 0x33: "baload", 0x34: "caload",
    0x35: "saload", 0x3b: "istore_0", 0x3c: "istore_1", 0x3d: "istore_2", 0x3e: "istore_3", 0x3f: "lstore_0",
    0x40: "lstore_1", 0x41: "lstore_2", 0x42: "lstore_3", 0x43: "fstore_0", 0x44: "fstore_1", 0x45: "fstore_2",
    0x46: "fstore_3", 0x47: "dstore_0", 0x48: "dstore_1", 0x49: "dstore_2", 0x4a: "dstore_3", 0x4b: "astore_0",
    0x4c: "astore_1", 0x4d: "astore_2", 0x4e: "astore_3", 0x4f: "iastore", 0x50: "lastore", 0x51: "fastore",
    0x52: "dastore", 0x53: "aastore", 0x54: "bastore", 0x55: "castore", 0x56: "sastore", 0x57: "pop", 0x58: "pop2",
    0x59: "dup", 0x5a: "dup_x1", 0x5b: "dup_x2", 0x5c: "dup2", 0x5d: "dup2_x1", 0x5e: "dup2_x2", 0x5f: "swap",
    0x60: "iadd", 0x61: "ladd", 0x62: "fadd", 0x63: "dadd", 0x64: "isub", 0x65: "lsub", 0x66: "fsub", 0x67: "dsub",
    0x68: "imul", 0x69: "lmul", 0x6a: "fmul", 0x6b: "dmul", 0x6c: "idiv", 0x6d: "ldiv", 0x6e: "fdiv", 0x6f: "ddiv",
    0x70: "irem", 0x71: "lrem", 0x72: "frem", 0x73: "drem", 0x74: "ineg", 0x75: "lneg", 0x76: "fneg", 0x77: "dneg",
    0x78: "ishl", 0x79: "lshl", 0x7a: "ishr", 0x7b: "lshr", 0x7c: "iushr", 0x7d: "lushr", 0x7e: "iand", 0x7f: "land",
    0x80: "ior", 0x81: "lor", 0x82: "ixor", 0x83: "lxor", 0x85: "i2l", 0x86: "i2f", 0x87: "i2d", 0x88: "l2i",
    0x89: "l2f", 0x8a: "l2d", 0x8b: "f2i", 0x8c: "f2l", 0x8d: "f2d", 0x8e: "d2i", 0x8f: "d2l", 0x90: "d2f",
    0x91: "i2b", 0x92: "i2c", 0x93: "i2s", 0x94: "lcmp", 0x95: "fcmpl", 0x96: "fcmpg", 0x97: "dcmpl", 0x98: "dcmpg",
    0xac: "ireturn", 0xad: "lreturn", 0xae: "freturn", 0xaf: "dreturn", 0xb0: "areturn", 0xb1: "return",
    0xbe: "arraylength", 0xbf: "athrow", 0xc2: "monitorenter", 0xc3: "monitorexit",
}
for _op, _name in _simple.items():
    OPCODES[_op] = (_name, 0)
for _op, _name in {0x10: "bipush", 0x12: "ldc", 0x15: "iload", 0x16: "lload", 0x17: "fload", 0x18: "dload",
                   0x19: "aload", 0x36: "istore", 0x37: "lstore", 0x38: "fstore", 0x39: "dstore", 0x3a: "astore",
                   0xa9: "ret", 0xbc: "newarray"}.items():
    OPCODES[_op] = (_name, 1)
for _op, _name in {0x11: "sipush", 0x13: "ldc_w", 0x14: "ldc2_w", 0x84: "iinc", 0x99: "ifeq", 0x9a: "ifne",
                   0x9b: "iflt", 0x9c: "ifge", 0x9d: "ifgt", 0x9e: "ifle", 0x9f: "if_icmpeq", 0xa0: "if_icmpne",
                   0xa1: "if_icmplt", 0xa2: "if_icmpge", 0xa3: "if_icmpgt", 0xa4: "if_icmple", 0xa5: "if_acmpeq",
                   0xa6: "if_acmpne", 0xa7: "goto", 0xa8: "jsr", 0xb2: "getstatic", 0xb3: "putstatic",
                   0xb4: "getfield", 0xb5: "putfield", 0xb6: "invokevirtual", 0xb7: "invokespecial",
                   0xb8: "invokestatic", 0xbb: "new", 0xbd: "anewarray", 0xc0: "checkcast", 0xc1: "instanceof",
                   0xc6: "ifnull", 0xc7: "ifnonnull"}.items():
    OPCODES[_op] = (_name, 2)
OPCODES[0xc5] = ("multianewarray", 3)
OPCODES[0xb9] = ("invokeinterface", 4)
OPCODES[0xba] = ("invokedynamic", 4)
OPCODES[0xc8] = ("goto_w", 4)
OPCODES[0xc9] = ("jsr_w", 4)
OPCODES[0xaa] = ("tableswitch", -1)
OPCODES[0xab] = ("lookupswitch", -1)
OPCODES[0xc4] = ("wide", -1)

REF_OPCODES = {"getstatic", "putstatic", "getfield", "putfield", "invokevirtual", "invokespecial",
               "invokestatic", "invokeinterface", "invokedynamic", "new", "anewarray", "checkcast",
               "instanceof", "ldc", "ldc_w", "ldc2_w", "multianewarray"}


@dataclass
class Instruction:
    offset: int
    opcode: int
    mnemonic: str
    ref: dict | None = None          # {"kind","owner","name","desc"} or {"kind":"Class","name"} or {"kind":"String"...}


@dataclass
class Member:
    name: str
    desc: str
    access: int
    annotations: list[dict] = field(default_factory=list)
    signature: str | None = None
    code: list[Instruction] | None = None      # methods only, None if abstract/native
    param_annotations: list[list[dict]] = field(default_factory=list)
    annotation_default: object = None          # annotation-interface methods: the default value

    @property
    def is_static(self) -> bool:
        return bool(self.access & 0x0008)


@dataclass
class ClassFile:
    name: str                       # internal form, e.g. net/minecraft/world/level/Level
    super_name: str | None
    interfaces: list[str]
    access: int
    major: int
    fields: list[Member]
    methods: list[Member]
    annotations: list[dict]
    signature: str | None = None
    inner_classes: list[dict] = field(default_factory=list)

    @property
    def kind(self) -> str:
        if self.access & 0x2000:
            return "annotation"
        if self.access & 0x0200:
            return "interface"
        if self.access & 0x4000:
            return "enum"
        if self.super_name == "java/lang/Record":
            return "record"
        if self.access & 0x0400:
            return "abstract_class"
        return "class"


class _Reader:
    def __init__(self, data: bytes) -> None:
        self.d = data
        self.p = 0

    def u1(self) -> int:
        v = self.d[self.p]
        self.p += 1
        return v

    def u2(self) -> int:
        v = struct.unpack_from(">H", self.d, self.p)[0]
        self.p += 2
        return v

    def u4(self) -> int:
        v = struct.unpack_from(">I", self.d, self.p)[0]
        self.p += 4
        return v

    def s4(self) -> int:
        v = struct.unpack_from(">i", self.d, self.p)[0]
        self.p += 4
        return v

    def bytes(self, n: int) -> bytes:
        v = self.d[self.p:self.p + n]
        self.p += n
        return v


def _modified_utf8(raw: bytes) -> str:
    try:
        return raw.decode("utf-8")
    except UnicodeDecodeError:
        # JVM modified UTF-8: encoded NUL and surrogate pairs; decode leniently.
        return raw.replace(b"\xc0\x80", b"\x00").decode("utf-8", errors="replace")


def _read_pool(r: _Reader) -> list:
    count = r.u2()
    pool: list = [None] * count
    i = 1
    while i < count:
        tag = r.u1()
        if tag == CONSTANT_Utf8:
            n = r.u2()
            pool[i] = (tag, _modified_utf8(r.bytes(n)))
        elif tag in (CONSTANT_Integer, CONSTANT_Float):
            pool[i] = (tag, r.u4())
        elif tag in (CONSTANT_Long, CONSTANT_Double):
            pool[i] = (tag, r.bytes(8))
            i += 1  # takes two slots
        elif tag in (CONSTANT_Class, CONSTANT_String, CONSTANT_MethodType, CONSTANT_Module, CONSTANT_Package):
            pool[i] = (tag, r.u2())
        elif tag in (CONSTANT_Fieldref, CONSTANT_Methodref, CONSTANT_InterfaceMethodref, CONSTANT_NameAndType,
                     CONSTANT_Dynamic, CONSTANT_InvokeDynamic):
            pool[i] = (tag, r.u2(), r.u2())
        elif tag == CONSTANT_MethodHandle:
            pool[i] = (tag, r.u1(), r.u2())
        else:
            raise ClassFileError(f"unknown constant pool tag {tag} at index {i}")
        i += 1
    return pool


class _Pool:
    def __init__(self, pool: list) -> None:
        self.pool = pool

    def utf8(self, idx: int) -> str:
        e = self.pool[idx]
        if e is None or e[0] != CONSTANT_Utf8:
            raise ClassFileError(f"constant {idx} is not Utf8")
        return e[1]

    def class_name(self, idx: int) -> str:
        e = self.pool[idx]
        if e is None or e[0] != CONSTANT_Class:
            raise ClassFileError(f"constant {idx} is not Class")
        return self.utf8(e[1])

    def name_and_type(self, idx: int) -> tuple[str, str]:
        e = self.pool[idx]
        if e is None or e[0] != CONSTANT_NameAndType:
            raise ClassFileError(f"constant {idx} is not NameAndType")
        return self.utf8(e[1]), self.utf8(e[2])

    def ref(self, idx: int) -> dict:
        e = self.pool[idx]
        if e is None:
            raise ClassFileError(f"constant {idx} is empty")
        tag = e[0]
        if tag in (CONSTANT_Fieldref, CONSTANT_Methodref, CONSTANT_InterfaceMethodref):
            name, desc = self.name_and_type(e[2])
            kind = {CONSTANT_Fieldref: "Fieldref", CONSTANT_Methodref: "Methodref",
                    CONSTANT_InterfaceMethodref: "InterfaceMethodref"}[tag]
            return {"kind": kind, "owner": self.class_name(e[1]), "name": name, "desc": desc}
        if tag == CONSTANT_Class:
            return {"kind": "Class", "name": self.utf8(e[1])}
        if tag == CONSTANT_String:
            return {"kind": "String", "value": self.utf8(e[1])}
        if tag in (CONSTANT_InvokeDynamic, CONSTANT_Dynamic):
            name, desc = self.name_and_type(e[2])
            return {"kind": "InvokeDynamic" if tag == CONSTANT_InvokeDynamic else "Dynamic",
                    "bootstrap": e[1], "name": name, "desc": desc}
        if tag == CONSTANT_MethodType:
            return {"kind": "MethodType", "desc": self.utf8(e[1])}
        if tag in (CONSTANT_Integer, CONSTANT_Float, CONSTANT_Long, CONSTANT_Double):
            return {"kind": "Const"}
        raise ClassFileError(f"constant {idx} has tag {tag}, not a reference")


# --- annotations (JVMS 4.7.16) ---------------------------------------------------
def _element_value(r: _Reader, cp: _Pool):
    tag = chr(r.u1())
    if tag in "BCDFIJSZs":
        idx = r.u2()
        e = cp.pool[idx]
        if tag == "s":
            return cp.utf8(idx)
        if tag == "Z":
            return bool(e[1])
        if tag in "BCIS":
            return struct.unpack(">i", struct.pack(">I", e[1]))[0]
        if tag == "F":
            return struct.unpack(">f", struct.pack(">I", e[1]))[0]
        if tag == "J":
            return struct.unpack(">q", e[1])[0]
        if tag == "D":
            return struct.unpack(">d", e[1])[0]
    if tag == "e":
        type_name, const_name = cp.utf8(r.u2()), cp.utf8(r.u2())
        return {"enum": type_name, "value": const_name}
    if tag == "c":
        return {"class": cp.utf8(r.u2())}
    if tag == "@":
        return _annotation(r, cp)
    if tag == "[":
        n = r.u2()
        return [_element_value(r, cp) for _ in range(n)]
    raise ClassFileError(f"unknown element_value tag {tag!r}")


def _annotation(r: _Reader, cp: _Pool) -> dict:
    type_desc = cp.utf8(r.u2())
    n = r.u2()
    values = {}
    for _ in range(n):
        name = cp.utf8(r.u2())
        values[name] = _element_value(r, cp)
    return {"type": type_desc, "values": values}


def _annotations_attr(data: bytes, cp: _Pool) -> list[dict]:
    r = _Reader(data)
    n = r.u2()
    return [_annotation(r, cp) for _ in range(n)]


def _param_annotations_attr(data: bytes, cp: _Pool) -> list[list[dict]]:
    r = _Reader(data)
    n = r.u1()
    out = []
    for _ in range(n):
        m = r.u2()
        out.append([_annotation(r, cp) for _ in range(m)])
    return out


# --- code ---------------------------------------------------------------------------
def _decode_code(code: bytes, cp: _Pool) -> list[Instruction]:
    out: list[Instruction] = []
    p, n = 0, len(code)
    while p < n:
        start = p
        op = code[p]
        p += 1
        if op not in OPCODES:
            raise ClassFileError(f"unknown opcode 0x{op:02x} at {start}")
        mnemonic, width = OPCODES[op]
        ref = None
        if width >= 0:
            operand = code[p:p + width]
            p += width
            if mnemonic in REF_OPCODES:
                if mnemonic == "ldc":
                    idx = operand[0]
                else:
                    idx = struct.unpack(">H", operand[:2])[0]
                ref = cp.ref(idx)
        elif mnemonic == "wide":
            sub = code[p]
            p += 1
            p += 4 if sub == 0x84 else 2
            mnemonic = "wide"
        elif mnemonic == "tableswitch":
            pad = (4 - (p % 4)) % 4
            p += pad
            _default = struct.unpack_from(">i", code, p)[0]
            low = struct.unpack_from(">i", code, p + 4)[0]
            high = struct.unpack_from(">i", code, p + 8)[0]
            p += 12 + (high - low + 1) * 4
        elif mnemonic == "lookupswitch":
            pad = (4 - (p % 4)) % 4
            p += pad
            npairs = struct.unpack_from(">i", code, p + 4)[0]
            p += 8 + npairs * 8
        out.append(Instruction(start, op, mnemonic, ref))
    return out


def _read_attributes(r: _Reader, cp: _Pool) -> dict[str, list[bytes]]:
    n = r.u2()
    attrs: dict[str, list[bytes]] = {}
    for _ in range(n):
        name = cp.utf8(r.u2())
        length = r.u4()
        attrs.setdefault(name, []).append(r.bytes(length))
    return attrs


def _member(r: _Reader, cp: _Pool, is_method: bool) -> Member:
    access = r.u2()
    name = cp.utf8(r.u2())
    desc = cp.utf8(r.u2())
    attrs = _read_attributes(r, cp)
    m = Member(name=name, desc=desc, access=access)
    for key in ("RuntimeVisibleAnnotations", "RuntimeInvisibleAnnotations"):
        for blob in attrs.get(key, []):
            m.annotations += _annotations_attr(blob, cp)
    for key in ("RuntimeVisibleParameterAnnotations", "RuntimeInvisibleParameterAnnotations"):
        for blob in attrs.get(key, []):
            m.param_annotations = _param_annotations_attr(blob, cp)
    if "Signature" in attrs:
        m.signature = cp.utf8(struct.unpack(">H", attrs["Signature"][0])[0])
    if "AnnotationDefault" in attrs:
        m.annotation_default = _element_value(_Reader(attrs["AnnotationDefault"][0]), cp)
    if is_method and "Code" in attrs:
        cr = _Reader(attrs["Code"][0])
        cr.u2()  # max_stack
        cr.u2()  # max_locals
        code_len = cr.u4()
        m.code = _decode_code(cr.bytes(code_len), cp)
    return m


def read_class(data: bytes) -> ClassFile:
    r = _Reader(data)
    if r.u4() != 0xCAFEBABE:
        raise ClassFileError("not a class file (bad magic)")
    r.u2()  # minor
    major = r.u2()
    cp = _Pool(_read_pool(r))
    access = r.u2()
    this_name = cp.class_name(r.u2())
    super_idx = r.u2()
    super_name = cp.class_name(super_idx) if super_idx else None
    interfaces = [cp.class_name(r.u2()) for _ in range(r.u2())]
    fields = [_member(r, cp, False) for _ in range(r.u2())]
    methods = [_member(r, cp, True) for _ in range(r.u2())]
    attrs = _read_attributes(r, cp)
    annotations: list[dict] = []
    for key in ("RuntimeVisibleAnnotations", "RuntimeInvisibleAnnotations"):
        for blob in attrs.get(key, []):
            annotations += _annotations_attr(blob, cp)
    signature = cp.utf8(struct.unpack(">H", attrs["Signature"][0])[0]) if "Signature" in attrs else None
    inner = []
    for blob in attrs.get("InnerClasses", []):
        ir = _Reader(blob)
        for _ in range(ir.u2()):
            inner_idx, outer_idx, name_idx, inner_access = ir.u2(), ir.u2(), ir.u2(), ir.u2()
            inner.append({"inner": cp.class_name(inner_idx), "outer": cp.class_name(outer_idx) if outer_idx else None,
                          "name": cp.utf8(name_idx) if name_idx else None, "access": inner_access})
    return ClassFile(name=this_name, super_name=super_name, interfaces=interfaces, access=access, major=major,
                     fields=fields, methods=methods, annotations=annotations, signature=signature, inner_classes=inner)


def read_class_from_jar(jar: Path | zipfile.ZipFile, internal_name: str) -> ClassFile:
    zf = jar if isinstance(jar, zipfile.ZipFile) else zipfile.ZipFile(jar)
    try:
        return read_class(zf.read(internal_name + ".class"))
    finally:
        if not isinstance(jar, zipfile.ZipFile):
            zf.close()


def iter_jar_classes(jar: Path, prefix: str = "") -> Iterator[tuple[str, ClassFile | ClassFileError]]:
    """Yield (internal name, ClassFile or the error that prevented reading it)."""
    with zipfile.ZipFile(jar) as zf:
        for entry in sorted(zf.namelist()):
            if not entry.endswith(".class") or not entry.startswith(prefix) or entry.startswith("META-INF/"):
                continue
            name = entry[:-6]
            try:
                yield name, read_class(zf.read(entry))
            except (ClassFileError, struct.error, IndexError) as exc:
                yield name, ClassFileError(f"{entry}: {exc}")


# --- descriptors -------------------------------------------------------------------
_PRIM = {"B": "byte", "C": "char", "D": "double", "F": "float", "I": "int", "J": "long", "S": "short", "Z": "boolean", "V": "void"}


def parse_descriptor_types(desc: str) -> list[str]:
    """Split a field or method descriptor into its type descriptors (params then return for methods)."""
    out = []
    i = 0
    if desc.startswith("("):
        i = 1
    while i < len(desc):
        c = desc[i]
        if c == ")":
            i += 1
            continue
        start = i
        while desc[i] == "[":
            i += 1
        if desc[i] == "L":
            end = desc.index(";", i)
            i = end + 1
        else:
            i += 1
        out.append(desc[start:i])
    return out


def type_to_java(t: str) -> str:
    dims = 0
    while t.startswith("["):
        dims += 1
        t = t[1:]
    base = _PRIM.get(t) if len(t) == 1 else t[1:-1].replace("/", ".")
    return (base or t) + "[]" * dims


def member_to_java(m: Member) -> str:
    """A javap-like one-line rendering, for display only; the descriptor stays authoritative."""
    mods = []
    for bit, word in ((0x0001, "public"), (0x0002, "private"), (0x0004, "protected"), (0x0008, "static"),
                      (0x0010, "final"), (0x0400, "abstract"), (0x0020, "synchronized"), (0x0100, "native")):
        if m.access & bit:
            mods.append(word)
    prefix = " ".join(mods) + (" " if mods else "")
    if m.desc.startswith("("):
        types = parse_descriptor_types(m.desc)
        params, ret = types[:-1], types[-1]
        if m.name == "<clinit>":
            return prefix + "static {}"
        if m.name == "<init>":
            return prefix + "<init>(" + ", ".join(type_to_java(p) for p in params) + ")"
        return prefix + type_to_java(ret) + " " + m.name + "(" + ", ".join(type_to_java(p) for p in params) + ")"
    return prefix + type_to_java(m.desc) + " " + m.name
