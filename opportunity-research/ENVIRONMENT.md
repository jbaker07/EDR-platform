# Environment and provenance notes

- Python 3.11.15 (see `requirements.txt` for exact package versions; `numpy`, `trimesh`, `pillow`, `pygltflib` were added
  on 2026-09-21 for the functional tests in `audit/`).
- Store: SQLite, `data/research.db` (about 500 MB at the end of pass 3; not committed). Portable export: the append-only
  deltas in `data/exports/delta/` plus the full cluster snapshots; `audit/recount.sh` rebuilds a store from them and
  compares counts (`audit/recount.md`).
- Autocomplete collection settings, constant for the whole run: Google `hl=en`, `gl=us` (chrome client); YouTube and Bing
  suggestion endpoints with default (US English) settings; the research user agent identifies the project.
- Provenance per observation (table `observations`): source, exact probe string, rank, `meta.intent_probe` (the intent
  prefix family that induced the suggestion, `bare` for the seed itself, `depth2` for a re-probe of a suggestion),
  Google's own relevance and type fields where returned, `observed_at` (UTC). Every stored query therefore records
  whether it appeared on a bare probe or was induced by a prefix.
- Gap: failed suggest calls were not stored as rows. Before the `_call` wrapper was added (pass 1, 05:10Z) a transport
  failure killed the worker (one domain, re-run); after it, failures are logged to `data/logs/expand_<domain>.log` as
  `source_error` lines with the probe. A `probe_log` table with per-probe status is the fix for future runs.
- Filtered queries are preserved with their status in `queries.status` (`kept`, `variant`, `off_topic`,
  `navigational`, `news`, `celebrity`, `adult`, `non_productizable`); nothing is deleted by cleaning.
- Cluster ids are content hashes and change per pass; `data/lineage/` holds the three snapshots and the explicit
  split/merge edges between passes (`cluster_lineage.json`). Task identities are separate from cluster ids and live in
  `taxonomy/tasks.yaml`.
- Search engines: no general web search is reachable from this environment except the harness's web-search tool, which
  has a 200-call session budget (spent). Bing, DuckDuckGo, Mojeek, Brave, Startpage and Yandex either block or return
  navigational junk from this address (tested 2026-09-21). Competitor pages were fetched directly by URL.
