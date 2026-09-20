# Search-opportunity discovery: pipeline design and access requirements

Purpose: find problem families with large, fragmented search demand that one free
resource could consolidate, by measuring rather than guessing. This document is the
pipeline design and the exact data/access it needs. It contains no business
recommendation; Phases 6-8 do not start until real volume data is in the store.

Everything below is one of three kinds of statement, and says which:
**measured** (a probe or a stored record), **inference** (reasoned from measurements),
**hypothesis** (to be tested). Nothing in this project sums a community post count
into a search volume.

## 1. What this sandbox can and cannot reach (measured 2026-09-20)

| source | role in the pipeline | status from this sandbox |
|---|---|---|
| Google autocomplete (`google.com/complete/search`, chrome and firefox clients) | discovery of long-tail queries; chrome client also returns a per-suggestion relevance score | **works**, unauthenticated |
| YouTube autocomplete | discovery; a video-intent signal | **works** |
| Bing and DuckDuckGo autocomplete | discovery; corroboration | **works** |
| Google Trends (explore, related/rising queries, interest over time) | relative interest, rising queries, cross-cluster comparison against an anchor term | **works through a cookie session, briefly**: related/rising queries and a 5-term comparison succeeded, then the address was throttled (429) even with the session; usable only at a handful of requests per hour, so a proxy for spot checks, not for bulk |
| Stack Exchange API v2.3 | community-activity proxy per query (question totals, top questions); keyless | **works**; 300 requests/day per IP without a key, 10,000 with a registered key |
| Wikipedia pageviews (per article) | informational-demand proxy for encyclopedic topics | **works but rate-limited** on this shared address (top-views and article search returned 429) |
| Bing result page (HTML) | SERP proxy for fragmentation classification | **parseable but unusable from here**: on a 12-query pilot sample it returned dictionary sites and unrelated shops for problem queries (see `data/pilot_report.md`); removed from the Phase 6 plan |
| YouTube results page | video supply per query (`estimatedResults`, top titles/views) | **works** (unofficial HTML) |
| `WebSearch` tool (agent-driven) | 10 titled results per query; US only | **works**; engine unspecified, labelled as such |
| Google result page (HTML) | true Google SERP | **blocked** (script challenge, no organic results) |
| Reddit JSON / API | community evidence | **blocked** (403) without OAuth credentials |
| GitHub search API | open-source competitor discovery | **blocked** (session bound to one repository) |
| Google Ads API (`googleads.googleapis.com`) | Keyword Planner volumes | reachable, **needs credentials** |
| DataForSEO (`api.dataforseo.com`) | paid Google-volume and SERP API | reachable, **needs credentials** |

## 2. Exactly what access is required from you

Ranked by how much of the research it unblocks.

1. **Google Keyword Planner data (required for any demand number).** Two ways:
   - *UI exports.* A Google Ads account (no spend needed; a campaign may need to exist but
     can stay paused). In Keyword Planner use "Get search volume and forecasts", paste
     keyword lists in batches (the UI caps each batch; thousands per batch in practice,
     confirm the limit in the UI), set location and language, and export CSV. Also use
     "Discover new keywords" exports for seed terms. Drop files into `data/raw/kp/` and
     run `python -m pipeline.cli import-kp <file> --geo US --lang en`. This is the
     zero-cost path and gives avg monthly searches, competition, top-of-page bids and 12
     months of history. Accounts without spend see bucketed volumes ("1K-10K") unless the
     account has run ads; the importer stores the bucket midpoint and the raw string.
   - *API.* Since 9 September 2026 Google Ads API access is granted per Google Cloud
     project (developer tokens are sunset) and Basic access requires brand verification of
     the project, per Google's documentation fetched 2026-09-20. That is a heavier
     process than the UI export for an individual; the adapter stays for when it exists. This gives exact rounded volumes for up to 10,000 keywords per request
     via `KeywordPlanIdeaService.GenerateKeywordHistoricalMetrics`. Documented (fetched
     2026-09-20): 1 request per second per customer id; returns approximate average monthly
     searches, twelve monthly volumes, competition and index, 20th/80th-percentile
     top-of-page bids. The per-request keyword cap is not stated on the quotas page and
     must be confirmed once credentials exist. The adapter is `pipeline/sources/google_ads.py`
     (written, untested: no credentials).
2. **A SERP source for real Google results (required for Phase 6 at scale).** Either a
   DataForSEO login (pay-as-you-go; SERP and Google Ads volume endpoints) or a SerpApi
   key. DataForSEO, per its pricing pages fetched 2026-09-20: $50 minimum deposit,
   Google Ads search volume $0.06 per task of up to 1,000 keywords in queued mode
   ($0.09 live), so 300,000 keywords cost about $18 and 900,000 about $54. SerpApi,
   per its pricing page fetched 2026-09-20: 250 free searches a month, then $25 for
   1,000 and $75 for 5,000 a month, cancellable monthly. Either is also a fallback for item 1. Without one, Phase 6 uses the `WebSearch` tool and Bing as labelled proxies and
   is limited to a few hundred clusters checked by hand.
3. **Reddit API credentials** (a script app: client id and secret; free tier). Unblocks
   community evidence per cluster: thread counts, recency, whether answers were marked
   solved. Without it, Reddit evidence comes only from SERP result rows.
4. **YouTube Data API key** (free quota 10,000 units/day). Replaces HTML scraping of
   result pages with a stable count and metadata per query.
5. **Stack Exchange app key** (free) to lift the daily cap from 300 to 10,000.
6. **GitHub access outside the current repository** (a token, or adding repositories to
   this session) for open-source competitor discovery in Phase 7.
7. Optional: Bing Webmaster / Microsoft Advertising keyword API (free with an account) as a
   second, independent volume source for cross-checking Keyword Planner.

## 3. Pipeline

```
taxonomy/domains.yaml   60 domains, 307 subdomains, 1477 seeds          (Phase 1)
        |  expand: intent probes + alphabet soup, depth 2, 3 engines    (Phase 2)
        v
data/research.db  queries / observations (source, probe, rank)  -- raw, append-only
        |  clean: navigational / celebrity / news / adult / variant     (Phase 3)
        |  cluster: canonical-token Jaccard union-find -> review          (Phase 4)
        |  import-kp: Keyword Planner volumes per query (never imputed)
        |  enrich: Stack Exchange totals, YouTube counts, Trends           (proxies, kept apart)
        |  serp: Bing / WebSearch / SERP-API result rows, category-classified (Phase 6)
        v
data/exports/clusters.json  per-cluster: demand with denominators, breadth,
                            fragmentation mix, proxies, qualitative fields (Phase 5-7)
```

Every derived table can be rebuilt from `queries` + `observations` + `metrics` + `serp`;
those four are never edited by hand.

### Phase 2 volume plan (measured on the pilot, inference for the full run)

Measured: with intent probes only (no alphabet soup), depth-2 re-probes of the 15 most
corroborated results, and three engines, a seed took about 2 minutes and yielded 360 to
1,290 new queries (broad seeds like "sourdough starter" and "check engine light" sit at the
top; a specific seed like "unity navmesh" at the bottom). Autocomplete is prolific: the
constraint is not discovery but drift and duplication, not request rate.

Two measured failure modes shape the full run:

- **Seed drift.** Unqualified seeds pull neighbouring topics: "stringing" returned guitar
  strings, weed-eater string and stinging nettles; "layer shift" returned hair layers and
  Photoshop; "maker's mark" returned the bourbon. Rule: seeds carry their domain word
  ("3d print stringing", "pottery maker's mark"), and the cleaner marks a query
  `off_topic` when it shares no canonical token with its seed or its subdomain's seed
  vocabulary. Off-topic queries stay in the store (they are still real searches) but are
  not clustered under that subdomain.
- **Clustering chains.** Plain single-linkage over token overlap joined 6,168 of 6,695
  kept pilot queries into one component through polysemous tokens ("engine" links
  "unity engine" to "check engine light"). The clustering now weights tokens by inverse
  document frequency, requires two shared tokens per link, and re-splits any component
  above 80 queries at a stricter threshold, recording the threshold each cluster formed at.

Full-run plan (inference): 1,477 seeds at ~2 minutes each is ~50 hours on one worker;
`run_full.sh` runs one worker per domain in parallel (each paced per engine host), so
four workers finish in roughly 12 hours if no engine throttles, and the run is resumable
per seed. Expect 300,000 to 900,000 raw queries before cleaning; the pilot's kept fraction
and variant rate (in `data/pilot_report.md`) are the calibration for how many survive.
Alphabet soup is off by default: it roughly doubles requests for mostly duplicate results
on Google, and it is the part most worth re-enabling for Google only on the subdomains
that reach Phase 6.

### Phase 3 rules

Status is stored per query and nothing is deleted: `navigational` (login, download,
near me, phone number...), `celebrity` (net worth, dating, height...), `news` (dated,
live, release date), `adult`, `non_productizable` (lyrics, single-word meanings,
translations of phrases), `variant` (identical canonical token key to a kept query;
the variant is linked to its canonical query so aggregate demand can be de-duplicated
exactly the way the report says it is).

### Phase 4 clustering and the double-counting rule

Pre-clustering is lexical and auditable: canonical token sets (stopwords and intent words
removed, crude stemming), Jaccard >= 0.5, union-find, minimum size 3. Each cluster is
labelled by its most frequent tokens and dominant intent and marked `unreviewed`. A
review pass (analyst, with a model as an assistant where useful) merges, splits and
renames, and records `confidence`. Demand aggregation reports, per cluster: the sum of
Keyword Planner volume over members that have a value, the number of members without a
value, and the count of distinct canonical keys. Variants are excluded from the sum.
Keyword Planner already groups close variants; when two members share a canonical key
only one contributes.

### Phase 6 fragmentation classification

Each result row is classified by URL pattern into: reddit, stackexchange, forum,
youtube, github, docs, wikipedia, marketplace, qa_platform, government, large_platform,
blog_editorial, recipe_site, vendor_content, and `independent_site` for any domain the
patterns do not recognise (a blog, a shop or a company page; the pilot shows most results
land here, so Phase 6 needs a per-site type table built as clusters are reviewed). Per query and per engine the store keeps the category mix, the
fragmented share (community + forum + video + code host + blog), and the dominant site's
share. A fragmented SERP is a signal to inspect, not a conclusion about answer quality;
answer quality is a separate manual judgement recorded in the qualitative fields.

### Phase 7 competitor record

`competitors` rows carry the twelve questions from the brief as columns (solves, does not
solve, workflow remaining, business model, currency, coverage, SERP dominance, install,
account, data reproducibility, AI replaceability, why people still search) plus the
evidence for each answer.

### Scoring

No single score. `clusters.json` exposes demand (with denominators), breadth,
fragmentation, proxies, and the qualitative fields left `null` until a person fills
them: incumbent strength, answer quality, productizability, repeatability, maintenance,
AI resilience, SERP resilience, monetization. Ranking is by demand first and then by
the visible metrics, with reasoning written out per finalist.

## 4. Commands

```sh
.venv/bin/python -m pipeline.cli expand [--domain D] [--subdomain S] [--no-soup] [--depth2 N] [--sources google,youtube,bing]
.venv/bin/python -m pipeline.cli clean
.venv/bin/python -m pipeline.cli cluster [--threshold 0.5] [--min-size 3] [--domain D]
.venv/bin/python -m pipeline.cli import-kp data/raw/kp/<export>.csv --geo US --lang en
.venv/bin/python -m pipeline.cli enrich --limit 50 --kinds se,yt      # proxies on a sample
.venv/bin/python -m pipeline.cli serp --limit 50                       # Bing proxy SERPs on a sample
.venv/bin/python -m pipeline.cli score --top 40                        # writes data/exports/clusters.json
.venv/bin/python -m pipeline.cli stats
```

## 5. What is deliberately not in this design

- No volume estimates from autocomplete presence, Trends indexes, Stack Exchange counts
  or YouTube counts. Those are proxies and are stored under their own names.
- No "opportunity score". The metrics stay visible.
- No product proposal before Phases 6 and 7 for the cluster in question.
