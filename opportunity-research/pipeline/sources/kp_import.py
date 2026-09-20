"""Import Google Keyword Planner exports (the only free Google-volume source; needs a Google Ads account).

Accepts the UI's CSV/TSV exports from both "Discover new keywords" and "Get search volume and forecasts".
Column names are matched loosely because Google renames them. Values are stored as provider
'google_keyword_planner' with kinds: avg_monthly_searches, competition, competition_index,
top_of_page_bid_low, top_of_page_bid_high, monthly_<YYYY-MM>. Keyword Planner numbers are Google's
own rounded estimates; they are stored as given and labelled approximate in every report.
"""
from __future__ import annotations

import csv
import re
from pathlib import Path

from pipeline import store

MONTH = re.compile(r"searches?:?\s*([A-Za-z]{3})\s+(\d{4})", re.I)
MONTHS = {m: i for i, m in enumerate(["jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec"], 1)}


def _num(v: str):
    v = (v or "").strip().replace(",", "").replace("–", "-")
    if not v or v in ("-", "—"):
        return None
    if re.fullmatch(r"\d+(\.\d+)?", v):
        return float(v)
    m = re.fullmatch(r"(\d+)\s*-\s*(\d+)", v)   # "1K – 10K" style buckets become a low/high pair
    if m:
        return (float(m.group(1)) + float(m.group(2))) / 2
    m = re.fullmatch(r"(\d+(?:\.\d+)?)([KM])\s*-\s*(\d+(?:\.\d+)?)([KM])", v.upper())
    if m:
        mult = {"K": 1e3, "M": 1e6}
        return (float(m.group(1)) * mult[m.group(2)] + float(m.group(3)) * mult[m.group(4)]) / 2
    return None


def import_file(con, path: Path, geo: str, lang: str, source_note: str = "") -> int:
    text = path.read_text(encoding="utf-16" if path.read_bytes()[:2] in (b"\xff\xfe", b"\xfe\xff") else "utf-8-sig", errors="replace")
    lines = text.splitlines()
    start = next((i for i, l in enumerate(lines) if l.lower().startswith("keyword")), 0)
    dialect = "excel-tab" if "\t" in lines[start] else "excel"
    rows = list(csv.DictReader(lines[start:], dialect=dialect))
    n = 0
    for r in rows:
        kw = (r.get("Keyword") or r.get("keyword") or "").strip()
        if not kw:
            continue
        store.add_query(con, kw)
        for col, val in r.items():
            if col is None:
                continue
            c = col.strip().lower()
            if c.startswith("avg. monthly searches") or c == "avg monthly searches":
                store.add_metric(con, kw, "google_keyword_planner", "avg_monthly_searches", _num(val), geo=geo, lang=lang, extra={"raw": val, "file": path.name, "note": source_note})
            elif c == "competition":
                store.add_metric(con, kw, "google_keyword_planner", "competition", None, geo=geo, lang=lang, extra={"raw": val})
            elif "competition (indexed" in c:
                store.add_metric(con, kw, "google_keyword_planner", "competition_index", _num(val), geo=geo, lang=lang)
            elif "top of page bid (low" in c:
                store.add_metric(con, kw, "google_keyword_planner", "top_of_page_bid_low", _num(val), geo=geo, lang=lang)
            elif "top of page bid (high" in c:
                store.add_metric(con, kw, "google_keyword_planner", "top_of_page_bid_high", _num(val), geo=geo, lang=lang)
            else:
                m = MONTH.search(col)
                if m and m.group(1).lower()[:3] in MONTHS:
                    period = f"{m.group(2)}-{MONTHS[m.group(1).lower()[:3]]:02d}"
                    store.add_metric(con, kw, "google_keyword_planner", "monthly", _num(val), geo=geo, lang=lang, period=period)
        n += 1
    con.commit()
    return n
