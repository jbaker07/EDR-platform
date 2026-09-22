#!/bin/sh
# Independent recount: rebuild a fresh store from the committed delta exports only, then compare counts with the live store.
set -eu
cd "$(dirname "$0")/.."
rm -f data/recount.db
.venv/bin/python - <<'PY'
import os, sqlite3, csv, gzip, json, time
from pipeline import store
t0=time.time()
con=store.connect(store.Path('data/recount.db')) if hasattr(store,'Path') else None
PY
.venv/bin/python - <<'PY'
import sqlite3, csv, gzip, os, time, json
from pathlib import Path
from pipeline import store
t0=time.time()
p=Path('data/recount.db')
con=store.connect(p)
OUT=Path('data/exports/delta')
files=sorted(os.listdir(OUT))
for name in files:
    if not name.endswith('.csv.gz') or '_full_' in name: continue
    table=name.rsplit('_',1)[0]
    with gzip.open(OUT/name,'rt',newline='') as fh:
        r=csv.reader(fh); cols=next(r)
        con.executemany(f"INSERT OR IGNORE INTO {table} ({','.join(cols)}) VALUES ({','.join('?'*len(cols))})", r)
    con.commit()
for name in files:
    if '_full_' in name:
        table=name.split('_full_')[0]
        con.execute(f"DELETE FROM {table}")
        with gzip.open(OUT/name,'rt',newline='') as fh:
            r=csv.reader(fh); cols=next(r)
            con.executemany(f"INSERT OR IGNORE INTO {table} ({','.join(cols)}) VALUES ({','.join('?'*len(cols))})", r)
        con.commit()
live=sqlite3.connect('data/research.db', timeout=120)
rows=[]
for t in ('queries','observations','metrics','serp','clusters','cluster_members','competitors'):
    a=con.execute(f"SELECT COUNT(*) FROM {t}").fetchone()[0]
    b=live.execute(f"SELECT COUNT(*) FROM {t}").fetchone()[0]
    rows.append((t,a,b))
md=["# Independent recount from committed deltas (%s)\n" % time.strftime('%Y-%m-%dT%H:%MZ', time.gmtime()),
    "The store was rebuilt from `data/exports/delta/*.csv.gz` alone (append-only exports committed during the run), then counted against the live store.\n",
    "| table | rebuilt from deltas | live store | note |","|---|---|---|---|"]
notes={'queries':'status column is not in deltas (set by clean on old rows); counts of rows compare, statuses need clean re-run',
       'competitors':'authored records, imported from data/competitors/*.yaml, not part of deltas'}
for t,a,b in rows: md.append(f"| {t} | {a} | {b} | {notes.get(t,'')} |")
# distinct queries and observation sources
md.append("\nDistinct queries in rebuilt store: %d; observation sources: %s" % (con.execute("SELECT COUNT(DISTINCT query) FROM queries").fetchone()[0], con.execute("SELECT source, COUNT(*) FROM observations GROUP BY source").fetchall()))
md.append("\nElapsed: %.0f s" % (time.time()-t0))
Path('audit/recount.md').write_text("\n".join(md)+"\n")
print("\n".join(md))
PY
