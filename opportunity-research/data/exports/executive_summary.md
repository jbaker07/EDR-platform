# Executive summary: evidence-driven search for a fragmented, high-demand internet problem (free route, 2026-09-21)

## 1. What was asked and how it was done

The brief: search across all human activities for a problem that very many people repeatedly search for, whose answer
exists but is fragmented across forums, blogs, videos, documentation and paid tools, and where one free destination could
complete the task; measure demand, fragmentation and competition; rank Top 100 → Top 20 → Top 5 → #1; never fabricate
volumes, traffic, revenue, market share, competitor features or user counts; label measured fact, inference and hypothesis.

The route chosen, at the founder's decision, was the free route: no paid keyword or SERP data. Every number below is
therefore either a count inside the research store or a relative signal, and the report says "unmeasured" wherever a
paid source would have given a figure.

Pipeline (all code in `opportunity-research/pipeline/`, all data reproducible from the committed deltas):

1. Taxonomy: 60 domains, 307 subdomains, 1,488 authored seed phrases with disambiguating qualifiers.
2. Discovery: Google, YouTube and Bing autocomplete, each seed probed bare and with 50 intent prompts in 14 families
   (how to, fix, compare, decide, compatibility, alternative, calculate, generate, convert, find, identify, optimise,
   track, plan), then a second level re-probing the on-topic suggestions. Three passes (one, two, three seeds per
   subdomain). Every suggestion is stored with its engine, probe and rank, so any query can be traced to the prompt
   that surfaced it.
3. Cleaning: navigational, celebrity, news, adult, non-productizable, off-topic and reordered-variant queries removed
   or flagged.
4. Clustering: IDF-weighted lexical components with a two-token minimum and a recursive split ladder.
5. Ranking with visible metrics and no composite score: members (long-tail breadth), task-intent share, "organic"
   task intent (intents the engines volunteered on bare probes, so that echoes of the research's own prompts do not
   inflate every cluster equally), cross-engine corroboration, relative demand, fragmentation.
6. Fragmentation: representative task queries run through the web-search tool available in the environment (a US-only
   proxy, not Google), results classified with a curated site list (community, editorial, aggregator, reference,
   vendor, tool, official, marketplace; unclassified never counted as fragmented).
7. Competitor research: for 24 clusters, up to eight result pages fetched each, the brief's twelve questions answered
   with a verbatim quote per fact, judgments labelled inference, "unknown" where a page is silent.
8. Synthesis: the twenty fields per Top-20 candidate, Top 5, #1, with every claim traceable to a measurement, a fetched
   quote or an authored judgment.

## 2. Numbers at each stage (measured)

| stage | result |
|---|---|
| seeds expanded | 921 (307 subdomains × 3 passes) |
| autocomplete observations | 1,387,717 |
| distinct queries | 850,100 |
| kept after cleaning | 647,999 (variants 141,035; off-topic 25,148; navigational 16,047; news 15,508; celebrity 2,466; non-productizable 1,891) |
| clusters | 42,259 (pass 3); 17,514 at pass 1 when the analysis was made |
| candidate problem clusters | 168 (≥20 members, majority task intent) |
| web-search checks | 200 queries (the tool's session budget), 94 clusters with results |
| competitor records | 24 clusters, 142 competitor entries, roughly one selected page in three blocked automated fetching |
| Trends chain | 763 head phrases on one relative scale with per-placement rounding error |

## 3. What the free route can and cannot say about demand

It cannot give monthly searches. It can order phrases: Google Trends compares five phrases at a time, and a chain of
overlapping comparisons puts 763 head phrases on one scale (root "sourdough starter" = 100). Reference points on that
scale: "password manager" 84.5, "circuit breaker" 51.4, "docker compose" 31.8, "excel formulas" 10.5, "chess openings"
9.5, "roof leaking" 4.7, "obd2 codes" 1.5; brand phrases "xbox one" 156.5, "pcpartpicker" 12.0.

The finding that matters most: every finalist's exact head phrase is small on this scale ("cpu motherboard
compatibility" 0.5, "best settings for rivals" 0.6, "stl files for 3d printing" 0.6, "how to move abroad" 0.7, modpack
phrasings near zero). None of the Top 20 is a head-term opportunity. Each is a bet on a long tail of product-, game-,
file- or country-specific phrasings that no free source sums. The earlier statement in this project was that free
signals mis-order neighbouring clusters by about a factor of two; that still holds for ordering. Absolute size was
never available for free and is the single biggest open question for all five finalists. One month of keyword data
(about $50 with a SERP-data provider, or a Google Ads account's Keyword Planner ranges) converts the whole scale: look up
one reference phrase, divide, multiply.

## 4. The finalists

Ordering rule, in this order: a reproducible structured dataset or a computation on the user's own file that a chat
assistant cannot substitute; a product-level long tail of legitimate entry pages; measured fragmentation with no complete
tool in the checked results; advertising fit; maintenance cost.

| # | opportunity | fragmented share | tools in results | pages fetched | head phrase on chain |
|---|---|---|---|---|---|
| 1 | CPU × motherboard compatibility with minimum BIOS version | 0.56 | 1 | 6 of 8 | 0.5 |
| 2 | modpack hardware requirements (RAM, CPU, launcher, install path) | 0.67 | 0 | 5 of 8 | ~0 |
| 3 | per-game settings by platform, device and patch (two clusters) | 0.89 / 0.68 | 0 / 0 | 6 of 8 / 5 of 8 | 0.6 |
| 4 | STL printability check and repair | 0.39 | 0 | 5 of 8 | 0.6 |
| 5 | move-abroad visa eligibility by passport and occupation | 0.61 | 0 | 7 of 8 | 0.7 |

**#1, conditional.** A cross-brand database of which CPUs each motherboard accepts, with the minimum BIOS version, a
"flash before install" flag, and a page per board, per CPU and per pair. Why: the board makers publish the underlying
tables; four editorial pages in the results restate the same method and defer to those tables; the only CPU-first tool
is Intel-only and dated 2023; the BIOS step, which two fetched pages say decides whether the machine boots, appears
nowhere as data; the lookup cannot be answered from memory by a chat assistant without being wrong in a way that stops
a PC booting; the audience is buying hardware.

Why conditional: PCPartPicker could not be read (bot protection blocked every fetch), its brand phrase is searched
about 24 times more often than the generic problem phrase, and on general knowledge (unverified) it already shows a
coarse "BIOS update may be required" note. The gap is therefore narrow: exact minimum BIOS versions and per-pair landing
pages beside a strong incumbent. A ten-minute browser check (in `pcpartpicker_conclusion.md`) and one month of keyword
data for twenty board-plus-CPU phrases decide it.

**Why not the others as #1.** The modpack database needs continuous manual measurement; the settings database faces at
least three unverified incumbents (prosettings.net, thespike.gg, trophi.ai) and low-CPC traffic; the STL utility is a
tool more than a search destination and online repair tools exist outside the checked results; the visa engine carries
the highest correctness cost and its incumbents were not explored.

## 5. The Top 20 in one line each (full twenty-field records in `top20.md`)

1. CPU × motherboard compatibility (pc_building) — data gap beside a strong incumbent; conditional #1.
2. Best PC for modding / modpack requirements (gaming_pc) — specific need, measurable dataset nobody publishes; measurement cost.
3. Best settings for Marvel Rivals / Roblox RIVALS (gaming_pc) — highest fragmentation (0.89), two games under one name, unverified settings databases.
4. STL files for 3D printing (three_d_printing) — printability check on the user's file; repair tools exist outside the checked results.
5. How to move abroad (travel) — every page defers the real question; heavy multi-jurisdiction maintenance.
6. Warhammer 40k edition changes (tabletop) — best AI resilience in the set; rights risk and an official app.
7. KDP self-publishing cost and manuscript check (writing) — reproducible fee data plus file validation; vendor may already calculate.
8. OBD2 codes (automotive) — definitions saturated; vehicle-scoped diagnosis lacks data.
9. Garden pruning planner (gardening) — reproducible, repeat-use, low maintenance; entry queries answered inline.
10. Unity build errors (game_development) — version-aware error index; chat assistants already the first stop.
11. Best settings for Rocket League (gaming_pc) — second member of the settings family; top-ranked incumbents unverified.
12. Export settings for YouTube (video_editing) — real fragmentation, narrow and AI-exposed.
13. PC gaming emulators (gaming_pc) — choosing layer commodity; legality and ad policy limit it.
14. Wood stain quantity and species (diy_woodworking) — calculator gap; manufacturers likely own it.
15. Cities: Skylines patch and mod timeline (gaming_titles) — freshness product; likely covered by SteamDB and the wiki.
16. Renovation cost per square foot (real_estate) — saturated lead-gen; differentiating data proprietary.
17. Pet bird species selector (pets) — selector-plus-cost pattern; low-stakes explainer category.
18. Roof leaking (home_repair) — high CPC; would be another article set without cost data.
19. Houdini render settings (three_d_animation) — documentation gap; niche audience.
20. App-pair integration index (productivity) — vendor's own directory likely complete.
Reviewed, not advanced: Notion × Microsoft; door won't close; printer offline; performance review comments (highest
fragmentation of all, fully AI-replaceable).

## 6. Cross-cutting findings

- Fragmentation alone did not predict a defensible product. The four highest-fragmentation clusters that did not
  advance all failed the same test: no layer a chat assistant cannot replace. What survived was a structured dataset
  (compatibility tables, points and rules, fee tables), an operation on the user's own file, or stateful personal data.
- Compatibility is the rarest task intent in autocomplete: 3,362 of 647,999 kept queries, present in all 60 domains
  but thin everywhere; fix (36,349), how-to (65,298), calculate (39,638) and decide (35,139) dominate. Autocomplete
  under-represents long questions, so this is partly a property of the source.
- A cross-domain family that clustering split by game: 604 stored queries combine "mod" with compatibility, conflict,
  crash or "not working" across at least five games. It was not web-search checked (budget) and is listed as an
  observation, not a ranked candidate.
- Roughly one competitor page in three blocks automated reading (403, empty body, JavaScript-only). Any product that
  depends on scraping incumbents or vendors must plan for that.

## 7. Limitations, stated plainly

- No monthly search volume, traffic, revenue or user count anywhere; demand is relative and, for every finalist, small
  at the head-phrase level.
- The web-search tool is a US-only proxy with a 200-call session budget; 129 planned checks were not run, and 25 of
  the final Top 100 have no fragmentation data.
- Clusters are lexical and unreviewed; some head terms are odd; cluster ids change on re-clustering, so the analysed
  pass-1 clusters are mapped to the final clustering by majority vote of their members.
- PCPartPicker, the incumbent that decides the #1, could not be read.
- Autocomplete is a biased sample of demand: short, popular phrasings, three engines, English, US.

## 8. What to do next, in order

1. Ten minutes in a browser on PCPartPicker (steps in `pcpartpicker_conclusion.md`).
2. About $50 of keyword data for twenty product-name phrasings per finalist; the pipeline re-scores automatically
   when volumes are imported (`import-kp`, or the SERP-data adapter).
3. If the compatibility gap is open and the long tail is real, prototype with three board vendors' lists and measure
   indexing; if it is closed, take the modpack-requirements or settings family to the same test.
4. Optionally raise the web-search budget in the environment settings and check the 25 unchecked Top 100 rows and the
   mod-compatibility family.

## 9. Files

`final_report_full.md` (everything below in one file, with appendices of competitor syntheses and every checked
query's result URLs), `top5_and_no1.md`, `top20.md`, `top100_final_free_route.md`, `demand_relative.md`,
`pcpartpicker_conclusion.md`, `intent_matrix_pass3.md`, `candidates_pass{1,2,3}.md`; competitor records in
`data/competitors/`, authored judgments in `data/top20/`, raw evidence deltas in `data/exports/delta/`, code and
README in `opportunity-research/`.
