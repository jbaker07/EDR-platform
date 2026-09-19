"""Fallback inspection: identity and layout only, with no ecosystem claims."""
from __future__ import annotations

import zipfile
from pathlib import Path

from .base import Artifact, Inspection, zip_entries


def inspect(artifact: Artifact) -> Inspection:
    ins = artifact.base_inspection("unknown")
    ins.add("mod_id", artifact.path.name, "extracted", "filename")
    if not artifact.path.is_dir() and zipfile.is_zipfile(artifact.path):
        ins.entries = zip_entries(artifact.path)
        ins.add("entry_count", len(ins.entries), "extracted", "archive layout")
    ins.checked = ["file identity (sha256, size)", "archive layout where the file is a zip"]
    ins.not_checked = ["everything ecosystem-specific: no loader manifest was recognised"]
    ins.warnings.append(
        "no ModCheck inspector recognised this artifact; only its identity is established")
    return ins


def detect(path: Path) -> bool:
    return True
