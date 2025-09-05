from __future__ import annotations
from typing import Dict, Any, List
from pathlib import Path
from .config import Paths, load_json
from .schema import Event

class Ontology:
    def __init__(self, p: Paths):
        self.paths = p
        self.core = load_json(p.core_ontology)
        self.anchors = load_json(p.anchor_defs)
        self.semantic = load_json(p.semantic_tags)
        self.inverse = load_json(p.inverse_edges)

        # quick lookups
        self._tag_rules = self.semantic.get("tag_rules", [])  # optional structure

    def tag_event(self, ev: Event) -> None:
        """Add ontology-based semantic tags (cheap heuristic matching)."""
        # Minimal impl: path prefixes and process names from semantic_tags.json
        path_rules: List[Dict[str, Any]] = self.semantic.get("path_prefix_tags", [])
        comm_rules: List[Dict[str, Any]] = self.semantic.get("comm_tags", [])

        if ev.path:
            p = ev.path
            for r in path_rules:
                pref = r.get("prefix")
                tag = r.get("tag")
                if pref and tag and p.startswith(pref):
                    if tag not in ev.tags:
                        ev.tags.append(tag)

        if ev.comm:
            c = ev.comm
            for r in comm_rules:
                name = r.get("comm")
                tag = r.get("tag")
                if name and tag and c == name and tag not in ev.tags:
                    ev.tags.append(tag)
