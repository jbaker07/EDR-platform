"""Ingest authored competitor records (data/competitors/<cluster_id>.yaml) into the competitors table.

Records are replaced per cluster on each import, so a re-run after editing a file is idempotent.
Evidence quotes and the cluster synthesis are stored as JSON in the evidence column.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from pipeline import store  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
FIELDS = ("solves", "does_not_solve", "workflow_remaining", "business_model", "currency", "coverage", "dominates_serp",
          "data_reproducible", "ai_replaceable", "why_still_searching")


def _flag(v) -> int | None:
    if isinstance(v, bool):
        return int(v)
    s = str(v).strip().lower()
    return 1 if s == "yes" else 0 if s == "no" else None


def import_all(con, folder: Path = ROOT / "data" / "competitors") -> dict:
    out = {}
    for path in sorted(folder.glob("c_*.yaml")):
        doc = yaml.safe_load(path.read_text())
        cid = doc["cluster_id"]
        con.execute("DELETE FROM competitors WHERE cluster_id=?", (cid,))
        synth = doc.get("synthesis", {})
        for c in doc.get("competitors", []):
            ev = {"quotes": c.get("evidence", []), "fetch_status": c.get("fetch_status"), "kind": c.get("kind"),
                  "evidence_basis": doc.get("evidence_basis"), "synthesis": synth}
            con.execute("""INSERT INTO competitors (cluster_id, name, url, kind, solves, does_not_solve, workflow_remaining, business_model,
                           currency, coverage, dominates_serp, needs_install, needs_account, data_reproducible, ai_replaceable,
                           why_still_searching, evidence, observed_at) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)""",
                        (cid, c.get("name"), c.get("url"), c.get("kind"), *[str(c.get(f, "")) for f in FIELDS[:7]],
                         _flag(c.get("needs_install")), _flag(c.get("needs_account")), *[str(c.get(f, "")) for f in FIELDS[7:]],
                         json.dumps(ev), str(doc.get("analyst_date", store.now()))))
        out[cid] = {"head_term": doc.get("head_term"), "competitors": len(doc.get("competitors", [])),
                    "fetched": doc.get("pages_fetched"), "failed": doc.get("pages_failed")}
    con.commit()
    return out


if __name__ == "__main__":
    con = store.connect()
    print(json.dumps(import_all(con), indent=1))
