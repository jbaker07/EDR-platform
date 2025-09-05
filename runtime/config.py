from __future__ import annotations
from dataclasses import dataclass
from typing import Optional
from pathlib import Path
import json
import os

ONT_ROOT = Path(os.getenv("EDR_ONTOLOGY_DIR", "ontology")).resolve()

@dataclass
class Paths:
    ontology_dir: Path = ONT_ROOT
    detection_rules: Path = ONT_ROOT / "detection_rules.json"
    anchor_defs: Path = ONT_ROOT / "anchor_definitions.json"
    core_ontology: Path = ONT_ROOT / "core_ontology.json"
    semantic_tags: Path = ONT_ROOT / "semantic_tags.json"
    inverse_edges: Path = ONT_ROOT / "inverse_edges.json"

def load_json(path: Path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)
