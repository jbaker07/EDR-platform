## What the pilot changes in the full-run plan (analyst notes on the measurements above)

Everything in this section is an inference from the tables above, written after reading them.

1. **Discovery is not the bottleneck; drift and duplication are.** Eleven seeds produced
   10,377 raw queries. After cleaning, 7,563 are kept, 1,696 are exact-key variants of a
   kept query (16 percent), 862 are off-topic drift (8 percent) and 256 are navigational,
   news, celebrity or non-productizable. The variant rate is the number to apply when
   reading any future "queries per cluster" figure: roughly one in six autocomplete
   results is a re-ordering or pluralisation of another.
2. **Generic seeds must be qualified.** "stringing" lost 608 of 1,136 results to
   filtering and still kept a 52-query guitar-string cluster and a 31-query badminton
   cluster, because "string" survives stemming and matches the seed. "maker's mark"
   produced bourbon clusters. The taxonomy will be revised so every seed carries its domain
   word, and the off-topic rule stays as a second net.
3. **Cross-source corroboration is rare and therefore informative.** Only 226 queries were
   suggested by all three engines and 1,007 by two; 9,144 by one. A query suggested by
   two or three engines is a stronger existence signal than one engine's tail, and the
   full run will carry the corroboration count into clustering and into the sample
   selection for SERP and Keyword Planner lookups.
4. **Clustering now forms recognisable topics but is not yet problems.** 455 components of
   size three or more, none oversized; the largest are "pottery mark identification"
   (70), "unity assets" (52), "obd2 code list pdf" (44), "sourdough starter for beginners"
   (39), "outlet not working / tripped" (29), "circuit breaker finder" (27), "layer shift"
   (21). These are lexical components; the review pass must still merge components that
   are one problem (for example the several sourdough-starter components) and split
   components that are one word.
5. **Bing HTML is unusable from this address as a SERP proxy.** Twelve Bing pages returned
   dictionary sites and unrelated shops for problem queries (premiumoutlets.com for
   "outlet not working but not tripped"; merriam-webster.com for a Unity navmesh query).
   The rows are kept in the store under engine `bing_html` as evidence of that, and Bing
   is removed from the Phase 6 plan. The WebSearch tool's six samples were on topic, and a
   real Google SERP source remains the requirement for Phase 6 at scale.
6. **Proxies behave as expected and stay separate.** On a 24-query sample, Stack Exchange
   totals averaged 13 questions per query and YouTube estimated results averaged about
   960,000 per query; the spread is what makes them useful for ranking within a domain
   and useless as volumes.
7. **Intent probes pull their intended families.** Fix probes yielded 1,627 suggestions,
   how-to 1,323, find 1,003, calculate 975; the depth-2 re-probe produced 4,658, so the
   second level is where most breadth comes from and where drift enters. Depth 2 will be
   restricted to queries that share a token with the seed.
8. **No demand number exists yet.** Every cluster shows volume 0 with 0 members valued.
   Nothing about opportunity size can be said until Keyword Planner data is imported.

## Environment fact learned during the full run (2026-09-21)

The sandbox hibernates between agent turns: the filesystem is restored (the store and
logs survived) but background processes do not. The first full-run attempt ran for about
13 minutes after the launching turn ended, expanded 19 seeds, and died. Consequence: the
expansion proceeds only during active turns, in foreground-kept chunks, and is ordered
breadth-first (one seed per subdomain across all 307 subdomains first, then a second and
third seed) so that partial progress is always a usable cross-section rather than a few
complete domains. Also corrected: the first health check compared ISO timestamps
containing 'T' with SQLite's space-separated datetime, which made dead workers look alive.
