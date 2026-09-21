# Pilot report (calibration only)

Measured on the pilot expansion; every number below is read from `data/research.db`. Nothing here is a demand estimate: no Keyword Planner data has been imported yet.

## Queries per seed

| seed | domain / subdomain | raw | kept | variants | filtered |
|---|---|---|---|---|---|
| unity | game_development / unity | 1731 | 1390 | 162 | 179 |
| circuit breaker | home_repair / electrical | 1351 | 1123 | 217 | 11 |
| check engine light | automotive / diagnostics | 1289 | 892 | 352 | 45 |
| sourdough starter | cooking_baking / baking | 1277 | 975 | 285 | 17 |
| stringing | three_d_printing / troubleshooting | 1136 | 461 | 67 | 608 |
| maker's mark | collecting_antiques / identification | 937 | 700 | 126 | 111 |
| obd2 code | automotive / diagnostics | 683 | 513 | 94 | 76 |
| outlet not working | home_repair / electrical | 589 | 423 | 156 | 10 |
| pottery mark | collecting_antiques / identification | 580 | 482 | 77 | 21 |
| layer shift | three_d_printing / troubleshooting | 441 | 322 | 91 | 28 |
| unity navmesh | game_development / unity | 363 | 282 | 69 | 12 |

Status totals: {"celebrity": 10, "kept": 7563, "navigational": 113, "news": 129, "non_productizable": 4, "off_topic": 862, "variant": 1696}

Observations by source: {'suggest:google': 6801, 'suggest:youtube': 3723, 'suggest:bing': 6503}

Queries by number of distinct sources that suggested them: {1: 9144, 2: 1007, 3: 226}

Suggestions by intent probe family: {'depth2': 4658, 'fix': 1627, 'how_to': 1323, 'find': 1003, 'calculate': 975, 'decide': 800, 'plan': 796, 'compare': 771, 'generate': 729, 'convert': 726, 'optimize': 701, 'alternative': 679, 'track': 622, 'identify': 592, 'compatibility': 549, 'bare': 476}

## Seed drift

10 kept depth-1 queries share no canonical token with their seed (a sign the seed was ambiguous or the probe pulled a neighbouring topic). Examples:

- `obd2 code` -> `how to clear engine codes without a scanner`
- `obd2 code` -> `how to check engine codes on harley davidson`
- `obd2 code` -> `how to get engine codes without scanner`
- `obd2 code` -> `how to scan engine codes`
- `obd2 code` -> `how to use codename engine`
- `obd2 code` -> `will obd 2 work for all cars`
- `obd2 code` -> `how many engine codes are there`
- `obd2 code` -> `how many check engine codes are there`
- `obd2 code` -> `obd 2 start date`
- `stringing` -> `springing vs shifting executory interest`

## Lexical pre-clustering

455 clusters of size >= 3; size distribution (size: count): {3: 202, 4: 94, 5: 43, 6: 30, 7: 21, 8: 17, 9: 8, 10: 5, 11: 3, 12: 4, 13: 4, 14: 3, 15: 3, 16: 3, 17: 2, 18: 1, 20: 1, 21: 1, 22: 1, 27: 1, 29: 1, 31: 1, 39: 1, 44: 1, 46: 1, 52: 2, 70: 1}

Largest pre-clusters (unreviewed; a lexical component, not yet a problem):

| cluster | label | intent | domain | queries | sample |
|---|---|---|---|---|---|
| c_94112270 | identification pottery mark guide | informational | collecting_antiques | 70 | australian pottery marks identification guide; australian pottery marks identification guide pdf; australian pottery marks identification guide pdf free |
| c_82417246 | unity asset so slow | optimize | game_development | 52 | assets unity for beginners; best unity assets; best unity vr assets |
| c_99458696 | guitar string classical acoustic | informational | three_d_printing | 52 | best strings for acoustic guitar; can i string an acoustic guitar with nylon strings; changing strings on a classical guitar |
| c_73956031 | version unity install vrchat | decide | game_development | 46 | best unity editor version; best unity version; best unity version control |
| c_93930071 | cod list obd2 pdf | find | automotive | 44 | bmw obd2 codes list pdf; chevy obd2 codes list; chevy obd2 codes list pdf |
| c_67145450 | sourdough starter recipe beginner | informational | cooking_baking | 39 | best sourdough starter for beginners; best sourdough starter kit; best sourdough starter kit for beginners |
| c_13434722 | badminton string machine racket | decide | three_d_printing | 31 | badminton racket stringing guide; badminton stringing tutorial; best badminton racket stringing machine |
| c_71272204 | outlet tripp work electrical | fix | home_repair | 29 | convert outlet not working breaker not tripped; electrical outlet not working; electrical outlet not working after reset |
| c_40227094 | finder circuit breaker panel | informational | home_repair | 27 | best circuit breaker finder; best circuit breaker finder for home use; best circuit breaker finder kit |
| c_88717094 | maker mark bourbon much | calculate | collecting_antiques | 22 | best maker's mark bourbon; best maker's mark whiskey; how much does maker's mark 46 cost |
| c_68870060 | print shift layer 3d | informational | three_d_printing | 21 | 3d printing problems layer shifting; big layer shift 3d printing; how to fix layer shift 3d printing |
| c_02519695 | unreal 5 unity graphic | compare | game_development | 20 | unity compatible with unreal engine 5; unity navmesh compatible with unreal engine 5; unity or unreal engine for beginners |
| c_37276915 | discard starter sourdough feed | calculate | cooking_baking | 18 | how do i discard sourdough starter; how many sourdough starter jars do i need; how many sourdough starters do i need |
| c_45957390 | tour distillery maker mark | generate | collecting_antiques | 17 | best maker's mark distillery tour; best maker's mark tour; maker's mark distillery kentucky |
| c_70777114 | mak who mark maker | generate | collecting_antiques | 17 | maker's mark making; mark making tutorials; what is mark making |
| c_20478812 | directory template circuit breaker | generate | home_repair | 16 | circuit breaker box label template; circuit breaker box labels; circuit breaker directory |
| c_52129375 | bad gone sourdough starter | informational | cooking_baking | 16 | can sourdough starter go bad; can sourdough starter go bad in the fridge; has sourdough starter gone bad |
| c_97957751 | print 3d string cause | informational | three_d_printing | 16 | 3d print random stringing; 3d stringing problem; fix stringing 3d printing |
| c_22964099 | position stay breaker circuit | fix | home_repair | 15 | breaker switch won't stay on; breaker won't stay in on position; circuit breaker on off position |
| c_42694719 | catalytic converter light check | convert | automotive | 15 | catalytic converter cause check engine light; catalytic converter check engine light fix; catalytic converter check engine light flashing |

## Proxies collected on a sample (never summed with search volume)

| provider | kind | queries | mean value |
|---|---|---|---|
| stackexchange | se_question_total | 24 | 13.0 |
| youtube | estimated_results | 24 | 961870.9 |

## SERP mixes on a sample

| engine | query | fragmented share | dominant site (share) | categories |
|---|---|---|---|---|
| bing_html | best sourdough bread to buy at grocery store | 0.90 | bestbuy.com (0.20) | {'independent_site': 9, 'docs': 1} |
| bing_html | can you diagnose a car without check engine light | 0.86 | merriam-webster.com (0.14) | {'independent_site': 5, 'youtube': 1, 'docs': 1} |
| bing_html | check engine light after alternator replacement | 0.70 | dictionary.cambridge.org (0.20) | {'independent_site': 7, 'docs': 3} |
| bing_html | circuit breaker design pattern in microservices in hindi | 0.70 | en.wikipedia.org (0.20) | {'independent_site': 7, 'government': 1, 'docs': 2} |
| bing_html | free circuit breaker panel template | 1.00 | poki.com (0.20) | {'independent_site': 9, 'forum': 1} |
| bing_html | how many market makers are there | 0.80 | merriam-webster.com (0.20) | {'independent_site': 8, 'docs': 2} |
| bing_html | how to fix sourdough starter that smells like alcohol | 0.80 | merriam-webster.com (0.20) | {'independent_site': 6, 'large_platform': 1, 'blog_editorial': 1, 'docs': 1, 'youtube': 1} |
| bing_html | maker's mark tour guides | 0.90 | makerworld.com (0.30) | {'independent_site': 9, 'docs': 1} |
| bing_html | obd2 pc software | 0.80 | support.microsoft.com (0.10) | {'large_platform': 2, 'independent_site': 8} |
| bing_html | outlet not working but not tripped | 1.00 | premiumoutlets.com (0.10) | {'independent_site': 10} |
| bing_html | show navmesh target path unity | 0.90 | merriam-webster.com (0.20) | {'independent_site': 7, 'youtube': 2, 'docs': 1} |
| bing_html | sourdough starter instructions printable | 0.90 | en.wikipedia.org (0.10) | {'docs': 1, 'recipe_site': 5, 'independent_site': 4} |
| websearch_tool | catalytic converter check engine light fix | 0.89 | quora.com (0.11) | {'forum': 1, 'independent_site': 4, 'blog_editorial': 3, 'marketplace': 1} |
| websearch_tool | fix stringing 3d printing | 0.89 | support.makerbot.com (0.11) | {'blog_editorial': 6, 'vendor_content': 1, 'government': 1, 'independent_site': 1} |
| websearch_tool | how do i make sourdough starter | 1.00 | littlespoonfarm.com (0.11) | {'recipe_site': 2, 'independent_site': 6, 'blog_editorial': 1} |
| websearch_tool | how to read obd2 codes | 1.00 | autopi.io (0.10) | {'blog_editorial': 4, 'independent_site': 3, 'forum': 1, 'vendor_content': 2} |
| websearch_tool | how to stop layer shifting | 0.89 | sovol3d.com (0.33) | {'independent_site': 2, 'blog_editorial': 6, 'docs': 1} |
| websearch_tool | how to use navmesh unity | 0.22 | docs.unity3d.com (0.33) | {'docs': 4, 'blog_editorial': 1, 'large_platform': 2, 'forum': 1, 'marketplace': 1} |

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

