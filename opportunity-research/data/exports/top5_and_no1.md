# Top 5 finalists and the #1 recommendation (free route, evidence as of 2026-09-21)

Basis: 24 clusters carried through Phase 7 (competitor pages fetched, twelve questions each), drawn from the 94
clusters with web-search fragmentation evidence, drawn from 168 candidate problem clusters, drawn from 17,514
lexical clusters over 265,088 cleaned autocomplete queries (pass 1). Demand is relative only: no monthly volume,
traffic, revenue or user count was measured, and none is stated. Ordering rule for the finalists, in this order:
a reproducible structured dataset or computation that a chat assistant cannot substitute; a product-level long tail
of legitimate entry pages; measured fragmentation with no complete tool in the checked results; advertising fit;
maintenance cost. Where a likely incumbent could not be verified, that is stated as the primary risk, not ignored.

## Finalists

| # | cluster | frag | tool hits | pages fetched | why it advances | what would stop it |
|---|---|---|---|---|---|---|
| 1 | CPU x motherboard compatibility with minimum BIOS version | 0.56 | 1 | 6 of 8 | public per-model vendor lists, AI-resistant lookup layer, tens of thousands of pair pages, hardware purchase intent | PCPartPicker already surfacing BIOS minimums per pair (unverified, site blocked fetching) |
| 2 | modpack hardware requirements (RAM, CPU, launcher, install path) | 0.67 | 0 | 5 of 8 | specific need every fetched page answers generically; measurable dataset nobody publishes | measurement cost per pack update; community answers unread (three sources blocked) |
| 3 | per-game settings by platform, device and patch (two clusters) | 0.89 / 0.68 | 0 / 0 | 6 of 8 / 5 of 8 | highest fragmentation in the set, high repeat use, values readable from the games | prosettings.net, thespike.gg and trophi.ai may already be the database (unverified) |
| 4 | STL printability check and repair | 0.39 | 0 | 5 of 8 | computation on the user's file, no licensed data, high repeat use | online repair tools exist outside the checked results (not verified) |
| 5 | move-abroad visa eligibility by passport and occupation | 0.61 | 0 | 7 of 8 | every fetched page defers the real question; high-value adjacent advertisers | maintenance across jurisdictions and liability; incumbents unexplored |

Next two, not finalists: KDP cost and manuscript checker (Amazon may already publish a royalty calculator, not
verified); Warhammer 40k edition tracker (best AI resilience in the set, capped by rights risk and an official app).

## #1: a cross-brand CPU-to-motherboard compatibility database with minimum BIOS versions

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

## Demand check after placement (added after the Trends chain reached the finalists)

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

## Update after the founder's browser check of PCPartPicker (2026-09-21)

Check 1 was run: a part list with an MSI B450 TOMAHAWK MAX and a Ryzen 5 5600X shows the incumbent's own warning that
the board "supports the ... Processor with BIOS version Latest Beta BIOS. If the motherboard is using an older BIOS
version, updating the BIOS will be necessary to support the CPU." PCPartPicker therefore already holds a per-pair
BIOS-support field. The #1 is withdrawn as a standalone opportunity: the residual (a more precise version string, a
board-first table, per-pair landing pages) is a content layer beside a dominant incumbent, not a missing dataset.
Details and the revised conclusion are in pcpartpicker_conclusion.md.

Consequence for the ranking: no finalist now has a verified-open incumbent picture. Finalists 2 to 5 were ranked on
the same structure-of-data reasoning and each carries an unverified incumbent risk of the same kind (modpack platforms
and launcher wikis; prosettings.net, thespike.gg, trophi.ai; online STL repair tools; visa-service sites and
government eligibility checkers). The free route has delivered a well-evidenced shortlist, not a verified winner. The
next step is the same ten-minute browser check for each of the four, in the order listed, before any spend on
keyword data:

1. Modpack requirements: open a large modpack's page on CurseForge or Modrinth and note whether it states RAM and
   CPU requirements; search "all the mods 10 ram requirements" and record who answers.
2. Settings family: open prosettings.net's page for Marvel Rivals or Rocket League; note whether values are
   platform-branched, patch-dated and importable; search "best rocket league settings ps5" and record the top ten.
3. STL checker: search "repair stl file online free", open the first tool, upload a small STL and note whether it
   reports manifold, wall-thickness and unit problems before repair.
4. Visa eligibility: search "am i eligible to move to portugal from the us checker"; open expatsi.com and one
   government eligibility checker; note whether either returns routes for a stated nationality and occupation.

## A family seen across domains, not ranked

604 stored queries combine "mod" with compatibility, conflict, crash or "not working" across at least five games
("cities skylines incompatible mods spreadsheet", "how to check if mods are compatible", "minecraft server mods
compatibility", "pc gaming mods compatible with skyrim special edition"). Lexical clustering split them by game, so
no single cluster reached the candidate list and none was SERP-checked. Whether that family is fragmented and
unserved is unknown from this data; it is listed here because it recurs across domains, not because of any prior
interest in it.
