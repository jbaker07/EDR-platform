"""Phase 3: classify queries we should not treat as productizable search problems.

Rules are deliberately conservative and every decision is stored in queries.status so it can be
audited or reversed. Nothing is deleted. Categories:
  navigational      the user wants a known site/app/login/download
  celebrity         a person's age, net worth, partner, height...
  news              dated or event-bound
  adult             adult content
  variant           near-duplicate of another kept query (canonical key identical)
  off_topic         shares no canonical token with its seed or its subdomain's seed vocabulary (drift)
  non_productizable lyrics, meanings of a single word, translations of a phrase, etc.
"""
from __future__ import annotations

import re

NAV = re.compile(r"\b(login|log in|sign in|sign up|download apk|app store|play store|official site|customer service|phone number|near me|hours|coupon|promo code|track(ing)? (my )?order|apk)\b")
CELEB = re.compile(r"\b(net worth|girlfriend|boyfriend|husband|wife|dating|height|age|died|death|cause of death|nude|leaked)\b")
NEWS = re.compile(r"\b(20(1|2)\d|today|yesterday|breaking|live score|election|schedule tonight|release date|leak(ed)?)\b")
ADULT = re.compile(r"\b(porn|xxx|sex|nsfw|hentai|onlyfans|escort)\b")
NONPROD = re.compile(r"\b(lyrics|meaning in (hindi|tamil|telugu|urdu|spanish)|synonym|antonym|pronunciation|translate to)\b")
# Intent and generic words are removed from the canonical key so clusters form around the TOPIC;
# intent is classified separately from the raw query (cluster.intent_of).
GENERIC = {"best", "better", "than", "faster", "fast", "slow", "slower", "free", "cheap", "cheaper", "cheapest", "vs", "versus",
           "alternative", "alternatives", "fix", "fixes", "fixing", "error", "errors", "problem", "problems", "issue", "issues",
           "working", "work", "works", "tutorial", "tutorials", "guide", "guides", "calculator", "generator", "template", "templates",
           "list", "top", "new", "latest", "update", "updates", "updated", "review", "reviews", "price", "prices", "cost", "costs",
           "near", "online", "download", "pdf", "app", "apps", "software", "tool", "tools", "tips", "ideas", "idea", "examples",
           "example", "meaning", "definition", "difference", "differences", "between", "without", "per", "need", "needed", "much",
           "many", "long", "good", "bad", "easy", "hard", "simple", "quick", "way", "ways", "used", "using", "use", "uses", "type",
           "types", "kind", "kinds", "explained", "explain", "beginner", "beginners", "compatible", "compatibility", "worth", "reddit",
           "youtube", "video", "videos", "2024", "2025", "2026", "year", "month", "week", "day", "days", "hours", "minutes", "vs.",
           "one", "two", "three", "first", "last", "next", "all", "any", "every", "some", "more", "most", "less", "least", "very",
           "really", "still", "also", "just", "only", "own", "same", "different", "other", "another", "each", "both"}
STOP = {"the", "a", "an", "to", "of", "in", "on", "for", "with", "and", "or", "is", "are", "my", "your", "i", "how", "do", "does",
        "what", "why", "which", "when", "where", "can", "cant", "can't", "wont", "won't", "not", "no", "it", "its", "this", "that",
        "at", "by", "from", "into", "vs", "versus", "be", "should", "you", "me", "up", "out", "get", "make", "use", "using"}


def canonical_key(q: str, drop_generic: bool = False) -> str:
    """Order-free, stopword-free, crudely stemmed token key; equal keys are variants of one search.
    With drop_generic=True (the TOPIC key used for clustering) intent and generic words are removed too,
    so 'best X' and 'fix X' share a topic but remain distinct searches."""
    toks = re.findall(r"[a-z0-9+#.']+", q.lower())
    out = []
    for t in toks:
        if t in STOP or (drop_generic and (t in GENERIC or t.isdigit())):
            continue
        t = t.replace("'", "")
        if len(t) > 4 and t.endswith("ies"):
            t = t[:-3] + "y"
        elif len(t) > 3 and t.endswith("es") and not t.endswith("ses"):
            t = t[:-2]
        elif len(t) > 3 and t.endswith("s") and not t.endswith("ss"):
            t = t[:-1]
        elif len(t) > 5 and t.endswith("ing"):
            t = t[:-3]
        elif len(t) > 4 and t.endswith("ed"):
            t = t[:-2]
        out.append(t)
    return " ".join(sorted(set(out)))


def topic_key(q: str) -> str:
    return canonical_key(q, drop_generic=True)


def classify(q: str) -> str:
    if ADULT.search(q):
        return "adult"
    if CELEB.search(q):
        return "celebrity"
    if NAV.search(q):
        return "navigational"
    if NONPROD.search(q):
        return "non_productizable"
    if NEWS.search(q):
        return "news"
    return "kept"


def load_vocab() -> dict[tuple, set]:
    """(domain, subdomain) -> canonical tokens of every seed in that subdomain, from the taxonomy."""
    import yaml
    from pathlib import Path
    tax = yaml.safe_load((Path(__file__).resolve().parents[1] / "taxonomy" / "domains.yaml").read_text())
    vocab: dict[tuple, set] = {}
    for d in tax["domains"]:
        for sd in d["subdomains"]:
            vocab[(d["id"], sd["id"])] = {t for s in sd["seeds"] for t in canonical_key(s).split()}
    return vocab


def run(con) -> dict:
    rows = con.execute("SELECT query, seed, domain, subdomain, depth FROM queries").fetchall()
    vocab = load_vocab()
    counts: dict[str, int] = {}
    seen_keys: dict[str, str] = {}
    for q, seed, domain, subdomain, depth in rows:
        status = classify(q)
        if status == "kept" and seed:
            qt = set(canonical_key(q).split())
            st = set(canonical_key(seed).split())
            if not (qt & st) and not (qt & vocab.get((domain, subdomain), set())):
                status = "off_topic"   # shares no token with the seed or its subdomain's vocabulary
        if status == "kept":
            key = canonical_key(q)
            if key in seen_keys and seen_keys[key] != q:
                status = "variant"
                con.execute("INSERT OR REPLACE INTO cluster_members(cluster_id, query, canonical_key, is_variant_of) VALUES (?,?,?,?)",
                            ("_variants", q, key, seen_keys[key]))
            else:
                seen_keys.setdefault(key, q)
        con.execute("UPDATE queries SET status=? WHERE query=?", (status, q))
        counts[status] = counts.get(status, 0) + 1
    con.commit()
    return counts
