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

---

## Relative demand on the Google Trends chain (free route)

Every value is the 12-month mean Google Trends interest of the exact phrase, chained onto one scale where the root anchor "sourdough starter" = 100. Each placement carries the integer-rounding error of the comparison it was read from. This is a measured relative signal for the exact head phrase only: it does not sum the hundreds of long-tail phrasings in a cluster, and it is not a monthly search count. To convert any row to an absolute number, one calibration point is enough: look up one reference phrase in a keyword tool that reports monthly volume, divide by its value here, and multiply every other row by that factor (error about a factor of two for neighbouring rows, larger for rows marked unreliable).

### Top-20 head terms and related phrasings

| rank | phrase | chain value (root = 100) |
|---|---|---|
| 1 | cpu motherboard compatibility | 0.5 (±10%) |
| 2 | best pc for modding | ~0 (unreliable, ±101%) |
| 3 | best settings for rivals | 0.6 (±36%) |
| 4 | stl files for 3d printing | 0.6 (±18%) |
| 5 | how to move abroad | 0.7 (±38%) |
| 6 | warhammer 40k new edition | ~0 (unreliable, ±101%) |
| 7 | kdp self publishing | not placed (Trends throttled or term still queued) |
| 8 | obd2 codes | 1.5 (±3%) |
| 9 | garden pruning | 1.1 (±4%) |
| 10 | unity builds | ~0 (unreliable, ±102%) |
| 11 | best settings for rocket league | ~0 (unreliable, ±58%) |
| 12 | export settings youtube | not placed (Trends throttled or term still queued) |
| 13 | pc gaming emulator | ~0 (unreliable, ±102%) |
| 14 | wood stain remover | 1.9 (±21%) |
| 15 | cities skylines | 16.2 (±5%) |
| 16 | renovation cost per square foot | 0.9 (±5%) |
| 17 | pet bird care | 0.4 (±40%) |
| 18 | roof leaking | 4.7 (±3%) |
| 19 | houdini render setting | not placed (Trends throttled or term still queued) |
| 20 | zapier excel integration | not placed (Trends throttled or term still queued) |
| 21 | does notion work with microsoft | ~0 (unreliable, ±100%) |
| 22 | door won't close | not placed (Trends throttled or term still queued) |
| 23 | printer offline error | ~0 (unreliable, ±101%) |
| 24 | performance review comments | 1.3 (±7%) |
| 1 (related) | motherboard cpu support list | ~0 (unreliable, ±102%) |
| 1 (related) | bios update for new cpu | ~0 (unreliable, ±102%) |
| 1 (related) | pcpartpicker | 12.0 (±2%) |
| 2 (related) | minecraft modpack requirements | ~0 (unreliable, ±102%) |
| 3 (related) | best settings for marvel rivals | not placed (Trends throttled or term still queued) |
| 4 (related) | stl file repair | not placed (Trends throttled or term still queued) |
| 7 (related) | kdp royalty calculator | not placed (Trends throttled or term still queued) |

### Reference phrases on the same scale

| phrase | chain value |
|---|---|
| sourdough starter | 100.0 (±0%) |
| password manager | 84.5 (±2%) |
| circuit breaker | 51.4 (±2%) |
| docker compose | 31.8 (±2%) |
| github actions | 27.7 (±4%) |
| docker desktop | 11.1 (±2%) |
| excel formulas | 10.5 (±7%) |
| chess openings | 9.5 (±6%) |
| guitar chords | 82.4 (±12%) |
| solar panels | 105.0 (±18%) |
| job resume | 37.0 (±4%) |
| wedding planning | 12.7 (±5%) |
| warhammer 40k | 61.5 (±2%) |
| unreal engine | 39.8 (±2%) |
| cities skylines | 16.2 (±5%) |
| ev charging | 58.1 (±7%) |
| workout plan | 31.9 (±8%) |
| logo design | 41.0 (±17%) |
| davinci resolve | 65.9 (±16%) |
| iphone 17 pro max | 284.4 (±6%) |
| xbox one | 156.5 (±2%) |
| genshin impact | 82.8 (±2%) |
| roof leaking | 4.7 (±3%) |
| obd2 codes | 1.5 (±3%) |

Reading: a phrase at 12 is searched about one-eighth as often as "sourdough starter" and about one-seventh as often as "password manager", within the stated error. Brand phrases ("pcpartpicker", "iphone 17 pro max") are shown because they bound the audience: a problem phrase far below its incumbent's brand phrase means most people go straight to the incumbent.


---

## Incumbent check: PCPartPicker (2026-09-21)

### What was attempted, and what blocked it (measured)

- The research environment's fetch tool returned HTTP 403 for pcpartpicker.com (homepage and FAQ).
- A direct request with a desktop browser user agent to three URLs (FAQ, motherboard product listing, a forum topic about
  BIOS updates) returned 403 with Cloudflare challenge tokens in the body: the site sits behind bot protection.
- web.archive.org is not reachable from this environment, and the web-search tool's 200-call session budget was spent.
- No page of pcpartpicker.com was therefore read in this project. Nothing below about its features is verified here.

### What is measured about it

- In the web-search proxy results for the two checked queries of the compatibility cluster ("cpu motherboard
  compatibility", "best cpu compatible with my motherboard"), pcpartpicker.com held 1 of 18 results, titled
  "PCPartPicker - Motherboard + CPU compatibility". Present, not dominant, in that proxy.
- A fetched editorial page (cgdirector.com) describes it as an aggregator of "noted compatibilities" and sends readers to
  board makers' CPU support lists for the authoritative answer.
- On the Google Trends chain (root "sourdough starter" = 100): the brand phrase "pcpartpicker" reads 12.0 (±2%); the
  problem phrase "cpu motherboard compatibility" reads 0.5 (±10%); "motherboard cpu support list" and "bios update for
  new cpu" read near zero with unreliable error. The brand is searched roughly 24 times more often than the generic
  problem phrase. Measured for these exact phrases only; long-tail product-name phrasings are not summed here.

### Background knowledge, stated as such (not verified in this project)

PCPartPicker's part list checks socket, chipset, form factor, memory type, cooler clearance and power. On some CPU and
motherboard pairings it shows a compatibility note saying a BIOS update may be required before the CPU is supported, and
that updating may need an older supported CPU. To my knowledge it does not show the minimum BIOS version for a specific
board model, does not reproduce board makers' per-model CPU support tables, and does not publish a standalone page per
CPU-board pair. Treat every sentence in this paragraph as a hypothesis to check in a browser.

### Conclusion (inference)

PCPartPicker is the dominant destination for "will these parts work together" and already answers the coarse form of the
##1 problem; the 24-to-1 brand-to-problem ratio says most people who have the problem go straight to it. What remains is
the fine form: the exact minimum BIOS version, the flash-before-install decision, and a search-landing page per pair,
which no checked result provides and which PCPartPicker, on general knowledge, only gestures at with a generic note.
That gap is real on the evidence but narrower than "no tool exists", and the incumbent holds the data pipeline that
could close it. The #1 recommendation therefore stands only as a narrow data gap beside a strong incumbent, with
head-phrase demand that is small on every free signal; its case rests on the long tail of product-name queries, which
this route cannot size.

### Ten-minute verification for a person with a browser

1. Build a list on pcpartpicker.com with an AMD B450 board and a Ryzen 5 5600X (and an AM5 B650 board with the newest
   Ryzen). Copy the exact compatibility note text. Does it name a minimum BIOS version, or only "may need an update"?
2. Open one motherboard product page. Is there a CPU support list with BIOS versions, or only specifications?
3. Search Google for "b450 tomahawk max 5600x bios version" and "b650 bios version for 9800x3d". Record which sites hold
   the top ten and whether any page states the version. If pcpartpicker.com or a board maker's page answers it inline,
   the gap is closed; if forums and blogs answer it, the gap is open.
4. Optional: one month of keyword data for twenty board-name plus CPU-name phrases sizes the long tail.

---

## Top 5 finalists and the #1 recommendation (free route, evidence as of 2026-09-21)

Basis: 24 clusters carried through Phase 7 (competitor pages fetched, twelve questions each), drawn from the 94
clusters with web-search fragmentation evidence, drawn from 168 candidate problem clusters, drawn from 17,514
lexical clusters over 265,088 cleaned autocomplete queries (pass 1). Demand is relative only: no monthly volume,
traffic, revenue or user count was measured, and none is stated. Ordering rule for the finalists, in this order:
a reproducible structured dataset or computation that a chat assistant cannot substitute; a product-level long tail
of legitimate entry pages; measured fragmentation with no complete tool in the checked results; advertising fit;
maintenance cost. Where a likely incumbent could not be verified, that is stated as the primary risk, not ignored.

### Finalists

| # | cluster | frag | tool hits | pages fetched | why it advances | what would stop it |
|---|---|---|---|---|---|---|
| 1 | CPU x motherboard compatibility with minimum BIOS version | 0.56 | 1 | 6 of 8 | public per-model vendor lists, AI-resistant lookup layer, tens of thousands of pair pages, hardware purchase intent | PCPartPicker already surfacing BIOS minimums per pair (unverified, site blocked fetching) |
| 2 | modpack hardware requirements (RAM, CPU, launcher, install path) | 0.67 | 0 | 5 of 8 | specific need every fetched page answers generically; measurable dataset nobody publishes | measurement cost per pack update; community answers unread (three sources blocked) |
| 3 | per-game settings by platform, device and patch (two clusters) | 0.89 / 0.68 | 0 / 0 | 6 of 8 / 5 of 8 | highest fragmentation in the set, high repeat use, values readable from the games | prosettings.net, thespike.gg and trophi.ai may already be the database (unverified) |
| 4 | STL printability check and repair | 0.39 | 0 | 5 of 8 | computation on the user's file, no licensed data, high repeat use | online repair tools exist outside the checked results (not verified) |
| 5 | move-abroad visa eligibility by passport and occupation | 0.61 | 0 | 7 of 8 | every fetched page defers the real question; high-value adjacent advertisers | maintenance across jurisdictions and liability; incumbents unexplored |

Next two, not finalists: KDP cost and manuscript checker (Amazon may already publish a royalty calculator, not
verified); Warhammer 40k edition tracker (best AI resilience in the set, capped by rights risk and an official app).

### #1: a cross-brand CPU-to-motherboard compatibility database with minimum BIOS versions

**What it is.** One page per motherboard model and one per CPU, each listing every supported pairing with the
minimum BIOS version, its release date, and a "flash before install" flag; board-first and CPU-first lookups; a
one-paragraph explanation of socket, chipset, TDP and BIOS for the pairing on screen. Built from the board makers'
published CPU support lists, which the fetched pages describe as a consistent "Brand Name, BIOS version, Date" table.

**Why this one (measured).** Of the 18 checked results, four editorial pages restate the same method and defer to
vendor lists; the only CPU-first lookup is Intel-only and dated 2023; the board-first tool that loaded checks memory
and SSDs, not CPUs; the BIOS step, which two fetched pages say decides whether the machine boots, exists nowhere in
the checked results as data. No fetched page completes the task.

**Why this one (inference).** The lookup layer cannot be answered from memory by a chat assistant without being
wrong in the way that stops a PC booting, so assistants and search engines with retrieval would cite such a table
rather than replace it. Every board and CPU pair is a legitimate long-tail page. Hardware purchase intent fits
display advertising and retailer affiliate links on the same page.

**Why not the others.** The modpack database needs continuous manual measurement; the settings database faces at
least three unverified incumbents and low-CPC traffic; the STL utility is a tool more than a search destination and
has unverified competitors; the visa engine has the highest correctness cost in the set.

**What must be verified before building (in order).**
1. Open PCPartPicker's compatibility checker in a browser and record whether it shows the minimum BIOS version per
   CPU-board pair, or only a generic "may need a BIOS update" note. If it shows the version, the opportunity narrows
   to long-tail pair pages and explanation.
2. Fetch three vendors' CPU support lists in a browser (ASRock, MSI, Gigabyte, ASUS) and confirm the table format
   and whether the pages are served without JavaScript; ASRock returned an empty body to an automated fetch.
3. Check whether AMD publishes a compatibility tool comparable to Intel's.
4. Size the demand with one month of keyword data for board-level and CPU-level queries ("<board> cpu support",
   "<cpu> compatible motherboard", "<board> bios version for <cpu>"); the free route cannot do this.

**Evidence quality.** Fragmentation and competitor evidence moderate; demand evidence weak: the two related pass-1
clusters hold 25 and 26 members and neither head term is placed on the Trends chain. The recommendation rests on the
structure of the problem and the shape of the data, not on a measured volume.

### Demand check after placement (added after the Trends chain reached the finalists)

The exact head phrases of the finalists were placed on the Trends chain after the ranking was made. All are small:
"cpu motherboard compatibility" 0.5, "best settings for rivals" 0.6, "stl files for 3d printing" 0.6, "how to move
abroad" 0.7, "best pc for modding" and "minecraft modpack requirements" near zero with unreliable error, against
"sourdough starter" 100, "excel formulas" 10.5, "obd2 codes" 1.5, "roof leaking" 4.7 and the brand phrase
"pcpartpicker" 12.0. Two consequences, stated plainly:

- None of the five is a head-term opportunity. Each is a long-tail bet: many product-, game-, file- or country-specific
  phrasings that no free source sums. That is consistent with the brief's problem-family framing, and it means the
  absolute size of every finalist is unmeasured, not merely imprecise.
- For the #1, the incumbent's brand phrase is searched about 24 times more often than the generic problem phrase, so
  most people with the problem already go to PCPartPicker. The #1 stands only as a narrow data gap (minimum BIOS
  version per pair, per-pair landing pages) beside a strong incumbent; see pcpartpicker_conclusion.md. The ranking
  among the five does not change, because it was made on data structure and resilience rather than on head-phrase
  demand, but the first thing to buy is one month of keyword data for twenty product-name phrasings per finalist.

### A family seen across domains, not ranked

604 stored queries combine "mod" with compatibility, conflict, crash or "not working" across at least five games
("cities skylines incompatible mods spreadsheet", "how to check if mods are compatible", "minecraft server mods
compatibility", "pc gaming mods compatible with skyrim special edition"). Lexical clustering split them by game, so
no single cluster reached the candidate list and none was SERP-checked. Whether that family is fragmented and
unserved is unknown from this data; it is listed here because it recurs across domains, not because of any prior
interest in it.

---

## Final free-route Top 100 problem clusters (pass 3: three seeds per subdomain, 647,999 kept queries, 42,259 clusters)

Rows with a blank `frag` column are clusters that rose into the Top 100 only after passes 2 and 3 and have no web-search check (the tool's 200-call session budget was spent on pass-1 candidates); they are ordered among themselves by organic problem members. The 75 checked rows keep the fragmentation ordering. No monthly search volume exists in this dataset.


Filter: members >= 20 and problem-intent share >= 0.45. Order: clusters with >= 10 checked results first, by fragmented share desc, then tool hits asc, then organic problem members desc; unchecked clusters after, by organic problem members desc, corroboration desc. No monthly search volume exists in this dataset; `trends` is the head term's relative interest on the chained Google Trends scale (root anchor = 100) with its rounding error, blank when not yet placed. `frag` is the fragmented share (community + editorial + aggregator) of the web-search results for the cluster's checked queries, `tool` the number of those results from a tool site or with a tool-naming title, `vendor` the product-vendor share, `uncl` the unclassified share, `n` the results checked (blank = not checked; the web-search tool is a US-only proxy, not Google). Cluster labels are lexical.

| # | head term | domain | members | problem (share) | organic | corr | intents | trends (±%) | frag | tool | vendor | uncl | n | problem-intent examples (organic first) |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | best settings for rivals | gaming_pc | 30 | 30 (1.0) | 3 | 8 | decide:29, how_to:1 | 0.6 (±36.4) | 0.89 | 0 | 0.00 | 0.06 | 18 | best settings for rivals; best settings for rivals roblox mobile; best settings for roblox rivals |
| 2 | apple watch ultra | consumer_tech | 37 | 20 (0.54) | 6 | 17 | informational:15, compare:10, how_to:4, calculate:4 | 64.3 (±7.4) | 0.79 | 0 | 0.11 | 0.11 | 19 | apple watch 11 vs ultra 3; apple watch ultra 3 vs ultra 2; apple watch ultra 4 vs 3 |
| 3 | performance review phrases | careers_jobs | 44 | 44 (1.0) | 4 | 10 | optimize:39, fix:2, decide:1, how_to:1 | 0.5 (±20.6) | 0.72 | 0 | 0.00 | 0.17 | 18 | performance review phrases; performance review comments; performance review comments for employees |
| 4 | unreal engine | game_development | 133 | 72 (0.54) | 10 | 38 | informational:44, how_to:23, calculate:18, commercial_nav:17 | 39.8 (±2.5) | 0.68 | 0 | 0.11 | 0.05 | 19 | unreal engine 5 tutorial; unreal engine tutorial for beginners; unreal engine 4 vs 5 |
| 5 | best settings for rocket league | gaming_pc | 38 | 31 (0.82) | 4 | 10 | decide:24, fix:7, informational:7 | ~0 (unreliable, ±57.5) | 0.68 | 0 | 0.05 | 0.26 | 19 | best settings for rocket league; controller not working on rocket league pc; controller not working on rocket league |
| 6 | best job highest salary | careers_jobs | 22 | 11 (0.5) | 7 | 0 | informational:11, decide:10, calculate:1 |  | 0.67 | 0 | 0.00 | 0.22 | 18 | best job highest salary; which job pays highest salary; which jobs have highest salary |
| 7 | export settings youtube | video_editing | 21 | 21 (1.0) | 4 | 2 | convert:10, decide:3, alternative:3, calculate:3 |  | 0.67 | 0 | 0.28 | 0.06 | 18 | export settings youtube; export settings for youtube video; best export settings youtube |
| 8 | best pc for modding | gaming_pc | 23 | 12 (0.52) | 3 | 3 | informational:10, how_to:7, decide:4, commercial_nav:1 | ~0 (unreliable, ±101.3) | 0.67 | 0 | 0.00 | 0.17 | 18 | how to play modded games on pc; pc game modding tutorial; best pc for modding and gaming |
| 9 | blog post | writing_publishing | 68 | 38 (0.56) | 5 | 15 | informational:27, calculate:10, generate:8, decide:6 | 28.4 (±4.0) | 0.67 | 12 | 0.11 | 0.00 | 18 | blog post template; blog post template free; blog post ideas generator |
| 10 | cities skylines | gaming_titles | 120 | 63 (0.53) | 3 | 34 | informational:48, how_to:11, track:9, commercial_nav:9 | 16.2 (±4.8) | 0.63 | 0 | 0.11 | 0.16 | 19 | cities skylines 1 vs 2; cities skylines 2 update; cities skylines not working |
| 11 | maker's mark distillery | collecting_antiques | 29 | 29 (1.0) | 5 | 5 | generate:26, calculate:2, decide:1 |  | 0.61 | 0 | 0.11 | 0.28 | 18 | maker's mark distillery tour; maker's mark distillery; maker's mark distillery photos |
| 12 | github actions | software_dev_ops | 80 | 50 (0.62) | 4 | 24 | informational:28, calculate:19, how_to:14, alternative:6 | 27.7 (±3.9) | 0.61 | 1 | 0.33 | 0.06 | 18 | github actions tutorial; github actions tutorial for beginners; github actions cost |
| 13 | pc gaming emulator | gaming_pc | 34 | 17 (0.5) | 3 | 1 | informational:12, decide:6, commercial_nav:5, alternative:4 | ~0 (unreliable, ±101.6) | 0.61 | 1 | 0.00 | 0.28 | 18 | best pc gaming emulator; top 5 gaming emulator for pc; top 10 gaming emulator for pc |
| 14 | password manager | cybersecurity | 96 | 47 (0.49) | 5 | 30 | informational:45, decide:22, calculate:7, find:6 | 84.5 (±1.7) | 0.61 | 3 | 0.06 | 0.11 | 18 | best easy to use password manager; password manager problem; best password manager tool |
| 15 | android app development | software_dev_mobile | 21 | 12 (0.57) | 3 | 8 | informational:8, how_to:7, calculate:3, commercial_nav:1 | 3.8 (±4.8) | 0.61 | 3 | 0.11 | 0.11 | 18 | android app development tutorial; android app development cost; android app development tutorial for beginners |
| 16 | vue js for beginners | software_dev_web | 43 | 27 (0.63) | 7 | 9 | how_to:15, informational:14, generate:3, alternative:2 | ~0 (unreliable, ±100.7) | 0.61 | 10 | 0.00 | 0.33 | 18 | vue js tutorial for beginners; vue js 3 tutorial for beginners; how to use vue js |
| 17 | door won't close | home_repair | 21 | 18 (0.86) | 5 | 4 | fix:16, informational:3, how_to:1, decide:1 |  | 0.58 | 0 | 0.05 | 0.05 | 19 | door won't close; door won't close all the way; door won't close at top |
| 18 | zapier excel integration | productivity_office | 24 | 13 (0.54) | 6 | 2 | informational:8, commercial_nav:3, find:3, decide:2 |  | 0.56 | 1 | 0.28 | 0.11 | 18 | list of zapier integrations; zapier integrations full list; zapier api integration tutorial |
| 19 | small business seo | small_business | 43 | 24 (0.56) | 5 | 6 | informational:19, decide:8, calculate:7, alternative:3 | 4.4 (±3.2) | 0.56 | 1 | 0.06 | 0.33 | 18 | is seo worth it for small business; best seo software for small business; best small business seo tips |
| 20 | cpu motherboard compatibility | pc_building | 25 | 12 (0.48) | 3 | 3 | informational:13, decide:3, find:2, alternative:2 |  | 0.56 | 1 | 0.17 | 0.17 | 18 | cpu motherboard compatibility list; best cpu compatible with my motherboard; cpu and motherboard compatibility calculator |
| 21 | game maker studio | game_development | 24 | 21 (0.88) | 5 | 6 | generate:13, calculate:4, commercial_nav:3, how_to:3 | 0.8 (±5.0) | 0.55 | 5 | 0.15 | 0.00 | 20 | game maker studio; game maker studio 2; game maker studio 2 tutorial |
| 22 | after effects | video_editing | 113 | 58 (0.51) | 5 | 37 | informational:40, commercial_nav:15, calculate:13, alternative:9 | 129.6 (±15.6) | 0.53 | 3 | 0.16 | 0.16 | 19 | after effects alternative; after effects free alternative; after effects tutorial |
| 23 | renovation cost per square foot | real_estate_housing | 42 | 41 (0.98) | 2 | 3 | calculate:38, decide:1, fix:1, how_to:1 | 0.9 (±4.6) | 0.53 | 6 | 0.00 | 0.21 | 19 | renovation cost per square foot; renovation costs per square foot uk; renovation cost per square foot ontario |
| 24 | apple watch series | consumer_tech | 50 | 26 (0.52) | 4 | 16 | informational:21, how_to:11, calculate:9, commercial_nav:3 |  | 0.50 | 0 | 0.11 | 0.28 | 18 | apple watch series 10 vs 12; apple watch series 11 vs 12; apple watch series 11 vs 10 |
| 25 | printer offline error | it_support | 42 | 25 (0.6) | 4 | 9 | fix:17, informational:14, commercial_nav:3, alternative:3 | ~0 (unreliable, ±101.1) | 0.50 | 0 | 0.22 | 0.06 | 18 | printer offline error; printer offline issue; printer offline fix |
| 26 | excel formulas | data_ai | 93 | 53 (0.57) | 4 | 28 | informational:26, commercial_nav:14, fix:11, find:10 | 10.5 (±6.7) | 0.50 | 2 | 0.06 | 0.28 | 18 | excel formula list with examples pdf; excel formulas tutorial; excel all formulas list pdf guide |
| 27 | unity builds | game_development | 33 | 19 (0.58) | 3 | 2 | informational:13, fix:6, optimize:3, calculate:2 | ~0 (unreliable, ±101.6) | 0.47 | 1 | 0.37 | 0.00 | 19 | unity build error; unity build not working; unity building tutorial |
| 28 | mattress product | shopping_products | 29 | 14 (0.48) | 3 | 0 | informational:14, find:3, generate:3, decide:2 |  | 0.44 | 0 | 0.00 | 0.22 | 18 | best product mattress; product review best mattress; what type of product is a mattress |
| 29 | photoshop lightroom | photography | 53 | 29 (0.55) | 5 | 11 | informational:19, calculate:8, alternative:6, commercial_nav:5 | 3.3 (±6.5) | 0.44 | 2 | 0.22 | 0.22 | 18 | photoshop lightroom cost; photoshop lightroom tutorial; photoshop lightroom alternative |
| 30 | wedding planning | relationships_social | 87 | 40 (0.46) | 7 | 22 | informational:42, decide:12, calculate:7, find:6 | 12.7 (±5.2) | 0.44 | 6 | 0.06 | 0.28 | 18 | wedding planning list; wedding planning list free; wedding planning template |
| 31 | google sheets | productivity_office | 110 | 63 (0.57) | 6 | 31 | informational:35, how_to:16, commercial_nav:12, calculate:12 | ~257 (unreliable, ±71.4) | 0.44 | 9 | 0.17 | 0.33 | 18 | google sheets tutorial for beginners; best google sheets templates; google sheets tutorial |
| 32 | llm chatgpt | data_ai | 43 | 21 (0.49) | 5 | 5 | informational:20, compare:4, alternative:4, decide:3 | 4.2 (±6.6) | 0.42 | 1 | 0.00 | 0.47 | 19 | chatgpt llm cost; chatgpt llm tutorial; llm chatgpt alternative |
| 33 | photography cameras | photography | 58 | 29 (0.5) | 4 | 11 | informational:24, decide:12, calculate:7, commercial_nav:5 | 5.8 (±2.9) | 0.42 | 1 | 0.00 | 0.53 | 19 | best photography cameras for beginners; best photography tips camera; best photography camera review |
| 34 | pc gaming emulator for android | gaming_pc | 22 | 10 (0.45) | 5 | 2 | informational:9, decide:7, commercial_nav:3, fix:2 | ~0 (unreliable, ±100.0) | 0.42 | 2 | 0.05 | 0.26 | 19 | best gaming emulator for pc windows 10; best gaming emulator for pc windows 11; best windows gaming emulator for android |
| 35 | database postgresql | software_dev_web | 48 | 45 (0.94) | 6 | 6 | find:26, calculate:4, decide:3, commercial_nav:3 | 5.6 (±3.0) | 0.39 | 0 | 0.28 | 0.28 | 18 | database postgresql; database postgresql tutorial; database postgresql free |
| 36 | stl files for 3d printing | three_d_printing | 45 | 24 (0.53) | 5 | 9 | informational:17, how_to:9, decide:4, commercial_nav:4 | 0.6 (±17.6) | 0.39 | 0 | 0.28 | 0.22 | 18 | best stl files for 3d printing; how to design stl files for 3d printing; where to find stl files for 3d printing |
| 37 | pet bird guide | pets | 22 | 13 (0.59) | 2 | 6 | informational:8, decide:6, calculate:3, find:2 |  | 0.39 | 0 | 0.00 | 0.44 | 18 | best pet birds for beginners; pet bird types list; how to pet bird |
| 38 | devops azure | software_dev_ops | 64 | 38 (0.59) | 3 | 14 | informational:21, how_to:10, calculate:8, fix:5 | 16.0 (±3.1) | 0.39 | 2 | 0.33 | 0.17 | 18 | devops azure tutorial; azure devops and azure are same or different; what is difference between azure and azure devops |
| 39 | study sat | education_study | 68 | 35 (0.51) | 4 | 10 | informational:31, decide:14, calculate:6, how_to:5 | 7.0 (±6.8) | 0.39 | 3 | 0.00 | 0.44 | 18 | best sat study online; how to study for sat in one week; is studying for the sat worth it |
| 40 | study flashcards | education_study | 57 | 28 (0.49) | 8 | 8 | informational:29, decide:11, how_to:5, generate:4 | 2.0 (±6.5) | 0.39 | 4 | 0.11 | 0.17 | 18 | best way to study flashcards; how to study flashcards fast; best study flashcards app |
| 41 | running plan | fitness_health | 61 | 59 (0.97) | 5 | 13 | plan:26, decide:10, optimize:6, generate:6 | 15.3 (±2.5) | 0.39 | 4 | 0.00 | 0.39 | 18 | running plan for beginners; running plan to get faster; running plan |
| 42 | houdini render setting | three_d_animation | 25 | 23 (0.92) | 4 | 4 | how_to:20, convert:3, informational:2 |  | 0.35 | 0 | 0.40 | 0.05 | 20 | how to render in houdini; import fbx into houdini; houdini fbx character import |
| 43 | aws lambda | software_dev_ops | 50 | 29 (0.58) | 3 | 15 | informational:20, how_to:9, calculate:8, alternative:5 | 13.1 (±4.2) | 0.33 | 2 | 0.44 | 0.22 | 18 | aws lambda cost; aws lambda tutorial; aws lambda example tutorial |
| 44 | apple watch | consumer_tech | 114 | 72 (0.63) | 8 | 33 | informational:33, how_to:15, calculate:15, fix:12 | ~962 (unreliable, ±100.8) | 0.33 | 3 | 0.22 | 0.11 | 18 | best apple watch for beginners; apple watch tutorial for beginners; apple watch 11 tutorial for beginners |
| 45 | meal prep | cooking_baking | 115 | 58 (0.5) | 4 | 35 | informational:56, calculate:15, how_to:10, decide:8 | ~126 (unreliable, ±105.8) | 0.33 | 3 | 0.11 | 0.33 | 18 | how to meal prep for beginners; is meal prep worth it; meal prep template pdf |
| 46 | daw ableton | audio_music_production | 39 | 19 (0.49) | 4 | 7 | informational:17, alternative:4, calculate:3, commercial_nav:3 | 0.9 (±8.9) | 0.33 | 4 | 0.00 | 0.22 | 18 | ableton daw tutorial; how much is ableton daw; daw ableton 12 tutorial |
| 47 | docker and kubernetes | software_dev_ops | 49 | 29 (0.59) | 6 | 10 | informational:19, how_to:12, compare:9, alternative:2 | 1.2 (±3.8) | 0.33 | 5 | 0.17 | 0.22 | 18 | docker kubernetes tutorial; docker vs kubernetes difference; is kubernetes better than docker |
| 48 | inventory management | logistics_operations | 83 | 39 (0.47) | 7 | 22 | informational:41, calculate:12, decide:8, how_to:8 | 35.5 (±12.8) | 0.33 | 6 | 0.06 | 0.22 | 18 | best inventory management software free; inventory management software cost; inventory management software list |
| 49 | ev charging | automotive | 61 | 31 (0.51) | 5 | 16 | informational:25, calculate:10, commercial_nav:5, decide:4 | 58.1 (±7.1) | 0.28 | 3 | 0.11 | 0.44 | 18 | ev charging cost; ev charging cost calculator; ev charging calculator |
| 50 | cleaning schedule | parenting_family | 51 | 49 (0.96) | 12 | 14 | plan:26, generate:8, decide:6, calculate:4 | 14.7 (±2.5) | 0.28 | 6 | 0.11 | 0.33 | 18 | cleaning schedule; cleaning schedule templates free; cleaning schedule app |
| 51 | unreal engine blueprints | game_development | 78 | 46 (0.59) | 10 | 15 | informational:29, how_to:21, alternative:6, fix:5 | 0.4 (±9.5) | 0.28 | 6 | 0.17 | 0.28 | 18 | unreal engine blueprint generator; unreal engine blueprint list; unreal engine 5 blueprint tutorial |
| 52 | bambu labs | three_d_printing | 72 | 42 (0.58) | 3 | 14 | informational:25, fix:14, how_to:8, alternative:6 | 31.3 (±3.8) | 0.26 | 2 | 0.42 | 0.26 | 19 | how to update bambu lab; bambu lab issue; bambu lab update issues |
| 53 | board game rules | tabletop_hobby_games | 40 | 18 (0.45) | 3 | 7 | informational:22, calculate:4, fix:3, generate:3 | 6.1 (±4.8) | 0.26 | 8 | 0.11 | 0.37 | 19 | board game rules template; make your own board game rules; board game rules template pdf |
| 54 | workout plan | fitness_health | 78 | 68 (0.87) | 5 | 12 | plan:30, decide:11, generate:11, informational:7 | 31.9 (±8.1) | 0.22 | 10 | 0.17 | 0.22 | 18 | workout plan for beginners; workout plan; workout plan for week |
| 55 | travel itinerary | travel | 65 | 37 (0.57) | 4 | 16 | informational:22, generate:13, calculate:7, commercial_nav:6 | 19.1 (±3.4) | 0.22 | 12 | 0.28 | 0.33 | 18 | travel itinerary template; travel itinerary template free; travel itinerary tutorial |
| 56 | solar panels | energy_sustainability | 72 | 42 (0.58) | 4 | 27 | informational:24, calculate:17, commercial_nav:6, decide:5 | 105.0 (±18.0) | 0.17 | 8 | 0.00 | 0.33 | 18 | solar panels cost; solar panels worth it; how to make a simple solar panel |
| 57 | protein intake for muscle gain | fitness_health | 53 | 28 (0.53) | 6 | 4 | informational:25, calculate:21, decide:3, how_to:2 | 0.7 (±21.5) | 0.17 | 11 | 0.06 | 0.33 | 18 | protein intake calculator for muscle gain; protein intake for muscle gain per kg; protein intake for muscle gain per day |
| 58 | small business invoice | small_business | 62 | 32 (0.52) | 11 | 18 | informational:26, decide:11, generate:11, commercial_nav:4 | 2.0 (±26.9) | 0.17 | 12 | 0.17 | 0.33 | 18 | small business invoice template; best small business invoice software; best invoice software for small business free |
| 59 | anime watch list | reading_media | 49 | 29 (0.59) | 8 | 0 | informational:18, find:15, decide:10, track:2 | 1.4 (±6.8) | 0.15 | 6 | 0.05 | 0.45 | 20 | where to watch anime; free anime watch list; reddit where watch anime |
| 60 | holiday christmas | events_seasonal | 42 | 20 (0.48) | 4 | 2 | informational:22, calculate:6, decide:3, alternative:2 | 72.0 (±32.4) | 0.11 | 1 | 0.00 | 0.72 | 18 | why is christmas best holiday; what holiday is it near christmas; what kind of holiday is christmas |
| 61 | heartbeat at 8 weeks | parenting_family | 46 | 30 (0.65) | 3 | 1 | calculate:23, informational:16, how_to:4, compare:2 | 1.0 (±10.5) | 0.11 | 6 | 0.00 | 0.21 | 19 | how many weeks pregnant am i; how many weeks pregnant are you; how many weeks pregnant will i be |
| 62 | game maker | game_development | 92 | 76 (0.83) | 8 | 26 | generate:41, commercial_nav:14, how_to:13, decide:9 | 22.4 (±3.7) | 0.11 | 8 | 0.28 | 0.22 | 18 | game maker free; game maker; game maker 2 |
| 63 | job resume | careers_jobs | 94 | 43 (0.46) | 4 | 14 | informational:45, generate:11, calculate:9, decide:8 | 37.0 (±4.1) | 0.11 | 13 | 0.06 | 0.28 | 18 | job resume template; job resume generator; job resume template free |
| 64 | energy bills | energy_sustainability | 40 | 19 (0.47) | 4 | 8 | informational:20, calculate:7, alternative:3, decide:3 | 4.8 (±2.9) | 0.11 | 18 | 0.00 | 0.68 | 19 | energy bill template; energy bill calculator; energy bill cost calculator |
| 65 | tax return | personal_finance | 76 | 46 (0.61) | 1 | 15 | informational:29, decide:10, calculate:10, generate:6 | 241.5 (±48.1) | 0.10 | 10 | 0.00 | 0.60 | 10 | tax return calculator; tax return update; how to tax return online |
| 66 | card value | collecting_antiques | 32 | 15 (0.47) | 3 | 7 | informational:15, decide:4, generate:3, alternative:2 | 94.8 (±7.5) | 0.06 | 2 | 0.28 | 0.50 | 18 | card value list; best card value app; card value calculator |
| 67 | running workout 5k | fitness_health | 34 | 28 (0.82) | 4 | 9 | plan:18, optimize:6, informational:5, decide:2 | ~0 (unreliable, ±100.9) | 0.06 | 8 | 0.06 | 0.33 | 18 | running plan for 5k; garmin 5k training plan review; running plan for 5k in 20 minutes |
| 68 | index funds | personal_finance | 61 | 29 (0.48) | 3 | 16 | informational:30, decide:13, calculate:5, optimize:2 | 33.0 (±7.8) | 0.06 | 10 | 0.00 | 0.50 | 18 | index fund calculator; best index funds for beginners; index funds list |
| 69 | lesson plans | education_study | 73 | 65 (0.89) | 6 | 16 | plan:28, generate:14, how_to:6, commercial_nav:5 | 12.5 (±3.6) | 0.06 | 11 | 0.11 | 0.33 | 18 | lesson plan template; lesson plan example; lesson plan examples |
| 70 | graphic design fonts | graphic_design | 46 | 21 (0.46) | 5 | 6 | informational:20, commercial_nav:5, decide:4, generate:4 | 0.3 (±10.9) | 0.06 | 11 | 0.00 | 0.61 | 18 | best graphic design fonts; graphic design fonts generator; best graphic design fonts reddit |
| 71 | android studio update gradle version | software_dev_mobile | 40 | 20 (0.5) | 5 | 11 | informational:18, track:8, find:6, how_to:6 | ~0 (unreliable, ±100.0) | 0.05 | 0 | 0.80 | 0.10 | 20 | android studio update gradle version; android gradle plugin version list; android gradle plugin latest version |
| 72 | train schedule | transport_commuting | 29 | 28 (0.97) | 3 | 3 | plan:13, generate:3, find:2, decide:2 | 76.4 (±6.6) | 0.00 | 0 | 0.00 | 0.10 | 20 | train schedule; train schedule update; train schedule app |
| 73 | maker's mark | collecting_antiques | 47 | 40 (0.85) | 5 | 9 | generate:24, calculate:6, alternative:4, fix:4 | 6.2 (±4.0) | 0.00 | 5 | 0.06 | 0.33 | 18 | maker's mark; maker's mark 46; maker's mark review |
| 74 | pregnancy week calendar by lmp | parenting_family | 20 | 15 (0.75) | 13 | 2 | calculate:15, informational:3, commercial_nav:2 |  | 0.00 | 9 | 0.10 | 0.50 | 10 | pregnancy week calculator by lmp; pregnancy week calculator by lmp tamil; pregnancy week calculator by lmp telugu |
| 75 | study flashcards ai | education_study | 24 | 19 (0.79) | 7 | 6 | generate:9, how_to:6, informational:5, decide:3 |  | 0.00 | 14 | 0.11 | 0.17 | 18 | study flashcards maker; create flashcards to study; study guide flashcard maker |
| 76 | how to stream 1080p obs | video_editing | 30 | 20 (0.67) | 20 | 6 | decide:17, informational:10, fix:2, how_to:1 |  |  |  |  |  |  | best obs settings for twitch 1080p 60fps; best obs settings for recording 1080p 60fps; best obs settings for streaming 1080p 60fps |
| 77 | marathon training plan for beginners | fitness_health | 47 | 46 (0.98) | 14 | 13 | plan:29, decide:8, calculate:4, generate:3 |  |  |  |  |  |  | marathon training plan for beginners; best marathon training plan for beginners; marathon training plan pdf |
| 78 | small business quickbooks cost | small_business | 76 | 43 (0.57) | 14 | 9 | informational:30, calculate:11, decide:9, alternative:9 |  |  |  |  |  |  | small business quickbooks cost; small business quickbooks alternative; best quickbooks software for small business |
| 79 | apartment hunting | real_estate_housing | 53 | 30 (0.57) | 13 | 13 | informational:23, plan:13, generate:5, decide:5 |  |  |  |  |  |  | apartment hunting checklist; apartment hunting checklist pdf; apartment hunting excel template |
| 80 | one month old feeding schedule | parenting_family | 22 | 22 (1.0) | 13 | 6 | plan:18, decide:3, calculate:1 |  |  |  |  |  |  | feeding schedule for 1 year old; feeding schedule for 1 month old; feeding schedule for 3 month old |
| 81 | termite repair | home_repair | 57 | 27 (0.47) | 12 | 10 | informational:30, calculate:12, how_to:9, decide:3 | 1.0 (±11.5) |  |  |  |  |  | how to repair termite damage; how to repair termite damage in walls; termite damage repair cost |
| 82 | video h264 vs h265 | video_editing | 48 | 44 (0.92) | 12 | 7 | convert:27, compare:14, informational:2, commercial_nav:2 |  |  |  |  |  |  | video h264 vs h265; video converter h265 to h264; h264 or h265 for youtube |
| 83 | final cut pro | video_editing | 132 | 60 (0.45) | 11 | 33 | informational:53, commercial_nav:19, how_to:17, track:9 | 21.0 (±12.1) |  |  |  |  |  | final cut pro tutorial; final cut pro tutorial pdf; how to update final cut pro |
| 84 | logic pro | audio_music_production | 117 | 60 (0.51) | 11 | 28 | informational:40, commercial_nav:17, how_to:16, decide:12 |  |  |  |  |  |  | logic pro cost; logic pro tutorial; is logic pro worth it |
| 85 | cpu cooler for 7800x3d | pc_building | 22 | 12 (0.55) | 11 | 5 | decide:12, informational:10 |  | 0.33 | 2 | 0.00 | 0.44 | 9 | best cpu cooler for ryzen 7 7800x3d; best cpu cooler for ryzen 7800x3d; best air cooler for ryzen 7 7800x3d |
| 86 | tire size | automotive | 60 | 58 (0.97) | 10 | 17 | calculate:50, fix:3, alternative:3, commercial_nav:2 |  |  |  |  |  |  | tire size guide; tire size meaning; tire size explained |
| 87 | 2fa security zerodha | cybersecurity | 67 | 57 (0.85) | 10 | 11 | how_to:57, informational:10 | ~0 (unreliable, ±100.0) |  |  |  |  |  | how to enable 2fa; how to enable 2fa easy; how to turn on 2fa |
| 88 | cycling training plan | fitness_health | 31 | 27 (0.87) | 10 | 6 | plan:16, decide:5, commercial_nav:3, calculate:3 |  |  |  |  |  |  | cycling training plan pdf free; cycling training plan for beginners; cycling training plan |
| 89 | fitness strava | fitness_health | 58 | 36 (0.62) | 10 | 2 | how_to:22, informational:20, decide:3, convert:3 | 1.3 (±5.4) |  |  |  |  |  | strava vs apple fitness; export mi fitness to strava; import mi fitness to strava |
| 90 | motorcycle won't turn over | motorcycles_powersports | 39 | 29 (0.74) | 10 | 2 | fix:29, informational:10 |  |  |  |  |  |  | dirt bike turns over but wont start; motorcycle won't turn over; car turns over but doesn't start |
| 91 | local full service movers | local_services | 51 | 27 (0.53) | 10 | 1 | informational:24, find:7, generate:7, calculate:4 |  |  |  |  |  |  | best same day movers service; full service movers near city; full service movers near state |
| 92 | backpacking gear | sports_outdoors | 57 | 34 (0.6) | 9 | 14 | informational:23, find:11, calculate:9, decide:8 | 2.4 (±4.8) |  |  |  |  |  | backpacking gear list; backpacking gear list pdf; best backpacking gear list |
| 93 | mod manager sims 4 | gaming_pc | 76 | 53 (0.7) | 9 | 12 | fix:30, informational:20, generate:4, how_to:4 |  |  |  |  |  |  | mod not working sims 4; best mod manager for sims 4; mod manager sims 4 broken cc |
| 94 | cut optimizer | diy_woodworking | 28 | 23 (0.82) | 9 | 10 | find:17, commercial_nav:3, decide:2, alternative:2 |  |  |  |  |  |  | cut list optimizer; cut list optimizer free; cut list optimizer free online |
| 95 | running plan for half marathon | fitness_health | 49 | 39 (0.8) | 9 | 7 | plan:19, informational:10, decide:5, generate:5 |  |  |  |  |  |  | running plan for half marathon; half marathon training plan for beginners; garmin half marathon training plan review |
| 96 | shoe size conversion chart | shopping_products | 35 | 34 (0.97) | 9 | 4 | calculate:34, informational:1 |  |  |  |  |  |  | shoe size conversion chart; shoe size conversion chart men; shoe size conversion chart kids |
| 97 | ev tesla model y | automotive | 33 | 22 (0.67) | 9 | 4 | find:11, informational:10, alternative:6, optimize:2 |  |  |  |  |  |  | ev database tesla model y; ev database tesla model 3 standard; ev database tesla model y standard |
| 98 | cpt code list | healthcare_professional | 71 | 36 (0.51) | 9 | 3 | informational:34, calculate:11, compare:6, how_to:6 | 2.2 (±5.2) |  |  |  |  |  | cpt code 93306 cost; cpt code 99203 cost; cpt code 99204 cost |
| 99 | home warranty termites | home_repair | 50 | 29 (0.58) | 9 | 3 | informational:21, how_to:15, calculate:8, decide:6 |  |  |  |  |  |  | how to treat termites at home; best treatment for termites in house; how to treat your house for termites |
| 100 | epic games launcher | gaming_pc | 84 | 46 (0.55) | 8 | 30 | informational:22, fix:20, commercial_nav:16, calculate:5 | 7.2 (±6.0) |  |  |  |  |  | epic games launcher error; epic games launcher update; epic games launcher not working |
