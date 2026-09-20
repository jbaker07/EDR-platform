"""SQLite store: raw observations stay auditable; derived tables can be rebuilt."""
from __future__ import annotations

import json
import sqlite3
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DB = ROOT / "data" / "research.db"

SCHEMA = """
CREATE TABLE IF NOT EXISTS queries (
  query TEXT PRIMARY KEY,           -- normalised text as observed (lowercase, single spaces)
  first_seen TEXT NOT NULL,
  domain TEXT, subdomain TEXT,      -- taxonomy assignment of the seed that surfaced it
  seed TEXT,                        -- the seed term it was expanded from
  depth INTEGER DEFAULT 0,          -- expansion depth from the seed
  status TEXT DEFAULT 'raw'         -- raw | kept | navigational | celebrity | news | adult | variant | non_productizable
);
CREATE TABLE IF NOT EXISTS observations (           -- one row per (query, source, probe): where it was seen
  id INTEGER PRIMARY KEY, query TEXT NOT NULL, source TEXT NOT NULL, probe TEXT NOT NULL,
  rank INTEGER, meta TEXT, observed_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS obs_q ON observations(query);
CREATE TABLE IF NOT EXISTS metrics (                -- demand metrics per provider; kind says what the number is
  id INTEGER PRIMARY KEY, query TEXT NOT NULL, provider TEXT NOT NULL, kind TEXT NOT NULL,
  geo TEXT, lang TEXT, period TEXT, value REAL, extra TEXT, observed_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS met_q ON metrics(query, provider, kind);
CREATE TABLE IF NOT EXISTS serp (                   -- top results per query per engine (a proxy unless engine=google_api)
  id INTEGER PRIMARY KEY, query TEXT NOT NULL, engine TEXT NOT NULL, rank INTEGER, url TEXT, title TEXT,
  site TEXT, category TEXT, observed_at TEXT NOT NULL
);
CREATE INDEX IF NOT EXISTS serp_q ON serp(query, engine);
CREATE TABLE IF NOT EXISTS clusters (
  cluster_id TEXT PRIMARY KEY, label TEXT, description TEXT, intent TEXT, domain TEXT,
  method TEXT, confidence TEXT, created_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS cluster_members (
  cluster_id TEXT NOT NULL, query TEXT NOT NULL, canonical_key TEXT, is_variant_of TEXT,
  PRIMARY KEY (cluster_id, query)
);
CREATE TABLE IF NOT EXISTS competitors (
  id INTEGER PRIMARY KEY, cluster_id TEXT NOT NULL, name TEXT, url TEXT, kind TEXT, solves TEXT, does_not_solve TEXT,
  workflow_remaining TEXT, business_model TEXT, currency TEXT, coverage TEXT, dominates_serp TEXT,
  needs_install INTEGER, needs_account INTEGER, data_reproducible TEXT, ai_replaceable TEXT,
  why_still_searching TEXT, evidence TEXT, observed_at TEXT NOT NULL
);
"""


def now() -> str:
    return datetime.now(timezone.utc).isoformat(timespec="seconds")


def connect(path: Path = DB) -> sqlite3.Connection:
    path.parent.mkdir(parents=True, exist_ok=True)
    con = sqlite3.connect(path, timeout=120)
    con.executescript(SCHEMA)
    return con


def norm(q: str) -> str:
    return " ".join(q.lower().strip().split())


def add_query(con, query: str, *, domain=None, subdomain=None, seed=None, depth=0) -> bool:
    q = norm(query)
    cur = con.execute("INSERT OR IGNORE INTO queries(query, first_seen, domain, subdomain, seed, depth) VALUES (?,?,?,?,?,?)",
                      (q, now(), domain, subdomain, seed, depth))
    return cur.rowcount == 1


def add_observation(con, query: str, source: str, probe: str, rank: int | None = None, meta: dict | None = None) -> None:
    con.execute("INSERT INTO observations(query, source, probe, rank, meta, observed_at) VALUES (?,?,?,?,?,?)",
                (norm(query), source, probe, rank, json.dumps(meta) if meta else None, now()))


def add_metric(con, query: str, provider: str, kind: str, value, *, geo=None, lang=None, period=None, extra=None) -> None:
    con.execute("INSERT INTO metrics(query, provider, kind, geo, lang, period, value, extra, observed_at) VALUES (?,?,?,?,?,?,?,?,?)",
                (norm(query), provider, kind, geo, lang, period, value, json.dumps(extra) if extra else None, now()))


def add_serp(con, query: str, engine: str, results: list[dict]) -> None:
    for r in results:
        con.execute("INSERT INTO serp(query, engine, rank, url, title, site, category, observed_at) VALUES (?,?,?,?,?,?,?,?)",
                    (norm(query), engine, r.get("rank"), r.get("url"), r.get("title"), r.get("site"), r.get("category"), now()))
