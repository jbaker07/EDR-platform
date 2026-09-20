"""Phase 4: group kept queries into candidate problem clusters.

Method (auditable, lexical, deliberately simple): each query -> canonical token set; queries are
joined when their token overlap (Jaccard) is at or above a threshold, with the intent words
removed so 'fix X' and 'how to X' land together. Union-find gives components; each component is
labelled by its most frequent tokens and its dominant intent. This is a PRE-clustering: the
analyst (or a model) reviews and merges/splits components before any demand is aggregated, and
the review decision is recorded in clusters.method/confidence.
"""
from __future__ import annotations

import re
from collections import Counter, defaultdict

from pipeline import store
from pipeline.clean import canonical_key

INTENT = [
    ("fix", r"\b(not working|won'?t|can'?t|doesn'?t|isn'?t|error|fix|stuck|broken|fail|crash|problem|issue|missing|freez|lag)\b"),
    ("how_to", r"^(how (to|do|can)|tutorial|guide|setup|set up|install|make|create|build)\b|\b(tutorial|how to)\b"),
    ("compare", r"\b(vs|versus|or|compared|comparison|difference between|better than)\b"),
    ("decide", r"\b(best|which|should i|recommend|top \d+|worth it)\b"),
    ("compatibility", r"\b(compatib|work with|works with|support(s|ed)? |fit|fits)\b"),
    ("alternative", r"\b(alternative|instead of|free|open source|like)\b"),
    ("calculate", r"\b(calculator|how much|how many|cost|price|estimate|calculate|size|per)\b"),
    ("generate", r"\b(generator|generate|random|template|maker|create a)\b"),
    ("convert", r"\b(convert|converter|to (pdf|mp4|mp3|png|jpg|svg|stl|obj|fbx|gltf)|export|import)\b"),
    ("find", r"\b(where|find|list|database|download|near|source)\b"),
    ("identify", r"\b(what is (this|my)|identify|what kind|which kind|what type)\b"),
    ("optimize", r"\b(faster|slow|speed up|optimi[sz]e|performance|improve|cheaper|reduce)\b"),
    ("track", r"\b(changelog|changes|update|new version|patch notes|what'?s new|latest)\b"),
    ("plan", r"\b(plan|planner|schedule|checklist|roadmap|steps)\b"),
]


def intent_of(q: str) -> str:
    for name, pat in INTENT:
        if re.search(pat, q):
            return name
    return "informational"


class UF:
    def __init__(self, n):
        self.p = list(range(n))

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra != rb:
            self.p[rb] = ra


def _components(qs: list[str], keys: list[set[str]], idf: dict[str, float], threshold: float, min_shared: int) -> list[list[int]]:
    """Union-find over IDF-weighted Jaccard; a link needs at least `min_shared` shared tokens."""
    index: dict[str, list[int]] = defaultdict(list)
    for i, ks in enumerate(keys):
        for t in ks:
            index[t].append(i)
    uf = UF(len(qs))
    for i, ks in enumerate(keys):
        if not ks:
            continue
        cands: dict[int, float] = defaultdict(float)
        shared_n: Counter = Counter()
        for t in ks:
            w = idf.get(t, 1.0)
            for j in index[t]:
                if j > i:
                    cands[j] += w
                    shared_n[j] += 1
        for j, shared_w in cands.items():
            if shared_n[j] < min_shared:
                continue
            union_w = sum(idf.get(t, 1.0) for t in (ks | keys[j]))
            if union_w and shared_w / union_w >= threshold:
                uf.union(i, j)
    groups: dict[int, list[int]] = defaultdict(list)
    for i in range(len(qs)):
        groups[uf.find(i)].append(i)
    return list(groups.values())


def run(con, threshold: float = 0.5, min_size: int = 3, domain: str | None = None, min_shared: int = 2,
        max_component: int = 80, ladder=(0.6, 0.7, 0.8, 0.9)) -> dict:
    """IDF-weighted single linkage, then any component larger than `max_component` is re-split with the
    next threshold on the ladder, so one polysemous token ('engine', 'code') cannot chain topics together.
    Each cluster records the threshold it was formed at."""
    where = "WHERE status='kept'" + (" AND domain=?" if domain else "")
    rows = con.execute(f"SELECT query, domain, subdomain FROM queries {where}", (domain,) if domain else ()).fetchall()
    qs = [r[0] for r in rows]
    keys = [set(canonical_key(q).split()) for q in qs]
    import math
    df = Counter(t for ks in keys for t in ks)
    n = max(len(qs), 1)
    idf = {t: math.log(n / c) + 1.0 for t, c in df.items()}
    final: list[tuple[list[int], float]] = []
    work = [(list(range(len(qs))), threshold)]
    while work:
        members, th = work.pop()
        sub_qs = [qs[i] for i in members]
        sub_keys = [keys[i] for i in members]
        for comp in _components(sub_qs, sub_keys, idf, th, min_shared):
            real = [members[i] for i in comp]
            nxt = next((l for l in ladder if l > th), None)
            if len(real) > max_component and nxt is not None:
                work.append((real, nxt))
            else:
                final.append((real, th))
    con.execute("DELETE FROM cluster_members WHERE cluster_id != '_variants'")
    con.execute("DELETE FROM clusters WHERE method LIKE 'lexical%'")
    n_clusters, oversized = 0, 0
    for members, th in final:
        if len(members) < min_size:
            continue
        if len(members) > max_component:
            oversized += 1
        toks = Counter(t for i in members for t in keys[i])
        # label by the highest-IDF frequent tokens, so 'navmesh agent' beats 'unity'
        label = " ".join(t for t, _ in sorted(toks.items(), key=lambda kv: -(kv[1] * idf.get(kv[0], 1.0)))[:4])
        intents = Counter(intent_of(qs[i]) for i in members)
        dom = Counter(rows[i][1] for i in members).most_common(1)[0][0]
        cid = f"c_{abs(hash(label + str(len(members)))) % 10**8:08d}"
        con.execute("INSERT OR REPLACE INTO clusters(cluster_id, label, description, intent, domain, method, confidence, created_at) VALUES (?,?,?,?,?,?,?,?)",
                    (cid, label, f"{len(members)} queries; intents {dict(intents.most_common(3))}; formed at threshold {th}",
                     intents.most_common(1)[0][0], dom, f"lexical_idf_jaccard@{th}", "unreviewed", store.now()))
        for i in members:
            con.execute("INSERT OR REPLACE INTO cluster_members(cluster_id, query, canonical_key, is_variant_of) VALUES (?,?,?,?)",
                        (cid, qs[i], " ".join(sorted(keys[i])), None))
        n_clusters += 1
    con.commit()
    return {"queries": len(qs), "clusters": n_clusters, "still_oversized": oversized,
            "singletons_or_small": sum(1 for m, _ in final if len(m) < min_size)}
