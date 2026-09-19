"""The Sims 4: DBPF ``.package`` files and ``.ts4script`` script mods.

Two things matter for Sims 4 compatibility and both are mechanically checkable:

* which *resource keys* (type, group, instance) a package overrides -- two
  packages overriding the same key is the ecosystem's actual conflict, not two
  packages merely existing;
* which Python version a script mod was compiled for -- the game embeds one
  specific CPython, and a mismatched ``.pyc`` magic simply will not load.
"""
from __future__ import annotations

import struct
import zipfile
from pathlib import Path

from .base import Artifact, Inspection, InspectionError, zip_entries

DBPF_MAGIC = b"DBPF"

# CPython bytecode magic numbers, little-endian u16 at the start of a .pyc.
# Source: CPython Lib/importlib/_bootstrap_external.py MAGIC_NUMBER history.
PYC_MAGIC = {
    3379: "3.6", 3390: "3.7", 3391: "3.7", 3392: "3.7", 3393: "3.7", 3394: "3.7",
    3400: "3.8", 3401: "3.8", 3410: "3.8", 3411: "3.8", 3412: "3.8", 3413: "3.8",
    3420: "3.9", 3421: "3.9", 3422: "3.9", 3423: "3.9", 3424: "3.9", 3425: "3.9",
    3430: "3.10", 3431: "3.10", 3432: "3.10", 3433: "3.10", 3434: "3.10", 3435: "3.10",
    3436: "3.10", 3437: "3.10", 3438: "3.10", 3439: "3.10",
    3450: "3.11", 3451: "3.11", 3452: "3.11", 3453: "3.11", 3454: "3.11", 3455: "3.11",
    3456: "3.11", 3457: "3.11", 3458: "3.11", 3459: "3.11", 3460: "3.11", 3461: "3.11",
    3462: "3.11", 3463: "3.11", 3464: "3.11", 3465: "3.11", 3466: "3.11", 3467: "3.11",
    3468: "3.11", 3469: "3.11", 3470: "3.11", 3471: "3.11", 3472: "3.11", 3473: "3.11",
    3474: "3.11", 3475: "3.11", 3476: "3.11", 3477: "3.11", 3478: "3.11", 3479: "3.11",
    3480: "3.12", 3481: "3.12", 3482: "3.12", 3483: "3.12", 3484: "3.12", 3485: "3.12",
    3486: "3.12", 3487: "3.12", 3488: "3.12", 3489: "3.12", 3490: "3.12", 3491: "3.12",
    3492: "3.12", 3493: "3.12", 3494: "3.12", 3495: "3.12",
    3500: "3.13", 3501: "3.13", 3502: "3.13", 3503: "3.13", 3504: "3.13", 3505: "3.13",
    3506: "3.13", 3507: "3.13", 3508: "3.13", 3509: "3.13", 3510: "3.13", 3511: "3.13",
    3512: "3.13", 3513: "3.13", 3514: "3.13", 3515: "3.13", 3516: "3.13", 3517: "3.13",
    3518: "3.13", 3519: "3.13", 3520: "3.13", 3521: "3.13", 3522: "3.13", 3523: "3.13",
    3524: "3.13", 3525: "3.13", 3526: "3.13", 3527: "3.13", 3528: "3.13", 3529: "3.13",
    3530: "3.13", 3531: "3.13",
}


def parse_dbpf(path: str | Path, max_entries: int = 20000) -> dict:
    """Parse a DBPF v2.x header and its resource index."""
    path = Path(path)
    file_size = path.stat().st_size
    with path.open("rb") as fh:
        header = fh.read(96)
        if len(header) < 96 or header[:4] != DBPF_MAGIC:
            raise InspectionError(f"not a DBPF package: magic is {header[:4]!r}")
        # Offsets per the SimsWiki DBPF 2.0 header table:
        #   0x24 index entry count, 0x2C index size, 0x3C index_version ("always
        #   3"), 0x40 index position.
        major, minor = struct.unpack_from("<II", header, 4)
        index_count, = struct.unpack_from("<I", header, 0x24)
        index_size, = struct.unpack_from("<I", header, 0x2C)
        index_version, = struct.unpack_from("<I", header, 0x3C)
        index_offset, = struct.unpack_from("<I", header, 0x40)
        if major != 2:
            raise InspectionError(f"DBPF major version {major} is not the Sims 3/4 layout")
        if index_count == 0:
            return {"major": major, "minor": minor, "index_version": index_version,
                    "entry_count": 0, "entries": []}
        if index_offset == 0 or index_size == 0:
            raise InspectionError("DBPF index offset/size is zero")

        fh.seek(index_offset)
        index = fh.read(index_size)
    if len(index) < 4:
        raise InspectionError("DBPF index truncated")

    entries, pos, convention = _read_index_entries(index, index_count, max_entries,
                                                  file_size)

    return {"major": major, "minor": minor, "index_version": index_version,
            "entry_count": index_count, "entries": entries,
            "index_bytes": len(index), "index_bytes_consumed": pos,
            "index_fully_consumed": pos == len(index),
            "entry_trailing_convention": convention}


# The last index-entry field is documented two ways, and they differ only for
# UNCOMPRESSED entries:
#
#   "dword"  -- the SimsWiki DBPF description gives field 7 as a full DWORD in
#               every entry: compression type (low word) + committed (high word).
#   "s4tk"   -- S4TK, the maintained Sims 4 implementation, reads the compression
#               type only when the size high bit is set, then always 2 more bytes.
#
# Real Sims 4 resources are compressed, where both agree on 4 bytes, so no
# artifact we can obtain settles it. Rather than pick one and silently misread
# packages following the other, both are attempted and the one that consumes the
# index exactly is used. Which one matched is reported.
TRAILING_CONVENTIONS = ("s4tk", "dword")


class IndexAmbiguous(InspectionError):
    """More than one candidate layout is valid and they disagree.

    Reported rather than resolved: picking one would manufacture certainty the
    bytes do not support, and every downstream conclusion that needs a unique
    interpretation is suppressed.
    """


def _entry_problems(entries: list[dict], declared_count: int,
                    file_size: int | None) -> list[str]:
    """Validity checks beyond "the stride consumed the index exactly".

    Consuming the index is necessary but not sufficient: a wrong stride can
    still land exactly on the boundary, as one does here for a two-entry
    package. Only genuine invariants are asserted -- the declared entry count,
    and every entry lying inside the file. Notably NOT asserted: that a
    compressed entry is smaller than its decompressed size. Compressing ten
    bytes can produce eighteen, and treating that as invalid rejected real
    packages written by the reference implementation.
    """
    problems = []
    if len(entries) != declared_count:
        problems.append(f"read {len(entries)} entries, header declares {declared_count}")
    for i, entry in enumerate(entries):
        if file_size is not None:
            if entry["offset"] <= 0 or entry["offset"] > file_size:
                problems.append(f"entry {i} offset {entry['offset']} outside the file")
            elif entry["offset"] + entry["size"] > file_size:
                problems.append(f"entry {i} extends past the end of the file")
    return problems


def _read_index_entries(index: bytes, index_count: int, max_entries: int,
                        file_size: int | None = None) -> tuple[list[dict], int, str]:
    """Choose among bounded candidate layouts, explicitly.

    Exactly one valid candidate  -> use it, naming the layout.
    Several valid and agreeing   -> use them, keeping the encoding unresolved.
    Several valid and disagreeing-> raise IndexAmbiguous; choose nothing.
    None valid                   -> raise InspectionError; malformed or unsupported.
    """
    valid: list[tuple[list[dict], int, str]] = []
    rejected: dict[str, str] = {}
    for convention in TRAILING_CONVENTIONS:
        try:
            entries, pos = _read_index_with(index, index_count, max_entries, convention)
        except (struct.error, IndexError) as exc:
            rejected[convention] = f"could not be parsed ({exc})"
            continue
        if pos != len(index):
            rejected[convention] = (
                f"consumed {pos} of {len(index)} index bytes")
            continue
        problems = _entry_problems(entries, index_count, file_size)
        if problems:
            rejected[convention] = "; ".join(problems[:3])
            continue
        valid.append((entries, pos, convention))

    if not valid:
        detail = "; ".join(f"{k}: {v}" for k, v in rejected.items())
        raise InspectionError(
            f"DBPF index matches no known entry layout ({detail})")

    if len(valid) == 1:
        return valid[0]

    # Several valid. Agreement is judged on what downstream actually uses: the
    # resource keys.
    def keys(entry_list: list[dict]) -> list[tuple[int, int, int]]:
        return [(e["type"], e["group"], e["instance"]) for e in entry_list]

    first_keys = keys(valid[0][0])
    if all(keys(candidate[0]) == first_keys for candidate in valid[1:]):
        names = "+".join(c[2] for c in valid)
        return valid[0][0], valid[0][1], f"ambiguous-but-equivalent({names})"

    raise IndexAmbiguous(
        "DBPF index is valid under more than one entry layout and they disagree "
        f"({', '.join(c[2] for c in valid)}); refusing to choose")


def _read_index_with(index: bytes, index_count: int, max_entries: int,
                     convention: str) -> tuple[list[dict], int]:
    flags, = struct.unpack_from("<I", index, 0)
    pos = 4
    # Bits 0..2 hoist a constant Type/Group/InstanceHi out of every entry.
    constants: dict[str, int] = {}
    for bit, name in ((0x1, "type"), (0x2, "group"), (0x4, "instance_hi")):
        if flags & bit:
            constants[name], = struct.unpack_from("<I", index, pos)
            pos += 4

    entries: list[dict] = []
    for _ in range(min(index_count, max_entries)):
        fields = {}
        for name in ("type", "group", "instance_hi"):
            if name in constants:
                fields[name] = constants[name]
            else:
                fields[name], = struct.unpack_from("<I", index, pos)
                pos += 4
        fields["instance_lo"], = struct.unpack_from("<I", index, pos)
        pos += 4
        position, raw_size, decompressed = struct.unpack_from("<III", index, pos)
        pos += 12
        compressed = bool(raw_size & 0x80000000)
        pos += 4 if (convention == "dword" or compressed) else 2
        entries.append({
            "type": fields["type"],
            "group": fields["group"],
            "instance": (fields["instance_hi"] << 32) | fields["instance_lo"],
            "size": raw_size & 0x7FFFFFFF,
            "decompressed_size": decompressed,
            "compressed": compressed,
            "offset": position,
        })
    return entries, pos


def _resource_key(entry: dict) -> str:
    return f"{entry['type']:08X}:{entry['group']:08X}:{entry['instance']:016X}"


def inspect_package(artifact: Artifact) -> Inspection:
    try:
        parsed = parse_dbpf(artifact.path)
    except IndexAmbiguous as exc:
        # Every conclusion that needs a unique interpretation is withheld: no
        # resource keys means the collision analyzer has nothing to work from.
        ins = artifact.base_inspection("sims4_package")
        ins.game = "sims4"
        ins.loader = "sims4_package"
        ins.add("mod_id", artifact.path.name, "extracted", "filename")
        ins.add("index_layout_ambiguous", True, "extracted", "DBPF index")
        ins.warnings.append(str(exc))
        ins.checked = ["DBPF header"]
        ins.not_checked = [
            "resource keys: the index is valid under more than one entry layout and "
            "they disagree, so no key list is reported",
            "anything derived from resource keys, including same-key collisions",
        ]
        return ins

    ins = artifact.base_inspection("sims4_package")
    ins.game = "sims4"
    ins.loader = "sims4_package"
    loc = "DBPF index"
    ins.add("mod_id", artifact.path.name, "extracted", "filename")
    ins.add("package_version", f"{parsed['major']}.{parsed['minor']}", "extracted", "DBPF header")
    ins.add("index_version", parsed["index_version"], "extracted", "DBPF header")
    ins.add("entry_trailing_convention", parsed["entry_trailing_convention"],
            "extracted", "DBPF index")
    if parsed["index_version"] != 3:
        ins.warnings.append(
            f"DBPF index_version is {parsed['index_version']}, not 3. The format "
            "specification gives this field as always 3, and other readers reject "
            "packages where it is not, so this package may be malformed.")
    ins.add("resource_count", parsed["entry_count"], "extracted", "DBPF header")
    keys = [_resource_key(e) for e in parsed["entries"]]
    ins.add("resource_keys", keys[:20000], "extracted", loc)
    type_counts: dict[str, int] = {}
    for e in parsed["entries"]:
        type_counts[f"{e['type']:08X}"] = type_counts.get(f"{e['type']:08X}", 0) + 1
    ins.add("resource_types", dict(sorted(type_counts.items(), key=lambda kv: -kv[1])),
            "extracted", loc)
    if parsed["entry_count"] > len(parsed["entries"]):
        ins.warnings.append(
            f"index declares {parsed['entry_count']} resources; read {len(parsed['entries'])}")
    if not parsed["index_fully_consumed"]:
        # Entry stride did not account for the whole index. Reported rather than
        # ignored: it means the keys after the first may be misaligned.
        ins.warnings.append(
            f"DBPF index is {parsed['index_bytes']} bytes but parsing consumed "
            f"{parsed['index_bytes_consumed']}; the entry layout does not match this "
            "package, so resource keys beyond the first may be wrong")

    ins.checked = ["DBPF header", "resource index: type/group/instance keys and sizes"]
    ins.not_checked = [
        "resource payloads (tuning XML, SimData, meshes) are not decoded",
        "which game tuning each overridden instance corresponds to",
        "whether the package targets the installed game patch",
    ]
    return ins


def inspect_script(artifact: Artifact) -> Inspection:
    entries = zip_entries(artifact.path)
    ins = artifact.base_inspection("sims4_script")
    ins.game = "sims4"
    ins.loader = "sims4_script"
    ins.entries = entries
    ins.add("mod_id", artifact.path.stem, "extracted", "filename")

    pyc = [e for e in entries if e.lower().endswith(".pyc")]
    py = [e for e in entries if e.lower().endswith(".py")]
    ins.add("pyc_count", len(pyc), "extracted", "archive layout")
    ins.add("py_count", len(py), "extracted", "archive layout")

    versions: dict[str, int] = {}
    unknown_magics: set[int] = set()
    with zipfile.ZipFile(artifact.path) as zf:
        for member in pyc[:200]:
            with zf.open(member) as fh:
                head = fh.read(4)
            if len(head) < 4:
                continue
            magic = struct.unpack("<H", head[:2])[0]
            version = PYC_MAGIC.get(magic)
            if version is None:
                unknown_magics.add(magic)
                continue
            versions[version] = versions.get(version, 0) + 1
    ins.add("bytecode_python_versions", versions, "extracted", ".pyc magic numbers")
    if unknown_magics:
        ins.warnings.append(
            f"unrecognised .pyc magic numbers: {sorted(unknown_magics)}; "
            "the compiled Python version could not be established")
    if len(versions) > 1:
        ins.warnings.append(
            f"script mod mixes bytecode from several Python versions: {sorted(versions)}")

    ins.checked = ["archive layout", ".pyc magic numbers"]
    ins.not_checked = [
        "what the Python code injects or overrides",
        "whether the targeted game tuning still exists",
    ]
    return ins


def inspect(artifact: Artifact) -> Inspection:
    suffix = artifact.path.suffix.lower()
    if suffix == ".package":
        return inspect_package(artifact)
    if suffix == ".ts4script":
        return inspect_script(artifact)
    raise InspectionError(f"unsupported Sims 4 artifact suffix {suffix!r}")


def detect(path: Path) -> bool:
    if path.is_dir():
        return False
    suffix = path.suffix.lower()
    if suffix == ".ts4script":
        return zipfile.is_zipfile(path)
    if suffix == ".package":
        try:
            with path.open("rb") as fh:
                return fh.read(4) == DBPF_MAGIC
        except OSError:
            return False
    return False
