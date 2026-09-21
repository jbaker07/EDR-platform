"""Phase 2: expand seed terms into a keyword dataset with autocomplete.

For each seed: the bare term, the term with intent prefixes/suffixes (how-to, fix, compare,
find, calculate, generate, convert, compatibility, alternative, optimise, track, plan),
and an alphabet/number soup ("<seed> a" ... "<seed> z"). Every suggestion is stored with its
source, probe string and rank, so a later reader can see why a query is in the dataset.
Depth-2 expansion re-probes the suggestions themselves, capped per seed.
"""
from __future__ import annotations

import string

from pipeline import store
from pipeline.sources import suggest

INTENT_PROBES = {
    "how_to": ["how to {s}", "how do i {s}", "how to use {s}", "{s} tutorial"],
    "fix": ["{s} not working", "{s} won't", "{s} error", "fix {s}", "why is {s}", "{s} problem", "{s} stuck"],
    "find": ["where to find {s}", "{s} list", "{s} database", "best {s}"],
    "identify": ["what is this {s}", "identify {s}", "what {s} do i have"],
    "compare": ["{s} vs", "{s} or", "{s} compared"],
    "decide": ["which {s}", "what {s} should i", "{s} for beginners", "{s} recommendation"],
    "compatibility": ["{s} compatible with", "does {s} work with", "{s} compatibility"],
    "alternative": ["{s} alternative", "free {s}", "{s} free alternative", "open source {s}"],
    "optimize": ["{s} faster", "{s} slow", "{s} performance", "improve {s}", "{s} cheaper"],
    "calculate": ["{s} calculator", "how much {s}", "how many {s}", "{s} cost"],
    "generate": ["{s} generator", "generate {s}", "random {s}", "{s} template"],
    "convert": ["convert {s}", "{s} to", "{s} converter"],
    "track": ["{s} changes", "{s} changelog", "{s} update", "{s} new version"],
    "plan": ["{s} plan", "{s} schedule", "{s} checklist", "{s} guide"],
}
SOUP = list(string.ascii_lowercase) + list("0123456789")


def probes_for(seed: str, soup: bool = True) -> list[tuple[str, str]]:
    out = [("bare", seed)]
    for intent, pats in INTENT_PROBES.items():
        out += [(intent, p.format(s=seed)) for p in pats]
    if soup:
        out += [("soup", f"{seed} {c}") for c in SOUP]
    return out


def expand_seed(con, seed: str, *, domain: str, subdomain: str, sources=("google", "youtube", "bing"),
                soup: bool = True, depth2_cap: int = 40, hl: str = "en", gl: str = "us") -> dict:
    """Returns counts. Depth 1: probes of the seed. Depth 2: the top depth-1 results re-probed bare."""
    if con.execute("SELECT 1 FROM queries WHERE seed=? AND depth=2 LIMIT 1", (seed,)).fetchone():
        return {"seed": seed, "skipped": "already expanded"}
    new, seen = 0, 0
    found: dict[str, int] = {}
    for intent, probe in probes_for(seed, soup):
        for src in sources:
            fn = suggest.SOURCES[src]
            results = fn(probe, hl=hl, gl=gl) if src == "google" else fn(probe)
            for r in results:
                q = store.norm(r["query"])
                if len(q) < 4:
                    continue
                if store.add_query(con, q, domain=domain, subdomain=subdomain, seed=seed, depth=1):
                    new += 1
                seen += 1
                found[q] = found.get(q, 0) + 1
                store.add_observation(con, q, f"suggest:{src}", probe, r.get("rank"),
                                      {"intent_probe": intent, "relevance": r.get("relevance"), "type": r.get("type")})
        con.commit()
    # depth 2: the most-corroborated depth-1 queries become bare probes, but only those that still
    # share a canonical token with the seed (the pilot showed depth 2 is where drift enters)
    from pipeline.clean import canonical_key
    seed_toks = set(canonical_key(seed).split())
    on_topic = {q: n for q, n in found.items() if seed_toks & set(canonical_key(q).split())}
    for q, _ in sorted(on_topic.items(), key=lambda kv: -kv[1])[:depth2_cap]:
        for src in sources:
            fn = suggest.SOURCES[src]
            results = fn(q, hl=hl, gl=gl) if src == "google" else fn(q)
            for r in results:
                q2 = store.norm(r["query"])
                if len(q2) < 4:
                    continue
                if store.add_query(con, q2, domain=domain, subdomain=subdomain, seed=seed, depth=2):
                    new += 1
                seen += 1
                store.add_observation(con, q2, f"suggest:{src}", q, r.get("rank"), {"intent_probe": "depth2", "relevance": r.get("relevance")})
        con.commit()
    return {"seed": seed, "new_queries": new, "suggestions_seen": seen}
