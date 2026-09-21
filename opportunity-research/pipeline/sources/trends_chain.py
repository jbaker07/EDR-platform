"""Place many terms on one relative scale with Google Trends, five at a time, through shared anchors.

Trends returns each term's interest as an integer 0-100 relative to the largest term in the request.
Chaining: every request carries one ANCHOR term whose value is already known on the global scale;
the other four terms are placed by the ratio of their mean to the anchor's mean. To keep rounding
error small, a term that reads below 8 against the current anchor is re-queued with a smaller
anchor (one of the terms already placed lower on the scale). State is persisted so a throttled
session can resume. Cadence is deliberately slow: this address was throttled after a handful of
quick requests (measured), so the default is one request per 90 seconds.

Everything produced is RELATIVE (provider 'google_trends', kind 'relative_interest_chain'), never
a volume. Rounding error per placement is recorded from the observed integer values.
"""
from __future__ import annotations

import json
import statistics
import time
from pathlib import Path

from pipeline import store
from pipeline.sources import trends

ROOT = Path(__file__).resolve().parents[2]
STATE = ROOT / "data" / "trends_chain_state.json"


class Chain:
    def __init__(self, geo: str = "US", cadence_s: float = 90.0) -> None:
        self.geo = geo
        self.cadence = cadence_s
        self.state = json.loads(STATE.read_text()) if STATE.exists() else {"scale": {}, "pending": [], "log": [], "anchors": []}

    def save(self) -> None:
        STATE.write_text(json.dumps(self.state, indent=1))

    def add_terms(self, terms: list[str]) -> None:
        for t in terms:
            if t not in self.state["scale"] and t not in self.state["pending"]:
                self.state["pending"].append(t)
        self.save()

    def set_root(self, term: str, value: float = 100.0) -> None:
        self.state["scale"][term] = {"value": value, "anchor": None, "reads": None, "error_pct": 0.0}
        self.state["anchors"] = [term]
        self.save()

    def _pick_anchor(self, hint: str | None = None) -> str:
        """Prefer a mid-scale anchor that itself read reliably (>= 10 on its own placement, error <= 25%)."""
        if hint and hint in self.state["scale"]:
            return hint
        reliable = [(t, v) for t, v in self.state["scale"].items()
                    if v.get("anchor") is None or ((v.get("reads") or 0) >= 10 and (v.get("error_pct") or 0) <= 25)]
        placed = sorted(reliable, key=lambda kv: kv[1]["value"])
        return placed[len(placed) // 2][0] if placed else self.state["anchors"][0]

    def step(self, anchor: str | None = None) -> dict | None:
        if not self.state["pending"]:
            return None
        last = self.state["log"][-1] if self.state["log"] else None
        if last and "anchor read 0" in str(last.get("error", "")):
            anchor = self.state["anchors"][0]
        anchor = self._pick_anchor(anchor)
        batch = self.state["pending"][:4]
        # a batch made of retried (small) terms is compared against the lowest reliable anchor instead of the median
        tries = self.state.get("tries", {})
        if batch and all(tries.get(t, 0) >= 1 for t in batch):
            low = [(t, v) for t, v in self.state["scale"].items()
                   if v.get("anchor") is None or ((v.get("reads") or 0) >= 10 and (v.get("error_pct") or 0) <= 25)]
            low = [(t, v) for t, v in low if v["value"] >= 3]
            if low:
                anchor = min(low, key=lambda kv: kv[1]["value"])[0]
        series = trends.interest_over_time([anchor] + batch, geo=self.geo)
        rec = {"at": time.time(), "anchor": anchor, "batch": batch, "ok": bool(series)}
        if not series:
            rec["error"] = "no data (throttled or unknown terms)"
            self.state["log"].append(rec)
            self.save()
            return rec
        n = len(batch) + 1
        means = [statistics.mean(p["values"][i] for p in series) for i in range(n)]
        a_mean = means[0]
        a_val = self.state["scale"][anchor]["value"]
        rec["means"] = dict(zip([anchor] + batch, [round(m, 2) for m in means]))
        if a_mean <= 0:
            # the anchor read zero: it is unusable at this scale; mark it and retry the batch against the root
            self.state["scale"][anchor]["unreliable"] = True
            rec["error"] = "anchor read 0; batch retried against the root next step"
            self.state["log"].append(rec)
            self.save()
            return rec
        for term, m in zip(batch, means[1:]):
            self.state["pending"].remove(term)
            tries = self.state.setdefault("tries", {}).get(term, 0) + 1
            self.state["tries"][term] = tries
            if m == 0:
                if tries < 2:
                    self.state["pending"].insert(0, term)  # one retry against a smaller anchor, soon
                    rec.setdefault("requeued", []).append(term)
                else:
                    self.state["scale"][term] = {"value": 0.0, "anchor": anchor, "reads": 0, "anchor_reads": round(a_mean, 2),
                                                 "error_pct": 100.0, "below_scale": True}
                continue
            if m < 8 and tries < 3:
                self.state["pending"].insert(0, term)       # too small here: retry with a smaller anchor, soon
                rec.setdefault("requeued", []).append(term)
                continue
            value = a_val * (m / a_mean)
            # integer rounding on a 0-100 scale: half a unit on each mean, propagated through the ratio
            err = 100 * (0.5 / max(m, 0.5) + 0.5 / max(a_mean, 0.5)) if m > 0 else 100.0
            self.state["scale"][term] = {"value": round(value, 3), "anchor": anchor, "reads": round(m, 2),
                                         "anchor_reads": round(a_mean, 2), "error_pct": round(err, 1)}
        self.state["log"].append(rec)
        self.save()
        return rec

    def run(self, max_steps: int = 10, con=None) -> int:
        done = 0
        while self.state["pending"] and done < max_steps:
            rec = self.step()
            done += 1
            if rec and not rec["ok"]:
                break
            if self.state["pending"]:
                time.sleep(self.cadence)
        if con is not None:
            for term, v in self.state["scale"].items():
                store.add_metric(con, term, "google_trends", "relative_interest_chain", v["value"], geo=self.geo,
                                 extra={"anchor": v["anchor"], "reads": v.get("reads"), "error_pct": v.get("error_pct")})
            con.commit()
        return done
