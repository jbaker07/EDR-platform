# Independent recount from committed deltas (2026-09-21T22:14Z)

The store was rebuilt from `data/exports/delta/*.csv.gz` alone (append-only exports committed during the run), then counted against the live store.

| table | rebuilt from deltas | live store | note |
|---|---|---|---|
| queries | 849799 | 850100 | status column is not in deltas (set by clean on old rows); counts of rows compare, statuses need clean re-run |
| observations | 1387447 | 1387717 |  |
| metrics | 27982 | 27982 |  |
| serp | 2000 | 2000 |  |
| clusters | 42244 | 42244 |  |
| cluster_members | 441593 | 441593 |  |
| competitors | 0 | 190 | authored records, imported from data/competitors/*.yaml, not part of deltas |

Distinct queries in rebuilt store: 849799; observation sources: [('suggest:bing', 597222), ('suggest:google', 501514), ('suggest:youtube', 288711)]

Elapsed: 44 s

## Residual explained and closed (2026-09-21T22:30Z)

The 301 queries and 270 observations missing from the delta rebuild were rows committed by workers after the export's
timestamp marker was taken but with timestamps before it (in-flight writes). A one-off catch-up file
(`*_catchup.csv.gz`) exports exactly those rows, and `pipeline/checkpoint.py` now overlaps each export window by
15 minutes; overlaps are harmless because restore inserts with INSERT OR IGNORE.
