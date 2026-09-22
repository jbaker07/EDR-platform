"""Cluster lineage across passes: explicit split/merge edges instead of a majority-vote transfer of conclusions.

Cluster ids are content hashes, so they change whenever the store is re-clustered. Rather than silently carrying a
pass-1 conclusion onto whichever later cluster most of its members landed in, this records every edge between passes:
for each old cluster, which new clusters its members went to and how many members each received. Splits (one old ->
many new), merges (many old -> one new) and dissolutions (members no longer clustered) are therefore visible.

The per-pass snapshots are the `*_full_*.csv.gz` exports committed at each pass boundary; `extract()` pulls them out
of git history into data/lineage/ (gitignored, since git already holds them) and `build()` computes the edges.
"""
from __future__ import annotations

import collections
import csv
import gzip
import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "data" / "lineage"
# pass boundary commits (checkpoint --full runs), oldest first
SNAPSHOTS = {"pass1": "7795dba1", "pass2": "a9c4d480", "pass3": "eb661b9a"}


def extract() -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    for name, commit in SNAPSHOTS.items():
        for table in ("clusters", "cluster_members"):
            listing = subprocess.run(["git", "ls-tree", "-r", "--name-only", commit, "--",
                                      "opportunity-research/data/exports/delta/"], cwd=ROOT.parent,
                                     capture_output=True, text=True, check=True).stdout.splitlines()
            path = next(p for p in listing if f"{table}_full_" in p)
            blob = subprocess.run(["git", "show", f"{commit}:{path}"], cwd=ROOT.parent, capture_output=True, check=True).stdout
            (OUT / f"{table}_{name}.csv.gz").write_bytes(blob)


def _members(path: Path) -> dict[str, set[str]]:
    m: dict[str, set[str]] = collections.defaultdict(set)
    with gzip.open(path, "rt", newline="") as fh:
        for row in csv.DictReader(fh):
            if row["cluster_id"] != "_variants":
                m[row["cluster_id"]].add(row["query"])
    return m


def build() -> Path:
    passes = {p: _members(OUT / f"cluster_members_{p}.csv.gz") for p in SNAPSHOTS}
    lineage = {}
    for a, b in zip(list(SNAPSHOTS)[:-1], list(SNAPSHOTS)[1:]):
        inv: dict[str, set[str]] = collections.defaultdict(set)
        for cid, qs in passes[b].items():
            for q in qs:
                inv[q].add(cid)
        edges, stats = [], collections.Counter()
        for cid, qs in passes[a].items():
            ov: collections.Counter = collections.Counter()
            for q in qs:
                for nc in inv.get(q, ()):
                    ov[nc] += 1
            if not ov:
                stats["dissolved"] += 1
                continue
            for nc, n in ov.items():
                edges.append({"from": cid, "to": nc, "shared": n, "from_size": len(qs), "to_size": len(passes[b][nc])})
            stats["split" if len(ov) > 1 else "kept_or_merged"] += 1
        recv = collections.Counter(e["to"] for e in edges)
        stats["merged_targets"] = sum(1 for _, n in recv.items() if n > 1)
        lineage[f"{a}->{b}"] = {"edges": edges, "stats": dict(stats)}
    p = OUT / "cluster_lineage.json.gz"
    with gzip.open(p, "wt") as fh:
        json.dump(lineage, fh)
    (OUT / "lineage_summary.json").write_text(json.dumps({k: v["stats"] for k, v in lineage.items()}, indent=1))
    return p


if __name__ == "__main__":
    if "extract" in sys.argv:
        extract()
    print(build(), json.loads((OUT / "lineage_summary.json").read_text()))
