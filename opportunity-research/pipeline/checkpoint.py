"""Append-only delta export of the research store, so a lost container loses at most one interval.

Each run writes data/exports/delta/<table>_<stamp>.csv.gz holding only rows newer than the previous
checkpoint (queries by first_seen, observations/metrics/serp by observed_at). clusters and
cluster_members are regenerated wholesale by `cluster`, so they are exported in full only when
`--full` is passed (pass boundaries). `restore` rebuilds an empty store from the deltas.
"""
import csv, gzip, json, os, sqlite3, sys
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
DB = os.path.join(HERE, "data", "research.db")
OUT = os.path.join(HERE, "data", "exports", "delta")
STATE = os.path.join(OUT, "state.json")
TS_COL = {"queries": "first_seen", "observations": "observed_at", "metrics": "observed_at", "serp": "observed_at"}
FULL = ("clusters", "cluster_members")


def _state():
    if os.path.exists(STATE):
        return json.load(open(STATE))
    return {"last": {}}


def export(full=False):
    os.makedirs(OUT, exist_ok=True)
    st = _state()
    con = sqlite3.connect(DB, timeout=120)
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    marker = datetime.now(timezone.utc).isoformat(timespec="seconds")
    written = {}
    for table, col in TS_COL.items():
        last = st["last"].get(table, "")
        # overlap the window by 15 minutes: a worker can commit a row whose timestamp predates the previous marker
        # (the recount found 301 such queries); duplicates are harmless because restore uses INSERT OR IGNORE
        if last:
            from datetime import datetime as _dt, timedelta as _td
            last = (_dt.fromisoformat(last) - _td(minutes=15)).isoformat(timespec="seconds")
        cur = con.execute(f"SELECT * FROM {table} WHERE {col} > ? AND {col} <= ? ORDER BY {col}", (last, marker))
        rows = cur.fetchall()
        if not rows:
            continue
        path = os.path.join(OUT, f"{table}_{stamp}.csv.gz")
        with gzip.open(path, "wt", newline="") as fh:
            w = csv.writer(fh); w.writerow([d[0] for d in cur.description]); w.writerows(rows)
        st["last"][table] = marker
        written[table] = len(rows)
    if full:
        for table in FULL:
            cur = con.execute(f"SELECT * FROM {table}")
            path = os.path.join(OUT, f"{table}_full_{stamp}.csv.gz")
            with gzip.open(path, "wt", newline="") as fh:
                w = csv.writer(fh); w.writerow([d[0] for d in cur.description]); n = 0
                for row in cur:
                    w.writerow(row); n += 1
            written[table] = n
            for old in os.listdir(OUT):
                if old.startswith(f"{table}_full_") and not old.endswith(f"{stamp}.csv.gz"):
                    os.remove(os.path.join(OUT, old))
    json.dump(st, open(STATE, "w"), indent=1)
    total = con.execute("SELECT COUNT(*) FROM queries").fetchone()[0]
    print(json.dumps({"checkpoint": stamp, "written": written, "queries_total": total}))


def restore():
    """Rebuild data/research.db from the delta files (INSERT OR IGNORE, so it is safe on a partial store)."""
    from . import store
    con = store.connect()
    files = sorted(os.listdir(OUT))
    for name in files:
        if not name.endswith(".csv.gz") or "_full_" in name:
            continue
        table = name.rsplit("_", 1)[0]
        with gzip.open(os.path.join(OUT, name), "rt", newline="") as fh:
            r = csv.reader(fh); cols = next(r)
            con.executemany(f"INSERT OR IGNORE INTO {table} ({','.join(cols)}) VALUES ({','.join('?' * len(cols))})", r)
        con.commit(); print("restored", name)
    for name in files:
        if "_full_" in name:
            table = name.split("_full_")[0]
            con.execute(f"DELETE FROM {table}")
            with gzip.open(os.path.join(OUT, name), "rt", newline="") as fh:
                r = csv.reader(fh); cols = next(r)
                con.executemany(f"INSERT OR IGNORE INTO {table} ({','.join(cols)}) VALUES ({','.join('?' * len(cols))})", r)
            con.commit(); print("restored", name)


if __name__ == "__main__":
    if "restore" in sys.argv:
        restore()
    else:
        export(full="--full" in sys.argv)
