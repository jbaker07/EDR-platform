# Top 20 opportunities (free route; analysed on pass-1 clusters, mapped to pass-2)

Every candidate carries the brief's twenty fields. Measured facts come from the research store (autocomplete corroboration, Google Trends chain, web-search proxy results, fetched competitor pages); inference and hypothesis are labelled. No monthly search volume, traffic, revenue or user count is stated anywhere, because none was measured.


## 1. cpu motherboard compatibility  (`c_25613146`, domain pc_building)

**1. Problem.** A PC builder or upgrader holds a specific motherboard (or CPU) and needs to know which CPUs (or boards) it accepts,
and whether the pairing boots without a BIOS update. The socket-and-chipset explanation is everywhere; the per-model
answer with the minimum BIOS revision is only on each board maker's own support list, one vendor at a time, and the
BIOS step is the one the fetched pages say decides whether the machine boots at all.


**2. Representative searches (measured; most-corroborated task-intent members first).** `cpu motherboard compatibility list`; `best cpu compatible with my motherboard`; `cpu and motherboard compatibility calculator`; `best motherboard for cpu`; `cpu motherboard compatible list`; `how to get cpu out of motherboard`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 3 of 25 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 25 member queries, 12 with task intent (0.48 share), 3 of them volunteered by the engines on bare probes; intent mix informational 13, decide 3, find 2, alternative 2; subdomains touched: compatibility. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_19533304` with 25 members (27 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** Every fetched page supplies a different fragment and none completes the task. The method - check socket, then chipset, then BIOS - is stated redundantly by four editorial pages (cgdirector.com, xda-developers.com, pcguide.com, apexgamingpcs.com), with xda-developers.com adding TDP and the concrete socket-to-generation mapping. The authority they all defer to is the board maker's own CPU support list, represented here by asrock.com (which returned an empty body). The only CPU-first lookup is Intel's, and it lives on a different domain from the article that describes it, covers Intel only, and is dated 2023. The BIOS step - which pcguide.com and cgdirector.com both say determines whether the PC boots at all - has no lookup anywhere in the checked results: it is left as manual work on the vendor site. The one interactive board-first database that loaded, teamgroupinc.com, checks memory and SSDs rather than CPUs, and the aggregator two pages point to, pcpartpicker.com, blocked fetching.

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| pcpartpicker.com | tool | failed: HTTP | unknown | unknown | unknown | unknown |
| intel.com | vendor | ok | First-party vendor support content; no pricing, subscription, ads or affiliate l | page shows 'Content Type: Compatibility  | False | False |
| teamgroupinc.com | vendor | ok | Component vendor's own support tool promoting its memory and SSD lines; no prici | no last-updated date shown on the page | False | False |
| asrock.com | vendor | failed: empt | unknown | unknown | unknown | unknown |
| cgdirector.com | editorial | ok | Reader-supported editorial with affiliate commissions, disclosed in the footer | page shows 'Updated November 21, 2022' | False | False |
| xda-developers.com | editorial | ok | Ad-supported enthusiast publisher with an optional sign-in; no affiliate disclos | page shows 'Updated May 18, 2024, 6:30 P | False | False |
| pcguide.com | editorial | ok | Reader-supported editorial with disclosed affiliate commissions plus a newslette | page shows 'Last Updated on April 18, 20 | False | False |
| apexgamingpcs.com | editorial | ok | System builder's content marketing: no prices quoted, but it promotes its own cu | page shows 'Jan 19, 2024' | False | False |

**7. Why competitors do not completely solve it.** A board-first and CPU-first lookup over a single cross-brand table: enter a motherboard model (or a CPU) and get the compatible parts from both Intel and AMD, each row carrying the minimum BIOS version and its release date, a flag for pairings that need a BIOS flash before install, and the socket/chipset/TDP checks resolved automatically rather than explained. Per the fetched pages that is four things no single checked result offers together: cross-vendor coverage (Intel's tool is Intel-only), board-first direction (Intel's is CPU-first), BIOS-revision data (absent from every fetched page as data rather than advice), and currency (the fetched method articles date from Nov 2022, Jan 2024, Apr 2024 and May 2024, and Intel's from Apr 2023). Existing tools: Partially: the Intel Product Compatibility Tool at compatibleproducts.intel.com, named and linked by the fetched Intel support page ('use the Intel Product Compatibility Tool', with a motherboard vendor filter) - Intel processors only, and not itself fetched. PCPartPicker is named by a fetched page as an aggregator of 'noted compatibilities' but blocked both fetch attempts, so its coverage and BIOS handling are unverified. TEAMGROUP's Compatibility Check by Motherboard is a working board-first tool but for memory and SSDs, not CPUs. No cross-brand CPU-to-motherboard tool was confirmed in the checked results.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.56, tool hits 1, vendor share 0.17, unclassified 0.17, dominant site cgdirector.com (0.11); classes: editorial 7, unclassified 3, community 3, vendor 3, tool 1, reference 1. Queries checked: best cpu compatible with my motherboard; cpu motherboard compatibility list.

**9. Proposed free resource.** A cross-brand compatibility database: one page per motherboard model and one per CPU, each listing every supported
pairing with the minimum BIOS version, its release date, and a "flash before install" flag, plus board-first and
CPU-first lookups and a plain explanation of socket, chipset, TDP and BIOS for the pairing viewed. Built from the
vendors' published CPU support lists (the fetched pages describe a consistent "Brand Name, BIOS version, Date" format).


**10. Why users would choose it.** The pieced-together path today is four editorial pages saying "check the vendor list", then a vendor page per board,
then guessing about BIOS. One page that already merged those lists across Intel and AMD and answered the BIOS
question would end the search in one click. Inference: the value is the data table, not the advice.


**11. Indexable useful pages / tools.** One page per motherboard model (thousands of current and legacy boards) and per CPU (hundreds), plus pair pages for
every supported combination (tens of thousands), each a legitimate long-tail entry point ("does ryzen 5 5600x work
with b450 tomahawk", "b550 bios version for 5800x3d"). Hypothesis: page count is bounded by vendor list coverage.


**12. Repeat-use mechanism.** Low per user (a build or upgrade every few years) but recurring across the audience with every CPU generation and
BIOS release; a "BIOS changed" feed per board is the only obvious return mechanism. Inference.


**13. Advertising suitability.** High: hardware purchase intent, product-specific pages, standard display ads plus retailer affiliate links on every
pair page. Inference from intent; no CPC was measured.


**14. Other monetization.** Retail affiliate links, a compatibility API for shops and forums, sponsored vendor placements. Hypothesis.


**15. Maintenance requirements.** Continuous but automatable: re-crawl vendor support lists on a schedule and diff BIOS entries; new CPU launches need
new rows within days. Measured obstacle: two vendor pages blocked or emptied automated fetches in this task.
 Data source and reproducibility (from the competitor record): inference: the source data is public and structured - every board maker publishes a per-model CPU support list, and three fetched pages instruct readers to use exactly those lists, with pcguide.com stating the rows follow a 'Brand Name, BIOS version, Date' format. That consistent shape, plus Intel's and TEAMGROUP's existing structured lookups, suggests an aggregated database is reproducible from public vendor pages. Two practical obstacles were observed in this task: asrock.com returned an empty body twice and pcpartpicker.com returned 403 twice, so ingestion would have to handle blocking and JavaScript-rendered tables. Specification data (sockets, chipsets, TDP) is stable; BIOS-version data changes with each vendor release and would need continuous refresh, which is also where the fetched articles are most out of date.

**16. AI threat.** The explanatory layer is fully replaceable by a chat assistant. The lookup layer is not, because the correct
answer is per-model, changes with firmware releases and a stale answer stops a PC booting; assistants with live
retrieval would cite the database rather than replace it. Inference.
 Competitor-record view: inference: split sharply. The conceptual layer - socket, chipset, BIOS and TDP, and what happens if you get it wrong - is fully AI-replaceable and is what four of the six fetched pages sell; a chat assistant answers it in one turn without a click. The lookup layer is highly AI-resistant: which specific CPUs a named board accepts, and at which minimum BIOS revision, is per-model data that changes with firmware releases, and an assistant answering from memory would be wrong in the exact way that, per the fetched pages, stops a PC from booting. This cluster's resilient value therefore sits entirely in the data, not the explanation.

**17. Search-engine direct-answer threat.** Socket-level questions ("does am4 fit b550") will be answered inline by search engines; the BIOS-version and
pair-level questions require a table and are less likely to be absorbed. Inference.


**18. Primary risk.** PCPartPicker already runs a compatibility checker inside its build tool and is the best-known destination; it
blocked both fetch attempts, so whether it already surfaces minimum BIOS versions per pair is unverified. If it
does, the gap shrinks to presentation and long-tail pages. Second risk: vendor pages resisting automated collection.


**19. Evidence quality / confidence.** Fragmentation and competitor evidence: moderate (18 proxy results, 6 of 8 pages fetched, two blocked). Demand
evidence: weak on the free route (25-member pass-1 cluster plus a 26-member sibling, no Trends placement); the
product-specific long tail is inferred from the shape of the vendor lists, not measured.
 Open questions from the competitor record: pcpartpicker.com blocked both fetch attempts, so whether its compatibility filter already covers CPU-to-board pairings with BIOS caveats - the core of the opportunity - is unverified and is the single biggest gap in this analysis | asrock.com returned an empty body, so the structure of a board maker's CPU support list (fields, BIOS columns, whether it is scrapeable) was not directly observed, only described second-hand by pcguide.com | no AMD-side equivalent of Intel's compatibility tool appeared in the checked results; whether AMD publishes one is unknown | the Intel Product Compatibility Tool itself was not fetched (only the support article pointing to it), so its coverage, currency and whether it surfaces BIOS requirements are unknown

**20. Verdict.** Finalist and provisional #1. Strongest combination in the reviewed set of a reproducible structured dataset, an
AI-resistant lookup layer, high commercial intent and a large product-level long tail. Two checks are required
before committing: verify PCPartPicker's BIOS handling in a browser, and buy one month of keyword data for the
board- and CPU-level queries to size the demand.



## 2. best pc for modding  (`c_06057594`, domain gaming_pc)

**1. Problem.** Players of modded games (modded Minecraft dominates the members) need to know what hardware a specific modpack
needs, how much RAM to allocate, and how to install and run the pack on their launcher. Every fetched hardware page
is written by a seller of hardware and gives a generic floor; both how-to pages state that no universal install
instructions exist; no page is specific to the pack the user actually has.


**2. Representative searches (measured; most-corroborated task-intent members first).** `how to play modded games on pc`; `pc game modding tutorial`; `best pc for modding and gaming`; `how to get modded games on pc`; `best gaming pc for modded minecraft`; `best pc for modding`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±101.3% error, so unreliable; 3 of 22 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 22 member queries, 11 with task intent (0.5 share), 3 of them volunteered by the engines on bare probes; intent mix informational 10, how_to 7, decide 4, commercial_nav 1; subdomains touched: game_mods. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_55353237` with 23 members (24 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** Like the other two clusters, this one contains two jobs that no page serves together - 'what hardware do I need for modded games' and 'how do I actually install and run mods' - and the searcher must visit different sites for each. On hardware: the core engineering fact, that modded Minecraft is CPU-bound, is stated most clearly on siriuspowerpc.com; the link between mod packs, RAM and the launcher ecosystem (CurseForge, Fabric, Technic) is on apexgamingpcs.com; a ten-machine price-tiered shortlist is on grandgoldman.com. All three sell or earn from the hardware they recommend, so an independent read is unavailable anywhere in the fetched set, and the two community threads that would have provided one (community.microcenter.com, quora.com) returned 403 while the Facebook post served only the question, not the answers. On the how-to side: the conceptual lifecycle and current destinations are on howtogeek.com (June 2024) and an older framework on pcworld.com (November 2017), and both explicitly refuse to give game-specific steps - "I can't give you universal instructions on how to download and install mods". The RAM numbers also conflict across pages (at least 4GB dedicated to a modpack versus at least 16GB system RAM) with no page reconciling allocated heap against system memory.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| howtogeek.com | editorial | ok | Affiliate commerce plus advertising - "When you make a purchase using links on o | "Published Jun 21, 2024, 1:00 PM EDT" -  | yes - downstream; the article recommends Nexus Mods' Vortex Mod Manager as its example of a mod manager. | yes - downstream; the article instructs the reader to register an account on a modding site. |
| community.microcenter.com | community | failed: HTTP | unknown - inference: the forum is hosted by a PC retailer, so recommendations th | unknown | unknown | unknown - inference: forums typically require an account to post, though not to read. |
| facebook.com | community | failed: logi | unknown | unknown | unknown | unknown - inference: the replies were not served to an unauthenticated fetch, which is consistent with a login requirement. |
| pcworld.com | editorial | ok | Affiliate commerce - "When you purchase through links in our articles, we may ea | "Nov 17, 2017" - nearly nine years old a | yes - downstream; the guide recommends installing several mod managers and helper tools. | yes - downstream; the guide directs readers to create accounts on mod hosting sites. |
| apexgamingpcs.com | editorial | ok | Direct hardware sales - the page links four of the company's own prebuilt system | "Jun 09, 2023" with "Edited: 1/2/2024" | no | no |
| quora.com | community | failed: HTTP | unknown | unknown | unknown | unknown |
| siriuspowerpc.com | editorial | ok | Direct hardware sales - five systems listed at "$1,799.99," "$2,199.99," "$2,139 | unknown - no published or last-updated d | no | no |
| grandgoldman.com | editorial | ok | Affiliate commerce plus display advertising - "This page may contain affiliate l | unknown - no published or last-updated d | no | no - though completing a purchase requires an Amazon account. |

**7. Why competitors do not completely solve it.** A single destination would have to take the user's actual modpack - by name or by manifest - and return both halves of the answer: the RAM to allocate and the CPU single-thread target for that specific pack, rather than a generic floor; a hardware shortlist priced today and not tied to one vendor's catalogue; and then the install path for that pack on that launcher (CurseForge, Fabric, Technic, Vortex, Steam Workshop) with version compatibility checked. The recurring failure across every fetched page is that the answer is generic while the user's situation is specific - two pages say so in their own words. It would also need a date on it: two of the five fetched pages show no date at all while claiming to be current-year guides. Existing tools: none found in the checked results. The input file records "tool_hits": 0 for this cluster and categorises no result as tool, vendor or official. Tools are named on the fetched pages but none of them is the search result: mod managers and hosts (Nexus Mods' Vortex Mod Manager, Nexus Mod Manager, ModDB, Steam Workshop, LOOT, Wrye Bash, the Twitch Desktop App) and modpack platforms (CurseForge, Fabric, Technic). One vendor mentions its own "Fully Custom Configurator" on its site, but the ranking page is a static article, not the configurator. Unselected results outside the rule: en.wikipedia.org (two reference results), itch.io (marketplace), windowscentral.com, pcgamer.com and builttofrag.com (unclassified).

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.67, tool hits 0, vendor share 0.0, unclassified 0.17, dominant site facebook.com (0.11); classes: community 6, editorial 6, unclassified 3, reference 2, marketplace 1. Queries checked: best gaming pc for modded minecraft; how to play modded games on pc.

**9. Proposed free resource.** A per-modpack requirements database: for each named pack and version, the measured RAM allocation and CPU
single-thread target, tested frame times on a few reference machines, the launcher it runs on, known conflicts,
and a hardware shortlist priced from live retail data; plus the install path for that pack on that launcher.


**10. Why users would choose it.** Today the answer is generic while the user's situation is specific, and two fetched pages say so in their own
words. A page for "this pack, this version" replaces a Reddit thread, a vendor blog and a launcher wiki. Inference.


**11. Indexable useful pages / tools.** One page per modpack version (thousands on the major modpack platforms), per launcher guide, and per
hardware-tier recommendation; pack-name queries are a natural long tail. Hypothesis on counts.


**12. Repeat-use mechanism.** Moderate: players change packs and packs update; a "does my machine run pack X" check is repeated. Inference.


**13. Advertising suitability.** High for the hardware side (GPU, RAM, prebuilt intent, affiliate-friendly); the install side is a gaming
audience with lower CPC. Inference; no CPC measured.


**14. Other monetization.** Hardware affiliate links, server-hosting affiliates for multiplayer packs, sponsored prebuilt listings. Hypothesis.


**15. Maintenance requirements.** High: the measured layer requires running packs on reference hardware each time a pack updates; the install
guides decay with launcher changes (one fetched page still recommends a 2017 launcher). Measurement cost is the
central cost of this product.
 Data source and reproducibility (from the competitor record): inference: almost entirely reproducible. Component roles, modpack RAM requirements and mod-manager instructions are publicly documented; retail hardware listings are available to any Amazon affiliate, which is why grandgoldman.com-style lists are cheap and numerous. Two inputs are not freely reproducible and both are behind failed fetches or logins: the accumulated community threads on community.microcenter.com, quora.com and the Facebook group, which can only be re-accumulated over time rather than rebuilt. A per-modpack performance dataset - measured frame times and RAM headroom for named packs on named hardware - does not appear on any fetched page and would have to be generated by testing, making it the one genuinely defensible asset available in this cluster.

**16. AI threat.** Explanation and generic install steps are fully replaceable, and an assistant can be asked about one game and
one mod; what an assistant cannot do is measure a pack's real requirements or quote current prices. Inference.
 Competitor-record view: inference: low for the content, with two narrow exceptions. Both how-to pages state that no universal instructions exist because steps vary per game and per mod, which is precisely the constraint a chat assistant escapes by being asked about one game and one mod - an assistant strictly outperforms the pcworld.com guide, which still recommends the Twitch Desktop App from 2017. Hardware spec reasoning ('modded Minecraft is CPU-bound, allocate RAM to the pack') is a one-paragraph assistant answer. The two things an assistant cannot do are quote today's prices and stock, and run the pack to measure what it actually needs. Durable value would come from measurement and live pricing, not from explanation.

**17. Search-engine direct-answer threat.** "How much RAM for modded Minecraft" will be answered inline; per-pack measured pages are less absorbable. Inference.


**18. Primary risk.** The defensible asset is measurement, which must be produced continuously; without it the site is another
affiliate listicle. Community threads that hold the lived answers (Micro Center, Quora, Facebook) could not be
read, so the strength of the community answer is unknown.


**19. Evidence quality / confidence.** Moderate on fragmentation (18 proxy results, 5 of 8 pages fetched); weak on demand (22-member pass-1 cluster,
Trends placement unreliable). The cluster also mixes a piracy-adjacent strand ("download modded pc games") that
would have to be excluded from any ad-supported site.
 Open questions from the competitor record: What do the community threads actually recommend? Three of the eight selected competitors are community sources (community.microcenter.com, quora.com, facebook.com) and all three failed to serve their answers - 403, 403, and a login-walled preview - so the non-vendor view of this question is entirely unread. | How should the conflicting RAM guidance be reconciled - at least 4GB dedicated to most mod packs (apexgamingpcs.com) against at least 16GB DDR4/DDR5 system RAM (grandgoldman.com)? No fetched page distinguishes allocated Java heap from system memory. | The cluster's members split between modded Minecraft hardware, general mod installation, and a third strand about obtaining modded or pirated game copies ('download modded pc games', 'how to get modded pokemon games on pc', 'how to get modded mobile games on pc'). No fetched page addresses that third strand, and it may carry legal and policy constraints that make it unsuitable as an opportunity. | Is windowscentral.com (rank 1 for 'best gaming pc for modded minecraft', categorised unclassified) an editorially independent review or another affiliate shortlist? It was outside the selection rule and was not fetched.

**20. Verdict.** Finalist. Clear unmet specific need with a measurable, defensible dataset; ranked below the compatibility
database because its maintenance is manual measurement rather than automated collection, and its demand
evidence is thinner.



## 3. best settings for rivals  (`c_28504254`, domain gaming_pc)

**1. Problem.** Players want the best in-game settings for a specific game on their device and platform (PC, console, mobile),
current for the latest patch, with a sensitivity that fits their own mouse or stick. The head term itself covers
two games (Marvel Rivals and Roblox RIVALS) and no fetched page disambiguates; every fetched page is a static
value list that ends at "type these in by hand" and admits there is no universal sensitivity.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best settings for rivals`; `best settings for rivals roblox mobile`; `best settings for marvel rivals`; `best settings for roblox rivals`; `what's the best settings in rivals`; `rivals best settings ps5`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'best settings for rivals' at 0.6 on the chained Trends scale (root = 100, rounding error ±36.4%); 8 of 35 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 35 member queries, 35 with task intent (1.0 share), 4 of them volunteered by the engines on bare probes; intent mix decide 34, how_to 1; subdomains touched: settings. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_21608252` with 44 members (37 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** No fetched page answers the cluster as posed, partly because the cluster is two different games under one name. Disambiguation is on nobody - the searcher must work out whether they mean Marvel Rivals or Roblox RIVALS before any page helps. Marvel Rivals PC graphics with measured performance data is on pcgamesn.com; the performance-first options rationale is on prosettings.net; Windows and GPU-level FPS work plus controller settings is on scufgaming.com. For Roblox RIVALS, a copyable value list is on robloxden.com, multi-device coverage (desktop, phone, tablet, Xbox, PS5) is on pixeltwelve.com, and the only per-player calibration method is on rivals.wiki. Sensitivity is the gap every page admits to - robloxden says it is "purely based on your comfort, desk setup, and mouse", rivals.wiki says "There is no universal best sensitivity". Nothing fetched offers a settings code, per-hero settings, or a way to verify that the applied settings worked.

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| robloxden.com | editorial | ok | unknown - the fetched page showed no pricing, subscription or affiliate disclosu | "Last checked for codes: Yesterday" is s | no | no |
| pcgamesn.com | editorial | ok | Affiliate commerce plus advertising - the page discloses "As an Amazon Associate | "Updated: June 25, 2025" | no | no |
| tiktok.com | community | failed: empt | unknown | unknown | unknown | unknown |
| prosettings.net | editorial | ok | Advertising plus affiliate commerce - the page mentions "targeted ads" and the f | "Last modified on Apr 11, 2025" | no | no - reading is open, though the fetch noted the comment section requires login to participate. |
| scufgaming.com | editorial | ok | Content marketing for the site's own hardware - the page states "For controller  | "Last updated: April 28, 2026" - the mos | yes - for part of the workflow; the fetch noted that implementing some recommendations, such as NVIDIA Profile Inspector, would require downloads. | no |
| rivals.wiki | editorial | ok | unknown - the fetch found no pricing, subscription or affiliate links in the vis | "Updated July 27, 2026" | no | no |
| pixeltwelve.com | editorial | ok | Advertising - the fetch reported multiple sections labelled "Advertisement"; no  | "Jul 31, 2026Updated Sep 16, 2026" - pub | no - reading is open; applying the settings requires owning Roblox RIVALS. | no |
| ggwtb.com | editorial | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** First, a game disambiguation step (Marvel Rivals vs Roblox RIVALS) before any values are shown. Then: device- and platform-branched settings (PC, PS5, Xbox, iPad/mobile, controller) rather than a PC-only table; a version stamp tied to the current patch or update number, since members ask for 'update 17' and 'new update'; a sensitivity output derived from the player's own input (mouse DPI, monitor, stick or touch) instead of a fixed number; an exportable or pasteable config for the games that support an import box, which pixeltwelve.com states its values are not; and an after-the-fact check so a player can tell whether the change worked. Every fetched page stops at 'now go type these in by hand'. Existing tools: none found in the checked results. The input file records "tool_hits": 0 for this cluster and categorises zero results as tool, vendor or official. All six pages that fetched successfully are static articles; even prosettings.net, whose brand implies a pro-settings database, served a static guide with no database or calculator on this URL. Two candidates were outside the selection rule and were not fetched: rivalwiki.com/settings (rank 4, categorised reference) and esportsinsider.com (rank 6, unclassified).

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.89, tool hits 0, vendor share 0.0, unclassified 0.06, dominant site scufgaming.com (0.17); classes: editorial 13, community 3, reference 1, unclassified 1. Queries checked: how to good settings in rivals; what's the best settings in rivals.

**9. Proposed free resource.** A per-game, per-platform settings database with a version stamp per patch, device-branched values, a sensitivity
converter driven by the player's DPI and monitor, pasteable or importable configs where the game supports them,
and sourced pro configurations with dates; the same schema across many games.


**10. Why users would choose it.** One dated, platform-specific, importable answer instead of three articles per game, each from a different year and
one of them funded by a boosting shop. Inference.


**11. Indexable useful pages / tools.** One page per game x platform x patch, plus per-pro configurations and per-device sensitivity pages: hundreds of
games, thousands of pages, with a new page each patch. Hypothesis on counts.


**12. Repeat-use mechanism.** High for active players: every patch, every new game, every hardware change. Inference.


**13. Advertising suitability.** High volume, low CPC gaming audience; peripheral (mouse, controller, monitor) affiliate fit is good. Inference.


**14. Other monetization.** Peripheral affiliates, coaching or boosting referrals (reputational risk), pro-team sponsorships. Hypothesis.


**15. Maintenance requirements.** Per-patch editorial upkeep across many games; values are readable from the games' own menus, so collection is
cheap but never finished. Measured: fetched pages carry April and September 2026 dates precisely because values drift.
 Data source and reproducibility (from the competitor record): inference: the underlying data is fully reproducible from public sources - every recommended value is readable from the games' own settings menus by anyone who owns them, and the games are free to play. The two inputs that are not free to copy are first-party FPS benchmarks on named hardware (pcgamesn.com is the only fetched page showing measured performance data) and owning the five device types pixeltwelve.com claims to cover. There is no licensed or proprietary dataset anywhere in this cluster, so the barrier to entry is editorial upkeep per patch, not data access.

**16. AI threat.** A value list with rationale is close to the ideal chat answer; resistance comes only from patch currency,
measured benchmarks and importable artifacts. Inference: low to moderate resilience.
 Competitor-record view: inference: low to moderate. The dominant artifact is a list of menu values with a short rationale, which is close to the ideal shape for a chat answer, and several pages concede there is no universal correct value anyway. Three things resist an assistant: currency against the live patch (scufgaming.com carries April 2026 and pixeltwelve.com September 2026 dates precisely because the values drift), measured hardware benchmarks, and the risk of fabricating a settings code that does not exist - pixeltwelve.com had to explicitly correct that expectation. A tool that read the player's device and current game version, rather than recited values, would be materially harder to replace than any of these pages.

**17. Search-engine direct-answer threat.** High for "best sensitivity for X" style questions; lower for per-device tables and import codes. Inference.


**18. Primary risk.** Established settings databases exist in the results but were not verified here: prosettings.net served a static
guide at the fetched URL, and thespike.gg and trophi.ai ranked first and second for the Rocket League sibling
cluster but were outside the selection rule. If one of them already does the per-platform, per-patch job, the
opportunity is a feature gap, not a site.


**19. Evidence quality / confidence.** Fragmentation evidence strong for the checked queries (0.89 fragmented share, no tool hits); competitor evidence
incomplete (two plausible databases unverified); demand evidence weak (35-member cluster, head term at 0.6 on the
Trends chain, which is low).
 Open questions from the competitor record: Does Roblox RIVALS actually accept a shareable settings code string? pixeltwelve.com refers to "that box" for importing settings while saying its own values are not pasteable into it, and the cluster contains the member query 'rivals best settings code' - unresolved from fetched pages. | Is rivalwiki.com/settings (rank 4, categorised reference in the input file) an interactive settings database or another static page? Not selected under the tool/vendor/official-then-editorial/community rule and therefore not fetched. | What share of this cluster's 35 members means Marvel Rivals versus Roblox RIVALS? The sample members contain both explicitly, and the split determines whether this is one opportunity or two. | Whether the TikTok topic feed and ggwtb.com contain tool-like functionality is unknown - both fetches failed (empty feed, HTTP 403).

**20. Verdict.** Finalist by fragmentation and repeat use, held back by unverified incumbents and low-CPC monetization. Verify
prosettings.net and thespike.gg in a browser before advancing further.



## 4. stl files for 3d printing  (`c_38358966`, domain three_d_printing)

**1. Problem.** Someone with a 3D printer needs a printable file: find or design a model, export it correctly from their CAD
program, and make sure the mesh actually prints. Format explainers and a dated software table sit on reference
and vendor pages; the export rules that decide printability are on one page and cover three CAD programs;
marketplaces hand over files without saying anything about printing them; nothing fetched checks or repairs a mesh.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best stl files for 3d printing`; `how to design stl files for 3d printing`; `where to find stl files for 3d printing`; `best stl files for 3d printing free`; `how to repair stl files for 3d printing`; `stl files changes 3d printing`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 8 of 30 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 30 member queries, 18 with task intent (0.6 share), 4 of them volunteered by the engines on bare probes; intent mix informational 10, how_to 5, decide 2, calculate 2; subdomains touched: models. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_74735723` with 45 members (32 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The cluster's two checked queries are really one workflow broken across four page types. Format definition sits on en.wikipedia.org (STL, 3MF, AMF, OBJ, STEP) and adobe.com. Tool selection sits on guides.atsu.edu, a dated library table of modelling software by user level and platform, which then defers outward to Wikipedia. The export settings that decide whether a file actually prints sit on protolabs.com - tessellation and tolerance, matching units, and 'the STL file needs to have a single, solid body' - and only for CREO, Solidworks and AutoCAD. Ready-made files sit on cults3d.com with '3.7M designs'. Nothing fetched covers repair of a broken mesh, slicing, or printer settings, although the member queries ask for all of them ('how to repair stl files for 3d printing', 'stl files converter 3d printing', 'stl files checklist 3d printing').

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| adobe.com | vendor | ok | Vendor funnel for Adobe Creative Cloud subscription plans, promoted throughout t | unknown - no published or updated date s | yes - the stated workflow requires Adobe Photoshop on desktop | unknown (not stated for reading; an Adobe subscription is required for the described workflow) |
| guides.atsu.edu | official | ok | University library guide; no pricing, subscription, ads or affiliate links state | "Last Updated: Sep 10, 2026 2:43 PM" | varies - the table lists browser-based options alongside Windows, Mac and Linux desktop software | unknown - not stated |
| en.wikipedia.org | reference | ok | Non-commercial encyclopedia; no pricing, subscription, ads or affiliate links st | "This page was last edited on 11 Septemb | no | no |
| cults3d.com | editorial | ok | Marketplace with advertising - the page carries 'Advertising' labels and heading | Monthly collections with the most recent | no | unknown - a Sign in option exists but the page does not state whether downloading requires it |
| protolabs.com | unclassified | ok | Lead generation for the company's own manufacturing service - 'Have a design rea | unknown - no published or updated date s | yes - the guidance presumes the reader already has a CAD program | unknown - not stated for reading; uploading for a quote is offered |
| myminifactory.com | unclassified | failed: HTTP | unknown | unknown | unknown | unknown |
| autodesk.com | vendor | failed: HTTP | unknown | unknown | unknown | unknown |
| markforged.com | editorial | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** A file-to-print path in one place - upload or pick a model, get an automatic printability check (manifold geometry, wall thickness, units, single body, overhangs), an automated repair, and per-CAD-program export instructions for whatever tool the user actually has, ending in slicer-ready output. The evidence gap is sharp: protolabs.com names the failure conditions but offers no way to test a file against them, and cults3d.com hands over files without saying anything about printing them. Existing tools: None found in the checked results for the design and repair half - the input file records tool_hits 0. For the finding half, two marketplaces appeared: Cults3D (fetched, '3.7M designs') and MyMiniFactory and Gambody (SERP entries only, not fetched or fetched unsuccessfully). Autodesk and Adobe appear as software vendors, not as STL validation or repair tools.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.39, tool hits 0, vendor share 0.28, unclassified 0.22, dominant site adobe.com (0.22); classes: vendor 5, unclassified 4, editorial 4, community 3, reference 1, official 1. Queries checked: best stl files for 3d printing; how to design stl files for 3d printing.

**9. Proposed free resource.** A file-to-print utility: upload a model, get an automatic printability report (manifold geometry, wall thickness,
units, single body, overhangs), an automated repair, per-CAD-program export instructions for the tool the user
has, and slicer-ready output; alongside a curated finder across the marketplaces.


**10. Why users would choose it.** It operates on the user's own file, which no fetched page does, and answers "will this print" before a failed
print costs hours of filament. Inference.


**11. Indexable useful pages / tools.** Per-CAD-program export guides, per-error repair pages (non-manifold, inverted normals, thin walls), per-printer
settings pages and model-category finder pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** High: every new model and every new print. Inference.


**13. Advertising suitability.** Good: maker audience with continuous consumable and hardware purchases (filament, printers, upgrades). Inference.


**14. Other monetization.** Filament and printer affiliates, marketplace referrals, premium batch repair. Hypothesis.


**15. Maintenance requirements.** Low for the geometry engine (computational, no licensed data); moderate for per-CAD export guides. Measured: the
explainer layer rests on open specifications.
 Data source and reproducibility (from the competitor record): inference: sharply split. The explainer layer is fully reproducible - the STL specification is open, format comparisons are on Wikipedia under a free licence, and export settings are in each CAD vendor's public documentation, so adobe.com, protolabs.com, guides.atsu.edu and the Wikipedia page rest on nothing proprietary. The marketplace layer is not reproducible: cults3d.com's asset is millions of user-uploaded models plus the community signals it ranks them by, which is a two-sided network, not a dataset. A printability-checking tool would need no licensed data at all - only computational geometry over the user's own file.

**16. AI threat.** Explainers are fully exposed; running geometry checks and repairs on a binary file is not something a chat
window does. Inference, and the strongest resilience argument in the set after the settings-data clusters.
 Competitor-record view: inference: the explainer half is highly exposed and the artefact half is not. 'What is an STL file', 'which software should I use' and 'how do I export' are stable questions a chat assistant answers well, which puts adobe.com, the Wikipedia page and much of protolabs.com at risk. What a chat assistant cannot do is hand over a mesh file, run geometry checks on a user's binary STL, or repair it - so the marketplace and any validation or repair utility sit on the resilient side. The defensible position in this cluster is the operation on the file, not the writing about the file.

**17. Search-engine direct-answer threat.** High for "what is an stl file"; nil for the file operation. Inference.


**18. Primary risk.** Online STL repair and validation tools exist outside the checked results (not searched for or verified here);
three of eight selected pages blocked fetching, so the tool landscape is under-evidenced. The finder half faces
marketplaces with millions of models and network effects.


**19. Evidence quality / confidence.** Fragmentation 0.39 measured with no tool hits; 5 of 8 pages fetched; demand weak (30-member cluster, no Trends
placement). Competitor coverage for repair tools is incomplete.
 Open questions from the competitor record: MyMiniFactory, Autodesk and Markforged all returned 403 twice, so three of the eight selected competitors - including the cluster's second marketplace - are evidenced only by SERP title. | adobe.com holds 4 of 18 checked results with the same explainer content appearing under multiple URLs; whether that dominance is editorial strength or URL duplication is unknown from the pages fetched. | Whether free tools already perform automated STL repair and printability checking is unknown - no such tool appeared in the checked results, but the checked queries were not repair queries. | Cults3D's free-versus-paid split and licensing terms are not explained on the fetched page, so the marketplace's actual economics are unknown.

**20. Verdict.** Finalist by resilience and repeat use: a computational utility with no licensed data and a natural long tail of
error and export pages. Verify existing online repair tools before advancing.



## 5. how to move abroad  (`c_63944196`, domain travel)

**1. Problem.** A person wants to know whether and how they can move to another country: which visa routes they qualify for given
nationality, occupation and income, what it costs, and which jobs would take them. Every fetched page defers the
actual question ("check with a local embassy") and monetises adjacent products; four of seven are generic job lists.


**2. Representative searches (measured; most-corroborated task-intent members first).** `how to move abroad`; `how to move abroad from uk`; `how to move abroad with no money`; `how to move abroad as an american`; `best jobs to move abroad`; `how to move abroad from usa`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'how to move abroad' at 0.7 on the chained Trends scale (root = 100, rounding error ±37.9%); 5 of 56 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 56 member queries, 39 with task intent (0.7 share), 8 of them volunteered by the engines on bare probes; intent mix how_to 20, informational 17, calculate 10, decide 7; subdomains touched: expat_moving. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_71768535` with 10 members (11 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The task splits into four parts and each fetched page holds at most one. Which job could take me abroad is answered by William Russell (ten sectors, named countries, salary figures), Preply (eleven jobs with salary ranges by country), AFAR (eight travel careers with the exact credentials each demands) and A Way Abroad (eleven jobs with required certifications, from lived experience). What are the steps is answered only by GoAbroad, which covers visas, housing, insurance, banking and money in ten generic steps and then tells the reader to ask an embassy. Should I go at all is answered by One Journey Away, on emotional readiness, and by the archived Yale essay from 2006 on career framing. Am I eligible, what will it cost, and what does my target country actually require is answered by none of the fetched pages - every one of them defers it to the reader (do your research, check with a local embassy, do your due diligence on understanding visas, you have the right visa and work permit). The cluster's own member queries ask for costs and a calculator; nothing fetched supplies either.

**6. Major competitors (8 reviewed, 7 pages fetched, 1 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| archive-yaleglobal.yale.edu | official | ok | inference-free reading of the page: none evident - no pricing, subscription, ads | Severely stale - the page is dated March | no | no |
| william-russell.com | editorial | ok | Content marketing for international health insurance - the fetch found repeated  | Page shows a May 2025 byline date. | no | no to read the article; the insurance quote tool is separate. |
| goabroad.com | editorial | ok | Affiliate and partner placements plus traffic to the publisher's own jobs direct | Page shows a spring 2026 last-updated da | no | no to read; the linked job search sits on a separate platform. |
| findawayabroad.com | editorial | ok | Affiliate commissions plus the author's own products - the page carries an expli | Page shows an April 2024 publication dat | no | no to read; the workbook and mentorship require contact details or payment. |
| preply.com | editorial | ok | Content marketing for the publisher's paid one-to-one tutoring marketplace - the | Page shows a February 2025 update date. | no for the article; an app download is promoted. | no to read; using the tutoring service requires signing up. |
| afar.com | editorial | ok | Magazine publishing - the fetch found an affiliate-marketing disclosure, paid ad | Page states it was originally published  | no | no to read the article; a subscription is sold separately. |
| onejourneyaway.com | editorial | ok | Personal travel blog with a free downloadable city guide in exchange for interes | unknown - the page shows a day and month | no | no |
| gooverseas.com | editorial | failed: HTTP | unknown | unknown - the title carries a 2026 refer | unknown | unknown |

**7. Why competitors do not completely solve it.** A destination would have to turn the deferral into the product: take nationality, occupation, qualifications, income and household (including pets, which appear in the member queries), then return the visa routes the person actually qualifies for, with current requirements, timelines and fees; the money answer - upfront cost of the move plus cost of living and tax exposure in each candidate country; job routes that match the qualifications rather than a generic top-ten list; and a document checklist that tracks progress. The four fetched job lists would collapse into one filtered result set, and the step-by-step guide would become a per-country plan instead of a caveat. Existing tools: none found in the checked results - the input file records tool_hits 0 and vendor_share 0.0 across 18 checked results, and all seven fetched pages are articles. One caveat: expatsi.com ranks sixth for the head term with the title Support for Americans Moving Abroad, but its category in the input file is unclassified so it was not selected under the selection rule and was not fetched; what it offers, whether it is a tool, and what it charges are all unknown. Two money-transfer guides (wise.com, 2 of 18 results) were also below the selection cut and are unassessed.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.61, tool hits 0, vendor share 0.0, unclassified 0.33, dominant site findawayabroad.com (0.11); classes: editorial 11, unclassified 6, official 1. Queries checked: best jobs to move abroad; how to move abroad.

**9. Proposed free resource.** An eligibility engine: enter passport, occupation, qualifications, income and household, get the visa routes
you qualify for in each candidate country with current requirements, fees, timelines and a document checklist,
plus cost-of-move and cost-of-living comparisons, each fact cited to the issuing authority.


**10. Why users would choose it.** It answers the question every fetched page refuses, with citations to primary sources, instead of a ten-step
generic guide plus a job listicle. Inference.


**11. Indexable useful pages / tools.** One page per origin x destination x route (thousands), per occupation-to-country pathway, and per country cost
profile. Hypothesis on counts.


**12. Repeat-use mechanism.** Moderate: a long decision cycle with many return visits, then a progress checklist. Inference.


**13. Advertising suitability.** High-value adjacent categories (insurance, money transfer, relocation, language tutoring) appear as the
monetisation of every fetched page, which indicates advertiser appetite; no CPC measured.


**14. Other monetization.** Insurance and transfer affiliates, relocation-service leads, premium checklist. Hypothesis.


**15. Maintenance requirements.** Very high: dozens of jurisdictions in legal language, changing frequently; errors have real consequences. This is
a sustained data operation, not a scrape. Measured: the fetched pages carry no sourcing for their salary figures.
 Data source and reproducibility (from the competitor record): inference: the data is public but heavy. Visa routes, eligibility criteria, fees and processing times are published by governments; cost-of-living and salary figures come from public statistical and job-board sources; tax treatment is published by revenue authorities. Nothing is proprietary, which is why none of these pages has a moat - but the same facts are spread across dozens of jurisdictions, are written in legal language, and change often, so the build is a sustained data-maintenance operation rather than a scrape. Note also that the salary figures stated on William Russell and Preply carry no source on the page, so their provenance is unknown.

**16. AI threat.** Generic steps and job lists are fully replaceable; verified current eligibility with citations is not. Inference.
 Competitor-record view: inference: split. Everything the fetched pages actually contain - generic steps, job lists, decision framing - is fully replaceable by a chat assistant, which can additionally filter by the reader's background, something no static list does. What resists is the part none of the pages attempt: current, jurisdiction-specific visa eligibility, fees and timelines, where a confident wrong answer has real consequences and the user needs a citation to an authority. A product whose value is advice is not defensible here; one whose value is verified current requirements plus progress tracking is.

**17. Search-engine direct-answer threat.** Moderate: engines will answer "can a US citizen move to Portugal" generically; per-route detail with fees and
timelines is harder to absorb. Inference.


**18. Primary risk.** Maintenance burden and liability for wrong eligibility answers; unverified incumbents (expatsi.com ranked sixth
for the head term and was not fetched; visa-service companies did not appear in the checked results but are
known to exist and were not searched for).


**19. Evidence quality / confidence.** Fragmentation moderate (0.61, no tools in results); competitor research incomplete for this category; demand weak
(56-member cluster, head term at 1 on the Trends chain) and the members mix US, UK and Indian origins.
 Open questions from the competitor record: Go Overseas ranks second for the head term and could not be fetched (403), so the strongest general guide in the checked set is unassessed. | What expatsi.com actually is - it ranks sixth for the head term with a title suggesting a service for Americans moving abroad, but it fell outside the selection rule and is unfetched; it may already be the tool this cluster lacks. | Whether willingness to pay exists at all - the visible monetisation across fetched pages is insurance, language tutoring, affiliate commissions, magazine subscriptions and one mentorship offer, i.e. everyone monetises adjacent to the task rather than the task itself. | Which direction of travel the demand is - the member queries mix moving from the USA, from the UK and from India, and visa eligibility differs entirely by passport, so the addressable problem may be several separate products.

**20. Verdict.** Serious candidate, not a finalist on this evidence: the product is credible and AI-resistant, but the cost of
correctness across jurisdictions is the highest in the set and the incumbent landscape is unexplored.



## 6. warhammer 40k new edition  (`c_07650702`, domain tabletop_hobby_games)

**1. Problem.** Players of a tabletop wargame need to know what changed in the new edition, whether their faction's rules are
still valid, current points, and how to build a legal list. The explanation, the change tracker (mixed with
rumour), the list validator and the opinion pieces are on four different sites; the community thread names the
wrong current edition.


**2. Representative searches (measured; most-corroborated task-intent members first).** `warhammer 40k 11th edition changes`; `warhammer 40k edition changes`; `warhammer 40k new edition changes`; `warhammer 40k 10th edition changes`; `warhammer 40k 11th edition vs 10th`; `how to play warhammer 40k 11th edition`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±101.1% error, so unreliable; 6 of 30 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 30 member queries, 17 with task intent (0.57 share), 5 of them volunteered by the engines on bare probes; intent mix informational 13, track 8, find 3, how_to 2; subdomains touched: miniatures_warhammer. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_80219368` with 61 members (32 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** No fetched page answers the cluster end to end, and the pieces are split by function. The narrative explanation of what changed and how to start sits on Wargamer, which is also the most recently updated fetched page and the one that states the full rules are downloadable. The running change tracker, including detachment counts, stratagem and terrain changes and whether an existing codex is still valid, sits on Spikey Bits, but mixed with rumour. The transactional part - build a legal army under the new detachment-points system - sits only on GrimSlate, which validates budgets and slot limits live but sends readers elsewhere for the rules and current points. Opinion on whether the changes are good sits on the retailer blog, written before release. The community thread, the one place a beginner asks what do I actually need, is a year stale and names the wrong current edition. The two pages that could not be fetched (Bell of Lost Souls, and Tabletop Battles which is the cluster's dominant site at 4 of 18 results) leave a real hole in this picture.

**6. Major competitors (7 reviewed, 5 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| steamcommunity.com | community | ok | inference-free reading of the page: platform forum, no ads or affiliate links fo | Stale for this cluster - posts are dated | yes - a Steam account and the Tabletop Simulator software are involved in using the models discussed. | yes - sign-in links appear throughout and posting requires a Steam account. |
| wargamer.com | editorial | ok | inference: advertising-supported games media. The fetch found no subscription an | Page shows an update date in the byline, | no | no - the fetch found the content freely accessible. |
| spikeybits.com | editorial | ok | Reader-supported plus retail - the fetch found recurring Patreon calls to action | Page shows a dated update line naming th | no for the article; the official app is mentioned as optional for list building and points. | no |
| grimslate.com | tool | ok | Free tool with accounts - the fetch found no pricing, subscription, ads or affil | Page shows both a byline date and a late | no - it is web-based. | unknown - sign-in and sign-up exist and the free builder link does not obviously require them; the fetch could not confirm either way. |
| flipsidegaming.com | editorial | ok | Retail - the fetch found product listings with prices and store information on t | Page shows a spring 2026 date and talks  | no | no to read; the store offers accounts for purchases. |
| belloflostsouls.net | editorial | failed: HTTP | unknown | unknown - the result URL path contains a | unknown | unknown |
| tabletopbattles.com | editorial | failed: empt | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** A maintained, dated, per-rule reference rather than an article: a 10th-versus-11th diff a reader can search by rule name, a per-faction status row saying whether that faction has an 11th edition codex or is still using 10th edition rules, current points with a change date, the mission and terrain rules a game actually needs, a clear confirmed-versus-rumoured flag on everything, and a list builder that validates against those same numbers. Every one of those parts exists somewhere in the checked results; nothing holds two of them except GrimSlate, which holds the builder and the explanation but not the rules. Existing tools: GrimSlate - a free web roster builder that validates detachment-point budgets, unique-tag conflicts and leader/support slots while building (verified on its own page). Two further tools are referenced by the fetched pages but were not fetched and are therefore unverified here: the publisher's official Warhammer 40,000 app, described by Spikey Bits as free and used for list building and points, and the online field manual GrimSlate points to for current points. The input file records tool_hits 1 across the 18 checked results.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.72, tool hits 1, vendor share 0.0, unclassified 0.17, dominant site tabletopbattles.com (0.22); classes: editorial 12, unclassified 3, reference 2, community 1. Queries checked: how to play warhammer 40k 11th edition; warhammer 40k 11th edition changes.

**9. Proposed free resource.** A maintained, dated, per-rule reference: a searchable edition diff, a per-faction status table (which codex, which
edition's rules), points with change dates, a confirmed-versus-rumoured flag on everything, and a list builder
validating against the same numbers.


**10. Why users would choose it.** Currency and arithmetic in one place; the fetched pages show each part exists somewhere and nothing holds more
than two parts. Inference.


**11. Indexable useful pages / tools.** Per faction, per unit, per rule, per edition-change entry: thousands of pages with a natural long tail. Hypothesis.


**12. Repeat-use mechanism.** High for active players (every points update, every codex release, every list built). Inference.


**13. Advertising suitability.** Hobby audience with strong purchase intent (miniatures, paints, retailers); moderate CPC. Inference.


**14. Other monetization.** Retailer affiliates, premium list-builder features. Hypothesis.


**15. Maintenance requirements.** Continuous, event-driven (publisher releases); data decays fast. Measured: one fetched thread is a year stale.
 Data source and reproducibility (from the competitor record): inference: the underlying facts are public but the rights are not. Core rules are published free by the game's publisher (two fetched pages say so), points are published digitally and change, and release dates and previews are announcements anyone can track - so a diff-and-status dataset is rebuildable by a diligent maintainer. Two constraints bite: reproducing rules or points text verbatim is a copyright exposure against a litigious rights holder, and the data decays continuously, so the cost is ongoing maintenance rather than one-off collection. GrimSlate's existence shows the build is feasible; its silence on funding leaves the economics unknown.

**16. AI threat.** The most AI-resistant cluster in the set: currency after any training cutoff and validation against live points
data both defeat a memory-based assistant. Inference supported by the fetched thread's wrong-edition statement.
 Competitor-record view: inference: this is the most AI-resistant of the three clusters, for two reasons visible in the evidence. First, currency - the edition post-dates any fixed training cutoff, and the failure mode is concrete: the fetched community thread confidently states the current edition is 10th. Second, arithmetic against live data - validating a roster against detachment-point budgets and slot limits is not something a chat assistant does reliably without the current numbers. An assistant with live retrieval would handle the explanatory half well; the tracking and validation half needs maintained data.

**17. Search-engine direct-answer threat.** Low for rule-level and points-level detail; moderate for "when did 11th edition release". Inference.


**18. Primary risk.** Rights: restating rules and points text verbatim is a copyright exposure against a litigious rights holder, and
the publisher's own free app already covers list building and points (described on a fetched page, not verified).
GrimSlate already offers a free validating builder.


**19. Evidence quality / confidence.** Fragmentation strong (0.72); competitor evidence incomplete (the dominant site, 4 of 18 results, could not be
fetched); demand: the generic "warhammer 40k" head is at 61 on the Trends chain, but that is the brand, not the
problem, and the cluster has 30 members.
 Open questions from the competitor record: Tabletop Battles could not be fetched yet supplies 4 of 18 checked results, and Bell of Lost Souls supplies 2 - together a third of the cluster is unassessed, so the competitive picture may be materially different from what is described here. | Whether the publisher's own free app and community site already deliver the rules, points and list-building functions well enough to make a third-party destination redundant; the fetched pages describe the app only in passing. | How much of the rules and points data can legally be restated by a third party, and whether GrimSlate's approach has been tested. | Whether the edition is fully released at the analyst date - the fetched pages span pre-release commentary (rulebook about eighty days away, dated March 2026) and post-release guidance (full rules now available, updated August 2026), so demand may be at a transient peak.

**20. Verdict.** Serious candidate with the best AI resilience, capped by rights risk and an official app. Not a finalist unless
the legal position on restated rules and points is confirmed.



## 7. kdp self publishing  (`c_05174339`, domain writing_publishing)

**1. Problem.** A self-publishing author needs to know what publishing on Amazon KDP will cost for their specific book and
whether their manuscript file will pass review. The platform rules live on the vendor's undated help page; the
fee mechanics and worked examples live on three third-party blogs that each sell their own tooling; no interactive
calculator or manuscript checker appeared in the checked results despite "kdp publishing price calculator" being
a cluster member.


**2. Representative searches (measured; most-corroborated task-intent members first).** `how to format for kdp publishing`; `how much is kdp publishing`; `how much does it cost to publish with kdp`; `writing kdp vs publishing`; `how to publish on kindle kdp`; `alternative to kdp publishing`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 2 of 25 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 25 member queries, 17 with task intent (0.68 share), 3 of them volunteered by the engines on bare probes; intent mix how_to 12, informational 7, calculate 2, alternative 1; subdomains touched: self_publishing. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_77892681` with 8 members (9 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** No fetched page answers the cluster end to end. The platform rules and file requirements live on kdp.amazon.com ('Download and install Kindle Create.', the 6x9 default, 'convert your interior manuscript file to a PDF'). The claim that publishing is free lives on the third-party blogs ('Signing up and uploading a book to Amazon KDP costs nothing' - BookBeam; 'publishing a book on Amazon through their Kindle Direct Publishing program is free' - Reedsy). The actual fee mechanics live on Reedsy ('$0.15 per megabyte on each ebook sold'; '$1.00 [fixed cost] + (300 x $0.012) [per page cost] = $4.60'). Price ranges for the work the author must outsource live on BookBeam ('Proofreading: $200 to $500'), ZonGuru ('$2.50 and $5 for a page') and their summary tables. Royalty context lives on Wikipedia ('Amazon keeps 65% of the revenue'). Someone to do the formatting instead of the author lives on dribbble.com. A reader wanting one number for their own book must visit at least three of these and do the arithmetic themselves.

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| kdp.amazon.com | vendor | ok | Vendor-owned help content for Amazon's publishing platform; no pricing, subscrip | unknown - no last-updated or published d | True | True |
| dribbble.com | tool | ok | Marketplace/portfolio monetised through paid design services with a promotional  | unknown - no last-updated or published d | False | True |
| en.wikipedia.org | reference | ok | Non-commercial encyclopedia; no pricing, subscription or affiliate links stated. | Last edited 12 August 2026 per the page  | False | False |
| bookbeam.io | editorial | ok | Content marketing for the site's own software: free tools and a Chrome extension | unknown - no published or updated date v | unknown | unknown |
| reedsy.com | editorial | ok | Editorial content feeding a marketplace of paid freelancers plus the site's own  | Last updated on Oct 23, 2025 per the pag | False | unknown |
| zonguru.com | editorial | ok | Blog content promoting the site's own subscription Amazon seller toolkit with a  | Posted June 24, 2022 and 'Updated on Apr | False | False |
| quora.com | community | failed: HTTP | unknown | unknown | unknown | unknown |
| blog.bookbaby.com | editorial | failed: empt | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** One page that takes the book's inputs - trim size, page count, ink and paper, target marketplaces, file size, list price - and returns the printing cost, delivery fee, royalty at 35% and 70%, and break-even price, with each fee traced to a dated Amazon source; then, in the same place, a manuscript checker that ingests the author's DOCX or PDF and reports the specific margin, bleed, trim and embedded-font problems that would fail KDP review, and emits a compliant file. The cost side and the file side are currently on different sites, and neither side is interactive anywhere in the checked results. Existing tools: None found in the checked results does both. Partial tools named on the fetched pages: Kindle Create (Amazon's own formatting app, requires download and install), BookBeam's 'free tools' and Chrome extension, ZonGuru's subscription toolkit ('starts from $24/month'), Reedsy Studio plus the Reedsy freelancer marketplace, and a freelance formatting service on Dribbble. No interactive KDP cost calculator appeared in the 18 checked results, despite 'kdp publishing price calculator' being a cluster member.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.44, tool hits 2, vendor share 0.39, unclassified 0.0, dominant site kdp.amazon.com (0.39); classes: vendor 7, editorial 6, community 2, reference 1, marketplace 1, tool 1. Queries checked: how much is kdp publishing; how to format for kdp publishing.

**9. Proposed free resource.** A KDP cost and compliance tool: enter trim size, page count, ink, marketplaces, file size and list price to get
printing cost, delivery fee, royalty at each tier and break-even price, each fee cited to a dated Amazon source;
plus a manuscript checker that reports margin, bleed, trim and font problems and emits a compliant file.


**10. Why users would choose it.** A number for their book and a pass/fail on their file, instead of a formula on a blog and a vendor page with no
prices. Inference.


**11. Indexable useful pages / tools.** Per trim size x page count x marketplace cost pages, per-error formatting pages, per-tool comparison pages:
thousands. Hypothesis.


**12. Repeat-use mechanism.** Moderate: each book, each price change, each marketplace. Inference.


**13. Advertising suitability.** Author-services audience with strong purchase intent (editing, covers, courses, tools); good CPC. Inference.


**14. Other monetization.** Service-marketplace affiliates, premium file conversion, course referrals. Hypothesis.


**15. Maintenance requirements.** Fee tables must track Amazon's changes per marketplace; formatting rules change rarely. Measured: the vendor page
shows no date and blogs restate fees verbatim.
 Data source and reproducibility (from the competitor record): The load-bearing numbers - print cost formula, per-megabyte delivery fee, royalty tiers, trim sizes, margin and bleed specs - are published by Amazon and are already being restated verbatim by every third-party page checked, so the data is fully reproducible from public sources; the cost is keeping it current per marketplace, which is exactly where the fetched pages are weak (kdp.amazon.com showed no date at all, BookBeam showed none, Wikipedia's figures are dated 2009-2019). Service price ranges are self-reported market estimates and cannot be verified from the pages.

**16. AI threat.** The formula and cost ranges are fully replaceable; inspecting the actual manuscript and guaranteeing a current
fee table are not. Inference.
 Competitor-record view: Low to moderate for the explanatory half: a chat assistant states the formula, computes the example and lists the cost ranges at least as well as these articles, and the Wikipedia and blog layers are largely displaced. Resilience comes from the two things an assistant cannot do from the chat window - inspecting the author's actual manuscript file against KDP's review rules, and guaranteeing a dated, per-marketplace fee table that is correct today. A destination built on file validation plus current fee data is defensible; one built on restating the formula is not.

**17. Search-engine direct-answer threat.** High for "is kdp free" and the royalty percentages. Inference.


**18. Primary risk.** Amazon's own Kindle Create and its help pages own the formatting step, and KDP's fee pages may already include
a calculator (not present in the checked results, not verified); tool vendors (BookBeam, Reedsy, ZonGuru) have
partial tools behind subscriptions.


**19. Evidence quality / confidence.** Fragmentation 0.44 measured with 2 tool hits; 6 of 8 pages fetched; demand weak (cluster small in pass 1).
 Open questions from the competitor record: Whether Amazon's printing-cost and delivery-fee tables differ enough by marketplace to make the India and Nigeria queries in this cluster a distinct, unserved need - the fetched pages state only that rates vary by country without giving them. | What the two unreadable pages (Quora 403, BookBaby empty) contain, and whether the community answer on Quora is where the real per-title numbers are being traded. | How often KDP changes the fee tables and specs, which determines whether a cached calculator can stay correct - unknown, since the KDP help page carries no date. | Whether the KDP manuscript review failure modes are documented publicly in enough detail to build a pre-flight checker, or whether they can only be learned by submitting and failing.

**20. Verdict.** Top 20, close to finalist: reproducible fee data plus a file-validation utility, in a category where the
vendor's own calculator is the main unknown.



## 8. obd2 codes  (`c_74700978`, domain automotive)

**1. Problem.** A driver has a diagnostic trouble code and wants to know what it means for their vehicle, whether it is safe to
drive, the likely cause, how to confirm it and what the fix costs. Definitions are commodity (a public CSV holds
3,071 rows); the two top-ranked "list" results are not lists; no fetched page goes past the definition to cause,
severity or fix, and the fullest coverage is a paid iOS app.


**2. Representative searches (measured; most-corroborated task-intent members first).** `obd2 codes list`; `obd2 codes list pdf`; `how to read obd2 codes`; `obd2 scanner codes list`; `obd2 codes list free`; `how to clear obd2 codes`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'obd2 codes' at 1.5 on the chained Trends scale (root = 100, rounding error ±3.2%); 12 of 56 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 56 member queries, 42 with task intent (0.75 share), 5 of them volunteered by the engines on bare probes; intent mix how_to 19, informational 10, find 10, commercial_nav 4; subdomains touched: diagnostics. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_54620816` with 53 members (41 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** Three separate needs sit behind one head term and no fetched page serves more than one. (1) What the code means: the fullest coverage is a $0.99 iOS app claiming "more than 18546 error descriptions for almost 50 car manufacturers", and the freest is a GitHub CSV of "3071 lines" with no search interface; the two top-ranked editorial results for "obd2 codes list" are not lists at all - calamp.com names four codes and autopi.io eleven. (2) How to get the code out of the car: autopi.io explains the format but ties the procedure to its own EUR 599 / EUR 129-plus-EUR 6-a-month hardware; greatwater360autocare.com says "you'll need a code reader"; carhop.com openly hands the reader onward to "refer to the manufacturer-specific code list or use an online database". (3) What to actually do about it: nobody. Every fetched page stops at the definition and routes to a mechanic, a demo, or an appointment. The cluster's own member queries show the same seam - "obd2 codes meaning" sits next to "fix obd2 codes" and "how to clear obd2 codes".

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| autozone.com | vendor | failed: HTTP | unknown | unknown | unknown | unknown |
| apps.apple.com | vendor | ok | Paid app at "$0.99"; no in-app purchases, subscription or ads are stated in the  | Not stated - no update date; the only da | True | unknown |
| autopi.io | editorial | ok | Hardware and subscription sales - "From EUR 599" for the CAN-FD Pro and "EUR 129 | "Updated 14 Aug, 2025". | True | True |
| calamp.com | editorial | ok | Fleet telematics product marketing - "The CalAmp iOn enhances this approach by o | "Published on October 23, 2023, by Mark  | False | False |
| greatwater360autocare.com | editorial | ok | Repair-shop lead generation - "At GreatWater 360 Auto Care, we've got diagnostic | "Published on August 13, 2025". | False | False |
| carhop.com | editorial | ok | Used-car dealership and financing - the site links throughout to vehicle invento | "May 14th, 2024 by Esmeralda Salgado". | False | False |
| github.com | community | ok | None stated - no pricing, subscription, ads, affiliate links or license terms ap | Not stated - the page shows a "Latest co | False | False |
| bobistheoilguy.com | community | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** It would have to run the chain the searcher is actually on: read the code (or accept one typed in), resolve it against a table that covers manufacturer-specific ranges and not just the generic P0 set, then narrow it for the specific year/make/model, rank likely causes with a stated basis, indicate severity - is it safe to drive, must it be fixed before an emissions test - give the diagnostic sequence to confirm which cause it is, estimate parts and labour, and say when the code can be cleared and what happens if it returns. Of those steps, the fetched pages collectively deliver only the second. A single destination also needs to be free and instantly readable, since the cluster's members are saturated with "free", "pdf", "download" phrasing, and it needs a visible update date, which four of the six successfully fetched pages do not have. Existing tools: In the checked results the input file records "tool_hits": 0, and of the eight sites selected here only two are data assets rather than articles - the "Obd2 Codes List" iOS app at $0.99 (install required, "No Internet connection needed") and the mytrile/obd-trouble-codes CSV on GitHub (free, no interface). Note that four lookup-style destinations appear in the cluster's results but were not selected by the sampling rule because they are categorised unclassified: klavkarr.com, edmunds.com, innova.com and repairpal.com; they were not fetched and nothing is claimed about them here.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.42, tool hits 0, vendor share 0.16, unclassified 0.42, dominant site autozone.com (0.11); classes: unclassified 8, editorial 5, vendor 3, community 2, aggregator 1. Queries checked: how to read obd2 codes; obd2 codes list.

**9. Proposed free resource.** A code-to-vehicle diagnostic reference: every generic and manufacturer-specific code, narrowed by year, make and
model, with ranked causes and a stated basis, severity, confirming tests, parts and labour estimates and clearing
guidance, all free and dated.


**10. Why users would choose it.** The fetched pages stop at the definition and route to a mechanic or a product; a page that carries the reader to
a decision would end the search. Inference.


**11. Indexable useful pages / tools.** Code x vehicle pages: thousands of codes times popular models gives tens of thousands of long-tail pages. Hypothesis.


**12. Repeat-use mechanism.** Moderate: each new code, each vehicle in the household. Inference.


**13. Advertising suitability.** High: automotive repair has strong commercial intent (parts, scanners, local repair leads). Inference.


**14. Other monetization.** Parts and scanner affiliates, repair-shop leads. Hypothesis.


**15. Maintenance requirements.** Generic table static; manufacturer-specific tiers compiled from service literature; cause-and-fix data must be
built from repair outcomes, which no fetched page publishes.
 Data source and reproducibility (from the competitor record): The generic layer is demonstrably reproducible: a public GitHub repository holds "3071 lines (3071 loc) · 164 KB" of code-to-description rows, downloadable with no account and no stated license, and the same standardised definitions recur verbatim across every page fetched. The scarce layer is manufacturer-specific - the iOS app claims "more than 13500 additional manufacturer specific codes" against "almost 5000 generic OBDII codes", so roughly three-quarters of its value is the non-generic tier, which is compiled from service literature rather than a single download. Beyond definitions, the genuinely unavailable dataset is empirical: which cause actually fixed which code on which vehicle. No fetched page publishes that.

**16. AI threat.** Definitions are the most replaceable content in the set; vehicle-scoped diagnosis with verified outcomes is not,
but it is also the part nobody has the data for. Inference.
 Competitor-record view: inference: low for definitions, high for the diagnostic step. Reciting what P0420 or P0300 means is the most AI-replaceable content in all three clusters - carhop.com literally tells readers to go find a database elsewhere, which is the substitution an assistant performs. What resists is anything needing vehicle-specific grounding and real outcomes: which of five plausible causes is likely on a 2014 model with this mileage, what the confirming test is, and what it costs. An assistant that answers confidently but wrongly here sends someone to buy the wrong part, so verified, vehicle-scoped data retains value that a generic table does not.

**17. Search-engine direct-answer threat.** Very high for "what does P0420 mean"; lower for vehicle-specific pages. Inference.


**18. Primary risk.** Entrenched lookup sites exist but were not fetched here: the dominant checked site (AutoZone) blocked fetching,
and four lookup-style destinations (klavkarr, edmunds, innova, repairpal) fell outside the selection rule. This is
a saturated category at the definition level; the underserved subproblem is the diagnosis layer, and its data is
hard to obtain.


**19. Evidence quality / confidence.** Moderate fragmentation (0.42), incomplete competitor coverage, weak demand evidence (56-member cluster, head term
1.5 on the chain although the generic term is certainly larger than the chain suggests for this narrow phrasing).
 Open questions from the competitor record: AutoZone is the cluster's dominant site at 2 of 19 checked results but returned 403; whether its list is complete, searchable, or a lead-in to free in-store code reading is unknown. | The bobistheoilguy thread returned 403, so what enthusiast communities add beyond code definitions is unknown from this task. | Four lookup-style destinations in the results - klavkarr.com, edmunds.com, innova.com, repairpal.com - fall outside the selection rule and were not fetched; whether any already offers cause-and-fix depth is unknown. | The iOS app's coverage claims ("over a hundred thousand fault codes", "more than 18546 error descriptions") are the listing's own assertions and are unverified; its last update date is not stated and the copyright reads 2019.

**20. Verdict.** Retained in the Top 20 for the diagnosis-layer gap; not a finalist because the definition layer is saturated and
the differentiating data does not exist to collect.



## 9. garden pruning  (`c_47957693`, domain gardening)

**1. Problem.** A gardener wants to know when and how to prune each plant they own, for their climate, this year. Timing tables
are scoped to one hardiness zone or one region; the portable decision rule sits on one nursery page; tool
categories and affiliate product picks sit elsewhere; no fetched page combines the user's plants, location and
the calendar.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best garden pruning tools`; `best gardening services for pruning`; `which garden tool is used for pruning`; `best gardening services for fall pruning`; `best garden pruning`; `garden pruning schedule`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 6 of 42 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 42 member queries, 19 with task intent (0.45 share), 4 of them volunteered by the engines on bare probes; intent mix informational 23, decide 7, find 3, alternative 2; subdomains touched: trees_shrubs. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_22646740` with 58 members (42 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The answer is split across at least four page types. Timing by species sits on libguides.nybg.org (tables by pruning time, scoped to 'Zone 6 and higher') and, for one region only, on swansonsnursery.com ('Here in the Maritime Northwest...'). The decision rule that makes timing portable ('When does it bloom? Then prune it within a month or two after it finishes blooming') sits on swansonsnursery.com. Tool categories and cut-diameter thresholds sit on gardendesign.com ('Best for: Cutting branches up to 1/2 inch in diameter'). Specific products to buy sit on affiliate roundups such as gardeningknowhow.com ('I tested six of the most popular pruners available'). southernlivingplants.com only routes to seasonal sub-articles. Nothing fetched combines plant identity, the user's climate, the current date, technique and tool into one place.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| libguides.nybg.org | unclassified | ok | No pricing, subscription, ads or affiliate links stated on the page (library gui | "Last Updated: Apr 24, 2026 12:58 PM" | no | no |
| gardeningknowhow.com | editorial | ok | Affiliate commerce - 'When you purchase through links on our site, we may earn a | "Published 23 July 2026" | no | no |
| swansonsnursery.com | editorial | ok | Retail nursery content marketing - a newsletter signup offering a discount on a  | "June 5, 2024" | no | no |
| gardendesign.com | unclassified | ok | No pricing or affiliate disclosure stated; the page offers a free newsletter - ' | unknown - no published or updated date s | no | no |
| southernlivingplants.com | unclassified | ok | Brand content for a plant collection - the site promotes the Southern Living(R)  | "Published June 4, 2021" | no | no |
| homesandgardens.com | editorial | failed: cont | unknown | unknown | unknown | unknown |
| hgtv.com | unclassified | failed: HTTP | unknown | unknown | unknown | unknown |
| camdocs.camden.gov.uk | official | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** A plant-keyed pruning planner - the user enters their plants (or identifies them from a photo) and their location, and gets a dated task list for the year, each task naming the cut to make, how much to remove, and which tool handles that branch diameter, with a warning for plants where mistiming costs the next season's flowers. It would need to generalise past the two regional anchors found here (Zone 6+, Maritime Northwest) and to cover the unnamed-plant case that nybg.org explicitly defers ('It's important to get to know your shrubs.'). Existing tools: None found in the checked results - the input file records tool_hits 0 and vendor_share 0.0 for this cluster, and every page fetched was an article, a library guide or a blog calendar; no calculator, database or interactive scheduler was found.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.39, tool hits 0, vendor share 0.0, unclassified 0.5, dominant site homesandgardens.com (0.11); classes: unclassified 9, editorial 7, marketplace 1, official 1. Queries checked: best garden pruning tools; garden pruning schedule.

**9. Proposed free resource.** A plant-keyed pruning planner: enter your plants (or identify from a photo) and location, get a dated task list
for the year naming the cut, how much to remove and the tool for that branch diameter, with mistiming warnings
for plants that flower on old wood; built from public extension-service and botanic-garden tables plus
frost-date and hardiness data.


**10. Why users would choose it.** A calendar for their garden instead of a zone-limited table and a regional blog. Inference.


**11. Indexable useful pages / tools.** Per species x region pages (thousands of species), per-month regional pages, per-tool pages. Hypothesis.


**12. Repeat-use mechanism.** High and seasonal: the planner fires every year, and users add plants. Inference.


**13. Advertising suitability.** Gardening audience with seasonal purchases (tools, plants, supplies); moderate CPC. Inference.


**14. Other monetization.** Tool and plant retailer affiliates, premium reminders. Hypothesis.


**15. Maintenance requirements.** Low: horticultural timing rules are stable; species coverage grows over time.
 Data source and reproducibility (from the competitor record): inference: highly reproducible. The core dataset is a species-to-pruning-window mapping derived from bloom wood (old vs new), which is public extension-service and botanic-garden horticulture - nybg.org publishes it as plain tables and swansonsnursery.com as a seasonal list. Climate adjustment would need a public frost-date/hardiness-zone source. The only non-reproducible material found was first-party product testing on gardeningknowhow.com, which is a commerce layer rather than the core data.

**16. AI threat.** Single-plant timing questions are fully replaceable; the stateful planner (your plants, your zone, reminders) and
photo identification are not. Inference.
 Competitor-record view: inference: weak for the explainer half, moderate for the planner half. Tool taxonomy (gardendesign.com) and single-plant timing questions are fully answerable by a chat assistant. What resists substitution is the stateful part - a persistent list of the user's own plants, a calendar that fires at the right week for their location, and photo identification of a shrub the owner cannot name. A product-purchase layer also resists, since assistants cannot supply live prices or a tested ranking.

**17. Search-engine direct-answer threat.** High for "when to prune hydrangeas". Inference.


**18. Primary risk.** Planner apps and extension-service calendars exist outside the checked results (not verified); the search entry
points are single-plant questions that engines answer inline, so the stateful product must be reached through
per-species pages.


**19. Evidence quality / confidence.** Fragmentation 0.39 measured, no tool hits; 5 of 8 pages fetched; demand weak (42-member cluster, no Trends placement).
 Open questions from the competitor record: What does the Camden council PDF (the only official result) actually contain? The fetch returned 503 twice, so its role in this SERP is unknown. | Homes & Gardens is the cluster's dominant site (2 of 18 checked results) but never returned page text; its actual depth and business model are unverified. | Do any of the unfetched pages (garden.org, gardenersworld.com, almanac.com) provide the plant-by-plant, region-aware schedule that the fetched pages lack? | The cluster label mentions tomatoes and many member queries are tomato-specific, but neither checked query surfaced tomato pruning; whether a separate set of competitors owns that sub-intent is unknown.

**20. Verdict.** Top 20: reproducible data, repeat use and low maintenance, held below the finalists by the direct-answer threat
on its entry queries.



## 10. unity builds  (`c_07440867`, domain game_development)

**1. Problem.** A Unity developer's build fails, sometimes with an error string and sometimes silently, and the fix depends on the
exact Unity, package and platform-SDK versions. Fixes live in one-off forum threads pinned to 2018-2022 versions;
the vendor's own troubleshooting page covers only its cloud build service and is a year stale; nothing indexes
error strings across versions or handles the silent-failure case.


**2. Representative searches (measured; most-corroborated task-intent members first).** `unity build error`; `unity build not working`; `unity building tutorial`; `fix unity builds`; `unity best build`; `unity build list`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±101.6% error, so unreliable; 2 of 33 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 33 member queries, 19 with task intent (0.58 share), 3 of them volunteered by the engines on bare probes; intent mix informational 13, fix 6, optimize 3, calculate 2; subdomains touched: unity. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_31795045` with 33 members (35 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** This cluster's SERP splits along two meanings of "build", and no site covers both. For compiling a player, the answer lives in individual community threads: forum.unity.com (redirecting to discussions.unity.com) supplies one error - compile errors from UnityEditor references - and its fix, discussions.unity.com supplies a different one - an incompatible build path - and its fix, each pinned to a 2018-2022 Unity version with no post newer than February 2023. Unity's own troubleshooting page, docs.unity.com, covers only the cloud Build Automation service, is marked "Last updated a year ago", and hands the reader on to per-category pages and their own logs. For constructing buildings and learning the Editor, learn.unity.com and unity.com carry first-party pathways ("Unity Version: 6.3"), gamedevacademy.org a city-builder tutorial, and github.com an unrelated calculator project. Nothing fetched indexes error strings across versions, and nothing addresses the cluster's slow-build and optimisation members ("unity build slow", "fast unity build", "unity build very slow") at all.

**6. Major competitors (7 reviewed, 7 pages fetched, 0 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| docs.unity.com | vendor | ok | First-party product documentation for a cloud service; no pricing, subscription, | "Last updated a year ago" | unknown | True |
| learn.unity.com | vendor | ok | Free first-party learning platform feeding the Editor; no pricing, subscription, | not stated - no published or last-update | True | unknown |
| unity.com | vendor | ok | First-party marketing for a product with paid tiers - the page links to "Plans a | not stated - no published or last-update | True | unknown |
| forum.unity.com | community | ok | First-party community forum; no pricing, subscription, ads or affiliate links st | most recent post dated "February 16, 202 | unknown | unknown |
| discussions.unity.com | community | ok | First-party community forum; no pricing, subscription, ads or affiliate links st | most recent post dated "January 12, 2023 | unknown | unknown |
| gamedevacademy.org | editorial | ok | Free tutorial funnelling to the publisher's paid course academy, with promotiona | "November 21, 2023December 19, 2022" - t | True | unknown |
| github.com | community | ok | Open-source repository; no pricing, subscription, ads or affiliate links stated. | not stated - no date or licence informat | True | False |

**7. Why competitors do not completely solve it.** One place where a developer pastes the build log or error text and states their Unity version, target platform and key package versions, and gets back the ranked likely causes for that exact combination, the fix steps, whether it is a known regression in that version, and what changed if it worked before - including the silent-failure case ("build fails without errors") where there is no string to paste, plus a build-time diagnosis path for the optimisation half of the cluster. That means version-aware coverage of both local Editor builds and cloud Build Automation, which today sit on different sites. Existing tools: None found in the checked results. Despite the input file recording tool_hits: 1, every fetched page was documentation, a learning pathway, a forum thread, a written tutorial or a sample repository - there was no error-lookup tool, log analyser or version-aware diagnostic among them.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.47, tool hits 1, vendor share 0.37, unclassified 0.0, dominant site discussions.unity.com (0.26); classes: community 8, vendor 7, marketplace 2, reference 1, editorial 1. Queries checked: unity build error; unity building tutorial.

**9. Proposed free resource.** A version-aware build-error index: paste the error or log, state Unity version, target platform and key packages,
get ranked causes for that combination, fix steps, whether it is a known regression, and what changed since it
last worked; covering local Editor builds and cloud builds together.


**10. Why users would choose it.** One lookup instead of scanning years-old threads for a matching error string. Inference.


**11. Indexable useful pages / tools.** One page per error string x version family, plus per-package regression pages: thousands, growing with each
release. Hypothesis.


**12. Repeat-use mechanism.** High for active developers (every upgrade breaks something). Inference.


**13. Advertising suitability.** Developer audience: modest display CPC, strong fit for asset-store and tooling sponsors. Inference.


**14. Other monetization.** Asset-store affiliates, sponsored tooling, a paid log analyser. Hypothesis.


**15. Maintenance requirements.** Continuous ingestion of forum threads and release notes; freshness is the asset since fetched threads are three
years stale. Whether the forum's terms permit re-indexing is unknown.
 Data source and reproducibility (from the competitor record): inference: the raw material is public and abundant - Unity error strings, their causes and their fixes sit in thousands of Unity Discussions threads of the kind fetched here, alongside Unity's release notes and issue tracker, and both fetched threads show the pattern of error string plus confirmed remedy plus affected version. An index over that corpus is therefore reproducible in principle, and its freshness would be the real asset, since the fetched threads are three years stale and the first-party page is a year stale. What is not visible from these pages is whether Unity's forum terms permit bulk aggregation, or how much of a modern failure depends on package and platform-SDK versions not recorded in old threads.

**16. AI threat.** High for pasteable error strings, which assistants already resolve well; lower for silent failures and
version-combination regressions that need the artifact. Inference.
 Competitor-record view: inference: mixed, and the most resilient of the three clusters. Where a user can paste a distinctive error string, a chat assistant is already competitive with these threads, as both fetched errors have deterministic, documented causes. Resilience concentrates in what the assistant cannot see: the actual Editor.log, the exact Unity and package version combination, and the silent failures that three of the checked results are explicitly about ("Build fails without errors", "Unity Build fails for no reason", "Unity Build Failing without showing any other errors"). Those need the artifact, not the answer - which is the part a tool could own.

**17. Search-engine direct-answer threat.** Moderate: engines surface a single forum thread inline; the version-combination answer is less absorbable. Inference.


**18. Primary risk.** Chat assistants are already the default first stop for pasted build errors; the resilient slice (silent failures,
log analysis) is the hardest to build.


**19. Evidence quality / confidence.** Fragmentation moderate (0.47, vendor share 0.37); competitor coverage complete for the checked set (7 of 7
fetched); demand weak (33-member cluster) and the head term splits between compile failures and city-building
tutorials.
 Open questions from the competitor record: Unknown: traffic, audience size or revenue for any of these sites - no fetched page states any such figure. | Only seven distinct sites qualified under the selection rule (three vendor plus every community and editorial site in the checked results), so an eighth could not be selected without reaching into reference or marketplace results. | Whether the cluster is one intent or two is unresolved: "unity build error" returns compile-failure threads while "unity building tutorial" returns construct-a-scene tutorials, and the head term "unity builds" does not disambiguate. | Whether Unity Discussions content may lawfully be aggregated and re-indexed is unknown - no terms statement appeared on the fetched pages.

**20. Verdict.** Retained in the Top 20 as the best example of a developer error-index family; not a finalist because the AI
threat is the most direct in the set.



## 11. best settings for rocket league  (`c_73257790`, domain gaming_pc)

**1. Problem.** Same family as the Marvel Rivals cluster: platform-specific in-game settings for one title, plus "controller not
working" troubleshooting that splits by launcher and controller model. Settings advice is spread across dated
affiliate and boosting-funded blogs that turn keyboard players away; the authoritative troubleshooting source
(the publisher) and the lived-experience corpus (Steam community, 7 of 19 checked results) both blocked fetching.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best settings for rocket league`; `controller not working on rocket league pc`; `controller not working on rocket league`; `controller not working pc rocket league epic games`; `best settings for pc rocket league`; `best settings for rocket league ps4`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±57.5% error, so unreliable; 10 of 31 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 31 member queries, 29 with task intent (0.94 share), 4 of them volunteered by the engines on bare probes; intent mix decide 22, fix 7, informational 2; subdomains touched: peripherals, settings. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_27191312` with 38 members (33 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** This cluster is two jobs wearing one label, and no page does both. For 'best settings': camera and controller values in copy-friendly form are on blast.tv; the widest category sweep (gameplay, camera, controller, sensitivity, interface, graphics, audio) is on skycoach.gg, behind a boosting storefront; the most recent independent numeric set including PS5-specific values is on heresthethingblog.com; the reasoning behind control choices, with named pros, is on dignitas.gg but dated March 2020. Keyboard-and-mouse players are actively turned away - blast.tv says "the best advice we can give you is to buy a controller" and dignitas.gg's author has "never even tried using a keyboard". For 'controller not working': generic Windows and Steam fixes are on minitool.com but dated November 2020 and Steam-only, while the authoritative answer and the lived variants sit on the two sources that could not be fetched - epicgames.com support (403) and steamcommunity.com threads (429), the latter holding 7 of 19 checked results.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| epicgames.com | vendor | failed: HTTP | unknown - inference: a first-party support page for a free-to-play title is a co | unknown | unknown | unknown |
| steamcommunity.com | community | failed: HTTP | unknown - inference: Steam guides are unpaid user contributions, so there is no  | unknown - the URL's guide id (1372492557 | unknown - inference: reading a Steam Community page is open on the web, but rating or commenting requires the Steam client or a Steam login. | unknown - inference: a Steam account is normally required to comment on or rate a guide. |
| minitool.com | editorial | ok | Software sales plus partner links - the page promotes MiniTool's own products (P | "Last Updated November 29, 2020" - close | no for the article; some fixes involve built-in Windows tools, and one recommends that you "install a correct driver can fix the problem". | no |
| blast.tv | editorial | ok | Affiliate commerce plus audience registration - "As an Amazon Associate we earn  | "February 13, 2025" shown at the top of  | no | no to read; the site prompts for sign-up but the content is readable without it. |
| dignitas.gg | editorial | ok | Merchandise and affiliate promotion - "Support us by getting our merchandise in  | "31 Mar 20" (31 March 2020) - roughly si | no | no |
| heresthethingblog.com | editorial | ok | unknown - the fetch found no pricing, subscription or disclosed affiliate links  | "Posted on August 10, 2026" - about six  | no | no |
| skycoach.gg | editorial | ok | Lead generation for paid gaming services - the page promotes "Rank Boost", "Acco | "Last Updated: 03.07.2026" - the most re | no | no to read; the fetch noted that links to purchase services would require account creation. |
| monitor.biology.washington.edu | editorial | failed: DNS  | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** It would have to cover both halves. On settings: platform-branched values (PC, PS4, PS5, Xbox, Switch/Switch 2, Sideswipe - all present in the cluster's members), keyboard-and-mouse as a first-class input rather than a redirect to buy a controller, sourced and dated pro configurations instead of one author's taste, and a sensitivity output derived from the player's input device. On troubleshooting: a symptom-first diagnostic that asks controller model, launcher (Epic Games Store versus Steam) and connection type before offering a fix, rather than an unranked list - the cluster's 'fix' intent members name the Epic launcher specifically, and the fetched troubleshooting page covers only Steam. Both halves currently end with the user doing manual work with no verification step. Existing tools: none found in the checked results that were selected and fetched. The input file records "tool_hits": 0 for this cluster, and all five successfully fetched pages are static articles. Two unselected results are worth flagging as possible tools whose nature is unknown because the selection rule excluded them: thespike.gg/rocket-league/settings (rank 1, categorised unclassified, titled "Best Rocket League settings") and trophi.ai/post/the-best-settings-for-rocket-league (rank 2, categorised unclassified, on a domain that reads as an AI coaching product). Neither was fetched, so whether either is an interactive database or tool is unknown.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.68, tool hits 0, vendor share 0.05, unclassified 0.26, dominant site steamcommunity.com (0.37); classes: community 7, editorial 6, unclassified 5, vendor 1. Queries checked: best settings for rocket league; controller not working on rocket league pc.

**9. Proposed free resource.** The per-game settings database described for cluster 3, applied to this title: platform-branched values
including keyboard-and-mouse, dated pro configurations, a sensitivity converter, and a symptom-first controller
diagnostic branching on launcher and controller model.


**10. Why users would choose it.** One dated, platform-branched page instead of a 2020 pro guide, a boosting shop's table and a Steam-only fix list. Inference.


**11. Indexable useful pages / tools.** Per platform, per controller model, per patch for this title; the family's page count is the point. Hypothesis.


**12. Repeat-use mechanism.** High for active players. Inference.


**13. Advertising suitability.** Gaming audience, low CPC, peripheral affiliates. Inference.


**14. Other monetization.** Peripheral affiliates; coaching referrals carry reputational risk. Hypothesis.


**15. Maintenance requirements.** Per patch; controller support changes with publisher releases. Measured: fetched pages dated 2020 to 2026.
 Data source and reproducibility (from the competitor record): inference: the settings half is fully reproducible - every value is enterable and readable in-game, and pro configurations appear on streams and broadcasts, so a settings database is a transcription effort rather than a data-access problem. The troubleshooting half is different: the authoritative statement of which controllers a given build supports belongs to the publisher (epicgames.com), and the corpus of real user symptom-and-fix reports on steamcommunity.com has accumulated since at least 2019 per the dates in this SERP. That corpus cannot be rebuilt from public sources on demand - it can only be re-accumulated - which makes it the one genuinely defensible asset in this cluster.

**16. AI threat.** Settings lists are archetypal chat answers; the troubleshooting half resists only where the publisher's
authoritative answer or a matching user report is needed. Inference.
 Competitor-record view: inference: split. The settings half is weakly resilient - a numbered list of camera and controller values with one-line rationales is the archetypal chat answer, and heresthethingblog.com and skycoach.gg are close to that already; an assistant also avoids the trust problem of taking settings advice from a boosting vendor. The troubleshooting half is more resilient in one direction and less in another: an assistant beats minitool.com's undifferentiated list because it can ask which controller, which launcher and which symptom first, but it cannot give the publisher's authoritative answer, and it cannot substitute for a thread where someone with identical hardware says what actually worked. The durable position is a diagnostic that branches on the user's specific setup, not a recited list.

**17. Search-engine direct-answer threat.** High for settings, moderate for troubleshooting. Inference.


**18. Primary risk.** thespike.gg and trophi.ai ranked first and second for the head term and were not fetched (categorised
unclassified); either may already be the database this cluster lacks.


**19. Evidence quality / confidence.** Fragmentation 0.68 measured; competitor evidence incomplete (3 of 8 pages failed, both top-ranked results
unverified); demand weak (31 members, Trends placement unreliable).
 Open questions from the competitor record: Are thespike.gg and trophi.ai actually tools (settings database, AI coach) rather than articles? Both ranked in the top two for 'best settings for rocket league' but were categorised unclassified in the input file and so fell outside the selection rule; neither was fetched. | What do the six steamcommunity.com controller threads actually conclude, and do they converge on one fix or diverge? The fetch was rate-limited (429), so the substance of the cluster's dominant site is unread. | Does the Epic Games support article cover controllers other than the PlayStation 5 pad, and does it address the Epic Games launcher scenarios the cluster's members ask about? Unknown - 403 on two URLs. | Skycoach's "Last Updated: 03.07.2026" is ambiguous between 3 July and 7 March 2026; its price strings as extracted ("Rank Boost ($199)", "Coaching ($2199)") appear to have lost decimal separators, so no reliable price figure should be drawn from them.

**20. Verdict.** Kept as the second member of the settings family; its rank depends entirely on what the two unverified
top-ranked sites already do.



## 12. export settings youtube  (`c_50800018`, domain video_editing)

**1. Problem.** A creator wants the exact export settings for their editor and version to upload to YouTube, and help when the
export fails or looks wrong. The generic numbers are duplicated on every fetched page; the editor-specific click
paths are split across sites and years (2021 to 2026, one undated); no page computes a bitrate from the source
clip, offers a preset file, or addresses the "not working" and "slow" members.


**2. Representative searches (measured; most-corroborated task-intent members first).** `export settings youtube`; `export settings for youtube video`; `best export settings youtube`; `best export settings for youtube videos`; `fix export settings youtube`; `free export settings youtube`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 2 of 21 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 21 member queries, 21 with task intent (1.0 share), 4 of them volunteered by the engines on bare probes; intent mix convert 10, decide 3, alternative 3, calculate 3; subdomains touched: codecs_export. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_69077419` with 23 members (23 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The generic numbers - H.264 in MP4, AAC audio, 1080p/4K frame sizes, a bitrate band - are repeated on every fetched page (adobe.com, help.pic-time.com, sfxengine.com, pixflow.net, animotica.com, makeuseof.com), so that part is commodity. Everything version- and editor-specific is split: Premiere click paths on adobe.com, pixflow.net and makeuseof.com; DaVinci Resolve on alliandwill.com; Final Cut Pro only inside sfxengine.com's three-editor guide; the H.264-versus-H.265 decision on pixflow.net; wider codec and surround-audio detail on help.pic-time.com; Shorts/vertical specs only on sfxengine.com and pixflow.net. Currency is scattered across 2021 (makeuseof.com), 2023 (animotica.com), 2026 (sfxengine.com, pixflow.net, help.pic-time.com) and no date at all (adobe.com), which is itself a reason to read several pages. The cluster's fix- and speed-shaped members ("export settings youtube not working", "why is export settings youtube slow") are not answered by any fetched page; the only results pointed at them are forum threads, and the Blackmagic thread returned 403.

**6. Major competitors (8 reviewed, 7 pages fetched, 1 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| adobe.com | vendor | ok | Vendor content marketing for a paid subscription; the page carries Creative Clou | not stated - no published or last-update | True | unknown |
| help.pic-time.com | vendor | ok | Help-centre documentation attached to a hosted product; no pricing, subscription | "Written by Lauren May 28, 2026" | unknown | unknown |
| forum.blackmagicdesign.com | community | failed: HTTP | unknown | unknown | unknown | unknown |
| animotica.com | editorial | ok | Vendor blog driving installs of its own editor via a store download button; no p | "By Animotica Team - February 16th , 202 | unknown | unknown |
| sfxengine.com | editorial | ok | Content marketing for the site's own paid-looking product, with a "Pricing" link | "January 16, 2026 · Kuba Rogut" | unknown | unknown |
| makeuseof.com | editorial | ok | Ad-supported publisher with optional sign-in; no pricing, subscription or affili | "Published May 3, 2021, 12:00 PM EDT" | unknown | False |
| pixflow.net | editorial | ok | Affiliate and sponsorship disclosure plus sale of its own motion-graphics templa | "11 Jun,2026" | unknown | unknown |
| alliandwill.com | editorial | ok | Affiliate and own-product sales - discounted editing products plus Amazon affili | "Aug 19" - day and month only, year not  | unknown | unknown |

**7. Why competitors do not completely solve it.** One place that takes the editor and its version, the delivery target (long-form, Shorts, 4K60), and the source clip's resolution, frame rate, length and colour space, then returns the exact field-by-field values for that editor's export dialog, a computed bitrate rather than a band, a downloadable preset file, an explanation of what YouTube re-encodes after upload and why processing looks soft or slow at first, and a symptom-driven path for exports that fail, stall or produce oversized files - all visibly dated and tied to a named editor build. Existing tools: None found in the checked results. The input file records tool_hits: 0 for this cluster, and all seven fetched pages were articles, vendor guides or help-centre documents; no calculator, preset generator or settings database appeared.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.67, tool hits 0, vendor share 0.28, unclassified 0.06, dominant site adobe.com (0.22); classes: editorial 9, vendor 5, community 3, unclassified 1. Queries checked: best export settings youtube; export settings youtube.

**9. Proposed free resource.** A per-editor, per-version export guide with a bitrate calculator driven by resolution, frame rate and length,
downloadable preset files, an explanation of YouTube's re-encoding, and a symptom path for failed or oversized
exports, each page dated and tied to a named editor build.


**10. Why users would choose it.** The exact dialog for their editor version plus a preset they can import, instead of a generic table and ads. Inference.


**11. Indexable useful pages / tools.** Editor x version x target (long-form, Shorts, 4K60) pages: hundreds, plus calculator pages. Hypothesis.


**12. Repeat-use mechanism.** Low to moderate: revisited on editor updates and new delivery formats. Inference.


**13. Advertising suitability.** Creator audience, moderate CPC (software, hardware, storage); vendor sponsorship fit. Inference.


**14. Other monetization.** Software and hardware affiliates, preset packs. Hypothesis.


**15. Maintenance requirements.** Re-verify menu paths and screenshots per editor release; values restate YouTube's public guidance. Measured decay
in the fetched pages.
 Data source and reproducibility (from the competitor record): inference: fully reproducible from public sources - the recommended values restate YouTube's published upload encoding guidance (help.pic-time.com and animotica.com both point back at it), and the per-editor field names and menu paths are observable in the shipping editors, including Resolve's free Windows version named on alliandwill.com. The recurring cost is maintenance: screenshots and menu paths have to be re-verified per editor release, which is precisely where the fetched pages decay (makeuseof.com 2021, animotica.com 2023, adobe.com undated).

**16. AI threat.** High for the head question; resilience only in presets, calculators tied to the user's footage, and failure
diagnosis. Inference.
 Competitor-record view: inference: low for the head question. A chat assistant already answers "what codec, container and bitrate for YouTube" as well as these pages do, and does it without ads. Resilience only appears where the answer depends on state the assistant does not have - the user's actual project and source footage, the exact dialog of the installed editor version, a downloadable preset artifact, and diagnosis of an export that failed or an upload that looks wrong, which needs logs or the file itself.

**17. Search-engine direct-answer threat.** High: "best bitrate for 1080p youtube" is a one-line inline answer. Inference.


**18. Primary risk.** A small, AI-exposed problem; the durable parts (presets, diagnostics) are features rather than a site.


**19. Evidence quality / confidence.** Fragmentation 0.67 measured; competitor coverage good (7 of 8 fetched); demand weak (21 members, no Trends placement).
 Open questions from the competitor record: Unknown: search volume, traffic or revenue for any of these sites, and whether this cluster's traffic is shifting to assistants - no fetched page states any such figure. | The fix/slow subset of the cluster (the input file records organic_problem_members: 4 of 21 members) had no fetchable destination; whether real demand sits behind "export settings youtube not working" and "why is export settings youtube slow" is unresolved. | forum.blackmagicdesign.com blocks automated fetching with 403, so both its content and whether Resolve users' troubleshooting is well served there remain unknown. | Whether a downloadable preset or a bitrate calculator would be used, or whether creators simply want a dated table, is untested.

**20. Verdict.** Top 20, not a finalist: real fragmentation but a narrow, largely AI-replaceable problem.



## 13. pc gaming emulator  (`c_06637354`, domain gaming_pc)

**1. Problem.** A player wants to run console games on a PC: which emulator, whether their machine can run it, the BIOS or
firmware prerequisites, per-game compatibility and fixes when it fails. Choosing is answered three times over with
overlapping listicles; downloads sit on a directory; prerequisites appear on one page; per-game compatibility,
hardware sizing and a symptom-to-fix path are absent from every fetched page.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best pc gaming emulator`; `top 5 gaming emulator for pc`; `top 10 gaming emulator for pc`; `fix pc gaming emulators`; `pc gaming emulator tutorial`; `how many pc gaming emulators`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±101.6% error, so unreliable; 1 of 34 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 34 member queries, 17 with task intent (0.5 share), 3 of them volunteered by the engines on bare probes; intent mix informational 12, decide 6, commercial_nav 5, alternative 4; subdomains touched: emulation. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_39083536` with 34 members (36 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** Choosing an emulator is answered many times over - retrododo.com (12, "All Tested", updated May 2026), beebom.com (10, updated February 2026) and esports.net (10, August 2025) overlap heavily on the same names. Getting the software is a separate stop, en.softonic.com, which hands over downloads but explains nothing. The prerequisites that actually block a first launch appear on exactly one fetched page: beebom.com names the PlayStation BIOS dump, RPCS3 firmware and Azahar CCI requirement. Legality sits on forums.pcgamer.com and in a paragraph of esports.net. Hardware sizing is implied by logicalincrements.com and per-game failure by the YouTube result, but neither could be fetched, so both are unknown. Nothing fetched offers per-game compatibility or a symptom-to-fix path, even though the cluster contains "pc gaming emulator not working", "pc gaming emulator problems" and "pc gaming emulator compatibility list". The SERP is also split between two meanings of emulator: retro-console (retrododo, beebom, esports.net) and Android-on-PC (marksangryreview.com, and memuplay.com in the unfetched results).

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| en.softonic.com | tool | ok | Ad- and download-monetised directory; the footer carries advertising and publish | not stated - no publication or update da | True | False |
| retrododo.com | editorial | ok | Affiliate-supported editorial with memberships, tipping and a linked accessory s | "Updated on May 13, 2026" | unknown | False |
| forums.pcgamer.com | community | ok | Publisher-run forum; no pricing, subscription, ads or affiliate links stated in  | most recent post dated "Apr 4, 2022" | unknown | True |
| esports.net | editorial | ok | Ad-supported editorial; the article states most emulators are free but no affili | "August 28, 2025" | unknown | False |
| beebom.com | editorial | ok | Ad-supported editorial that reports each emulator's own pricing (free, paid tier | "Updated: February 18, 2026" | unknown | False |
| youtube.com | community | failed: empt | unknown | unknown | unknown | unknown |
| logicalincrements.com | editorial | failed: empt | unknown | unknown | unknown | unknown |
| marksangryreview.com | editorial | failed: empt | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** One place that takes the console or game the user wants to play plus their PC's specifications, and returns the emulator to use and its current build, whether that machine can run it, the BIOS/firmware or decryption prerequisites with a legal way to satisfy them, a known-good settings profile for that game, and a symptom-driven fix path (crash, stutter, black screen, audio desync) tied to emulator version - i.e. choosing, setup, compatibility and troubleshooting in one flow rather than across four sites. Existing tools: Only en.softonic.com among the checked results is a tool in any sense, and it is a download directory, not a compatibility or diagnostic tool (the input file records tool_hits: 1 for this cluster). Per-game compatibility databases maintained by emulator projects were not present in the checked results, so their existence and quality are unknown from this evidence.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.61, tool hits 1, vendor share 0.0, unclassified 0.28, dominant site retrododo.com (0.11); classes: editorial 8, unclassified 5, community 3, reference 1, tool 1. Queries checked: best pc gaming emulator; fix pc gaming emulators.

**9. Proposed free resource.** A game-first emulation guide: enter the game and your PC specs, get the emulator and build, whether the machine is
fast enough, prerequisites with a legal path, a known-good settings profile and a symptom-driven fix path tied to
emulator version.


**10. Why users would choose it.** Choosing, setup, compatibility and troubleshooting in one flow instead of four sites. Inference.


**11. Indexable useful pages / tools.** Per game x emulator x version compatibility pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** Moderate to high (each game, each emulator update). Inference.


**13. Advertising suitability.** Constrained: ad networks are cautious with emulation and ROM-adjacent content; hardware affiliates fit. Inference.


**14. Other monetization.** Hardware affiliates, controller affiliates. Hypothesis.


**15. Maintenance requirements.** Per-game compatibility must be tested or aggregated from emulator projects' wikis; continuous. Measured: the choice
layer is commodity across three fetched sites.
 Data source and reproducibility (from the competitor record): inference: the choosing layer is trivially reproducible - emulator names, supported systems, price tiers and BIOS requirements are all published by the projects themselves, which is why three fetched sites carry near-identical lists and why the layer has no defensibility. The layer that would be worth building, per-game and per-version compatibility plus hardware sizing, is not reproducible from a single public feed: it needs either systematic testing (which retrododo.com gestures at with "All Tested") or aggregation of emulator-project compatibility wikis and user reports, neither of which appeared in the checked results.

**16. AI threat.** High for "best emulator"; lower for current per-game behaviour on a named build and machine-specific diagnosis. Inference.
 Competitor-record view: inference: low for the head term. "Best PC emulator" is exactly the stable, well-documented question a chat assistant answers better than an ad-laden listicle, and the fetched lists converge on the same names anyway. Resilience lies in the parts none of the fetched pages hold: current per-game behaviour on a named emulator build, whether a specific PC is fast enough, and a diagnosis that depends on the user's logs, GPU driver and settings - all state-dependent and fast-changing.

**17. Search-engine direct-answer threat.** High for the head term. Inference.


**18. Primary risk.** Emulator projects maintain their own compatibility databases (not present in the checked results, not verified
here), and legality plus ad-policy exposure limit monetisation. The cluster also mixes retro-console and
Android-on-PC senses of "emulator".


**19. Evidence quality / confidence.** Fragmentation 0.61 measured; 3 of 8 pages failed; demand weak (34 members, Trends placement unreliable).
 Open questions from the competitor record: Unknown: traffic, audience size or revenue for any of these sites - no fetched page states any such figure. | Three of eight selected pages could not be read (youtube.com, logicalincrements.com, marksangryreview.com), so the hardware-sizing and per-game-failure corners of this cluster are unassessed. | Whether the cluster is one problem or two - retro-console emulation versus Android-on-PC emulation - is unresolved; the checked results contain both, and the head term does not disambiguate. | Whether per-game compatibility data can be aggregated lawfully and kept current, and whether emulator projects' own wikis already satisfy that need, is unknown from the checked results.

**20. Verdict.** Top 20 on fragmentation; excluded from finalists for monetisation and legal constraints and likely project-run
compatibility databases.



## 14. wood stain remover  (`c_51963424`, domain diy_woodworking)

**1. Problem.** Two jobs share a result page: applying stain to a door or deck (how much product, which colour on which species,
real cure and recoat times) and removing stains. Four vendor blogs repeat the same procedure and none states
quantity or colour-on-species; "wood deck stain calculator" is an explicit member with no calculator in the results.


**2. Representative searches (measured; most-corroborated task-intent members first).** `ways to remove oil stains best`; `best method to apply wood stain`; `best type of stain for wood deck`; `what kind of stain is the easiest to apply on wood`; `how to wood stain a door`; `how to wood stain a table`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'wood stain remover' at 1.9 on the chained Trends scale (root = 100, rounding error ±21.3%); 10 of 69 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 69 member queries, 49 with task intent (0.71 share), 4 of them volunteered by the engines on bare probes; intent mix how_to 27, informational 20, decide 9, calculate 7; subdomains touched: finishing, laundry_care, woodworking. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_42910658` with 4 members (4 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** This cluster is really two unrelated jobs, and the checked queries land on different sites for each. For applying stain to a door, the procedure is duplicated across four manufacturer and retailer blogs (rustica.com, realcraft.com, zar.com, krosswood.com) that each hold one missing piece: rustica.com supplies the finish-system framing and the dry-to-touch warning, realcraft.com supplies a photographed worked example with a 24-hour between-sides wait but only for one wood and one product, zar.com supplies the only concrete cure times and the broadest coverage of stain types, and krosswood.com supplies the pre-stain conditioner rule for pine and fir while deferring drying times back to the product label. Not one of the six fetched pages states how much stain a door needs, and none gives a method for choosing a colour for a given species. For removing stains, the checked query "ways to remove oil stains best" returns laundry content (grove.co, whirlpool.com) and a carpet/fabric flipbook (issuu.com) - none of which mentions wood at all - and the one vendor-category result is a patent for staining fiberglass doors. The cluster's own wood-stain-removal members ("best stain remover for wood", "how to remove old stain", "wood stain remover not working") are served by none of the pages fetched.

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| patents.google.com | vendor | failed: HTTP | unknown | unknown | unknown | unknown |
| issuu.com | vendor | ok | Publishing platform; the page structure carries advertisements and prominently o | Published on Jan 3, 2025 per the page. | no | unknown - a sign-up option is displayed, but the page does not state that reading requires one. |
| rustica.com | editorial | ok | Retail - the page carries links to the site's own barn doors and hardware throug | Published Tuesday, July 31st, 2018, with | no | no - a login option exists but is not required to read the guide. |
| realcraft.com | editorial | ok | Retail - the page links its own Osmo stain collection, unfinished doors and door | Published June 17, 2022 per the page. | no | unknown - not stated on the page. |
| zar.com | editorial | ok | Manufacturer content marketing - product links to its own semi-transparent, soli | unknown - no published or updated date s | no | unknown - not stated on the page. |
| krosswood.com | editorial | ok | Door manufacturer promoting its own doors, stain samples and consultation servic | March 11, 2024, by Brady Kohl, per the p | no | no to read; the site has an optional Pro Program login. |
| grove.co | editorial | ok | Retail - multiple product links into Grove Collaborative's own store (dish soap, | Last Updated: August 31, 2022 per the pa | no | unknown - not stated for reading; membership signup is promoted for purchases. |
| whirlpool.com | editorial | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** One destination would have to (a) disambiguate at the door: applying stain versus stripping old stain versus removing a spill stain from wood versus removing an oil stain from fabric - the checked queries show searchers in all four buckets landing on the same results; (b) compute quantity from door or deck dimensions and the product's stated coverage rate, which no fetched page does and which several cluster members ask for directly ("how much wood stain for a door", "how much wood stain for a deck", "wood deck stain calculator"); (c) branch on wood species, telling the user whether their wood needs a conditioner and how a given colour reads on it, with visual evidence - realcraft.com shows this is possible but only for one species; (d) give the real timeline - recoat, cure, rehang, weather exposure - in one place instead of deferring to a label as krosswood.com does; and (e) for removal, branch by finish type and stain type on wood, which is absent from every page fetched. Existing tools: none found in the checked results - the cluster's fragmentation record reports tool_hits 0, and all eight selected results were blog articles, a patent document or a hosted flipbook. In particular no stain quantity calculator appeared, despite "wood deck stain calculator" being an explicit cluster member and 7 calculate-intent members in the cluster.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.56, tool hits 0, vendor share 0.11, unclassified 0.28, dominant site rustica.com (0.06); classes: editorial 9, unclassified 5, vendor 2, community 1, marketplace 1. Queries checked: how to wood stain a door; ways to remove oil stains best.

**9. Proposed free resource.** A stain quantity and species guide: coverage-rate and cure-time data compiled across brands from technical data
sheets, a calculator from door or deck dimensions, photographic colour-on-species samples, and a disambiguating
entry for apply versus strip versus remove.


**10. Why users would choose it.** A number and a colour decision instead of four identical procedures that defer to the product label. Inference.


**11. Indexable useful pages / tools.** Product x species x surface pages plus calculators: thousands, if the data-sheet compilation is broad. Hypothesis.


**12. Repeat-use mechanism.** Low (seasonal projects). Inference.


**13. Advertising suitability.** Home-improvement audience, good CPC, retailer affiliates. Inference.


**14. Other monetization.** Retailer affiliates, brand sponsorship. Hypothesis.


**15. Maintenance requirements.** Data-sheet compilation refreshed per product line; colour samples must be produced physically.
 Data source and reproducibility (from the competitor record): inference: the procedural content is commodity and fully reproducible - four fetched pages describe the same sequence. Two genuine data assets are visible but unbuilt in the checked results: (1) coverage rate and cure/recoat times per product, which exist on manufacturers' public technical data sheets and labels and could be compiled across brands - zar.com shows one brand's version of this, and krosswood.com's deferral to the label shows the gap; and (2) how a given stain colour reads on a given species, which realcraft.com demonstrates photographically for sapele mahogany but which would have to be produced, not scraped, since it requires physical samples. Neither is protected by anything but the work of assembling it.

**16. AI threat.** Procedure fully replaceable; product-specific coverage and cure data plus photographic samples resist. Inference.
 Competitor-record view: inference: low for the how-to layer - four of six readable pages are near-identical procedures, which is the strongest available evidence that this content is commodity and an assistant can generate it while also adapting to the user's wood and product. Resilience is higher for three things: product-specific cure times and coverage rates, which an assistant should not invent and which zar.com and krosswood.com both point to as label data; visual colour-on-species evidence, which is photographic; and a quantity calculation tied to real measurements and a real product's coverage rate, which is a computation over data rather than a recall task.

**17. Search-engine direct-answer threat.** High for "how much stain for a deck" generic answers. Inference.


**18. Primary risk.** Paint and stain manufacturers publish their own coverage calculators (known to exist, not in the checked results,
not verified here); the removal half of the cluster was never checked.


**19. Evidence quality / confidence.** Fragmentation 0.56 measured but on an ambiguous head term; 2 of 8 pages failed; demand weak (69 members with a
head term that misnames the cluster).
 Open questions from the competitor record: Two of eight selected pages failed with HTTP 503 (patents.google.com, whirlpool.com), so the vendor-category coverage in this cluster is thin; a second pass is needed to confirm whether Google Patents offers anything of use to a homeowner here, which on the title alone looks unlikely. | The cluster label is "stain wood apply furniture" and the head term is "wood stain remover", but neither checked query targets removal from wood. The removal half of the cluster - members such as "best stain remover for wood", "how to remove old stain", "wood stain remover not working", "fix wood water stain" - has no competitor evidence at all in this pass and should be its own SERP check. | "wood deck stain calculator" is an explicit cluster member and decks appear repeatedly in the member list, but the checked query was about doors, so deck-specific competition and any existing deck stain calculators are unexamined. | It is unknown from the fetched pages what coverage rates manufacturers publish and whether they are consistent enough across brands to power a quantity calculator; this needs a direct check of technical data sheets rather than blog pages.

**20. Verdict.** Top 20 for the calculator gap; not a finalist because manufacturers likely own the calculator and the cluster
is two problems.



## 15. cities skylines  (`c_94231707`, domain gaming_titles)

**1. Problem.** Players of a city-building game series want to know what the latest patch changed, whether the sequel is now
better than the original, whether their mods still work, and whether it is worth buying today. The publisher
posts one dated page per patch with no timeline; the aggregating patch databases both failed to load; the
comparison that ranks is frozen at launch; the store page shows a sentiment split it does not explain.


**2. Representative searches (measured; most-corroborated task-intent members first).** `cities skylines 1 vs 2`; `cities skylines 2 update`; `cities skylines not working`; `cities skylines error`; `cities skylines 2 slow`; `cities skylines update`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'cities skylines' at 16.2 on the chained Trends scale (root = 100, rounding error ±4.8%); 34 of 120 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 120 member queries, 63 with task intent (0.53 share), 3 of them volunteered by the engines on bare probes; intent mix informational 48, how_to 11, track 9, commercial_nav 9; subdomains touched: sim_strategy. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_92525747` with 120 members (42 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The answer is scattered across formats, and the aggregating pages are the ones that would not load. Authoritative but single-patch change information is on paradoxinteractive.com, dated 2026-02-18, one page per patch with its own known-issues list. The version timeline that would turn those into an answer to 'what is the latest update' is supposedly on steamdb.info and cs2.paradoxwikis.com, and neither could be fetched. Live sentiment and price are on the Steam store page, where the split between 'Mostly Positive (835)' in the last 30 days and 'Mixed (34,703)' overall is itself the unanswered question. The feature comparison between the two games is on screenrant.com but frozen at 'Published Oct 24, 2023', the sequel's release day, so it predates three years of patching. Everything after that is social - Facebook, TikTok, Steam discussions, the Paradox forum - eleven of nineteen checked results are community pages, and the one that rendered offered only 'full of people complaining about it basically everyday'.

**6. Major competitors (8 reviewed, 4 pages fetched, 4 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| paradoxinteractive.com | vendor | ok | Publisher-owned marketing and support content for a paid game, with a 'Buy now'  | Dated 2026-02-18 on the page | True | False |
| screenrant.com | editorial | ok | Ad-supported publisher content with a premium subscription tier and a marketing  | Published Oct 24, 2023, 9:31 AM EDT - th | False | False |
| store.steampowered.com | marketplace | ok | Storefront selling the game, with a time-limited discount and in-game purchases  | Release date shown as 'Released Oct 24,  | True | True |
| facebook.com | community | ok | unknown - not stated on the page | unknown - no date was visible on the fet | False | unknown |
| steamdb.info | unclassified | failed: HTTP | unknown | unknown | unknown | unknown |
| cs2.paradoxwikis.com | unclassified | failed: the  | unknown; the only monetisation-adjacent text retrieved was the error page's inst | unknown | unknown | unknown |
| forum.paradoxplaza.com | community | failed: the  | unknown | unknown | unknown | unknown |
| tiktok.com | community | failed: only | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** A maintained, dated version timeline for both titles that links each patch to what it changed and to what players reported afterwards, sitting next to a comparison that is versioned rather than frozen at launch - 'as of patch 1.5.7f1, here is where CS2 now beats or still trails CS1 on performance, mods, economy and DLC' - plus current price and review trend for both games and their DLC. In other words it must answer 'should I buy 1 or 2 today, at this price, on this hardware' rather than 'what shipped in October 2023'. No checked result combines the change history, the sentiment trend and the comparison. Existing tools: None found in the checked results that could be verified - the input file records tool_hits of 0. Two candidate databases ranked (SteamDB's patch list and the Cities Skylines 2 wiki's Patches page) but both failed to fetch, so whether either is maintained and complete is unknown. The Steam store page is the only live data source that loaded, and it covers price and reviews, not changes.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.63, tool hits 0, vendor share 0.11, unclassified 0.16, dominant site steamcommunity.com (0.32); classes: community 11, unclassified 3, marketplace 2, vendor 2, editorial 1. Queries checked: cities skylines 1 vs 2; cities skylines 2 update.

**9. Proposed free resource.** A maintained version timeline for both titles linking each patch to its changes, mod breakage reports and the
review trend afterwards, next to a versioned comparison ("as of patch X") with current prices; the mod
compatibility half connects to the cross-game mod-compatibility family noted in the report.


**10. Why users would choose it.** Freshness: today's patch, today's price, this month's sentiment, in one place. Inference.


**11. Indexable useful pages / tools.** Per patch, per mod, per DLC pages for two titles: hundreds. Hypothesis.


**12. Repeat-use mechanism.** High for active players around each patch. Inference.


**13. Advertising suitability.** Gaming audience, low CPC; store affiliates. Inference.


**14. Other monetization.** Store affiliates, hardware affiliates. Hypothesis.


**15. Maintenance requirements.** Event-driven per patch; community sentiment collection faces bot-blocking (three of four community sources failed).
 Data source and reproducibility (from the competitor record): Mixed. The change descriptions originate with Paradox and can only be copied, not derived - though they are published openly and are trivially crawlable, which is presumably how the wiki and SteamDB pages exist. Build and release timing is observable from Steam's public depot and news feeds. Review scores and their trend are Steam's own platform data, readable but not rebuildable. Player sentiment is spread over Facebook, TikTok, Steam discussions and the Paradox forum, three of which blocked or failed to render for an automated reader, so any product depending on scraping community sentiment faces real collection risk.

**16. AI threat.** Low for the live half (an assistant cannot state today's patch or price), high for the launch-era comparison. Inference.
 Competitor-record view: High for the live half, low for the static half. A chat assistant cannot state today's patch version, today's discount or this month's review trend, and should not guess at them - which is why an update query returns a forum, a database and a wiki rather than an explainer. But the launch-era 1-vs-2 feature comparison is exactly the stable, widely-written-up material an assistant reproduces well, and the ScreenRant article adds nothing an assistant would not say. A defensible destination here is a freshness product, not a knowledge product.

**17. Search-engine direct-answer threat.** Moderate. Inference.


**18. Primary risk.** SteamDB and the game's wiki likely already provide the patch timeline (both ranked, both failed to fetch); this is
a single-franchise problem unless generalised to a per-game patch-and-mod tracker.


**19. Evidence quality / confidence.** Fragmentation 0.63 measured; 4 of 8 pages failed including both candidate databases; demand: the head term is a
game brand at 16 on the Trends chain, not the problem.
 Open questions from the competitor record: Whether SteamDB's patch list and the cs2 wiki's Patches page are actually complete and current - both ranked for the update query and neither could be read, so the existing-coverage picture for this cluster is materially incomplete. | Whether the 403 and the loading errors affect real visitors or only automated clients, which decides whether those pages are genuine competitors or only nominal ones. | Why recent Steam reviews (73% positive, last 30 days) diverge so sharply from all-time (57%), and whether that recovery is what the 1-vs-2 searchers are trying to establish. | Whether the cluster's 'not working', 'error fix' and 'slow' members are patch-version-specific, which would make a version-to-known-issues index the real product.

**20. Verdict.** Top 20 as the clearest "freshness product" example and as a bridge to the mod-compatibility family; not a
finalist on its own.



## 16. renovation cost per square foot  (`c_05810365`, domain real_estate_housing)

**1. Problem.** A homeowner wants a defensible renovation cost for their rooms, finish level and location. National ranges on
three fetched pages disagree while citing the same upstream source; room-level rates sit on one page; local rates
only for New York; every calculator routes into a contractor sales funnel and one site says outright that it does
not compute individual costs.


**2. Representative searches (measured; most-corroborated task-intent members first).** `renovation cost per square foot`; `renovation costs per square foot uk`; `house cleaning rates per square foot`; `house cleaning cost per square foot`; `renovation cost per square foot ontario`; `renovation cost per sq`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'renovation cost per square foot' at 0.9 on the chained Trends scale (root = 100, rounding error ±4.6%); 4 of 63 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 63 member queries, 60 with task intent (0.95 share), 3 of them volunteered by the engines on bare probes; intent mix calculate 56, commercial_nav 2, decide 1, fix 1; subdomains touched: cleaning_moving, hardscape_irrigation, renovation_value, smart_home, trees_shrubs, vegetables, woodworking. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_03480751` with 39 members (24 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** Four fragments, none co-located. The national range sits on thisoldhouse.com ("$15–$150 per square foot"), nerdwallet.com ("$15 to $150 per square foot") and sofi.com ("$15 and $60 per square foot") - and these three disagree with each other while two of them cite the same upstream source, Angi. The room-level breakdown, which is what "per square foot" actually needs, is on blockrenovation.com (nine room types, kitchens "$150–$400+" down to bedrooms "$40–$120"). The local layer is separate again: brickunderground.com and the 2018 Goodreads post both give NYC bands, and no fetched page gives a second city. The step from a range to a number is on nobody's page - homeadvisor.com says outright that it "does not calculate individualized costs without professional input", sofi.com says "The best way to estimate your renovation costs is to talk to a local contractor", and thisoldhouse.com converts the reader into a lead for All Star Pros. The reader must therefore combine a national tier, a room mix, a local multiplier they cannot find, and a contingency ("an extra 10-15% as a cushion") by hand.

**6. Major competitors (8 reviewed, 7 pages fetched, 1 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| angi.com | tool | failed: HTTP | unknown | unknown | unknown | unknown |
| sofi.com | tool | ok | Lending - the page promotes SoFi's financial products, including "SoFi now offer | "September 17, 2026 · 16 minute read". | False | False |
| nerdwallet.com | tool | ok | Advertising and partner compensation: "partner compensation is one of several fa | "Updated Apr 20, 2026". | False | False |
| homeadvisor.com | tool | ok | Lead generation - the page's calls to action are "Get quotes from up to 3 pros!" | "Updated Jun 19, 2026" in the byline, al | False | unknown |
| goodreads.com | tool | ok | Author/business promotion - the post carries embedded links to the author's real | "Published on April 17, 2018 09:46" - ei | False | False |
| thisoldhouse.com | editorial | ok | Triple: affiliate ("We may be compensated if you purchase through links on our w | "Updated 03/31/2026". | False | False |
| blockrenovation.com | editorial | ok | Its own renovation marketplace plus financing - "Block Renovation connects you w | "08.11.2025" published, with schema last | False | unknown |
| brickunderground.com | editorial | ok | Sponsored and partner content - "Pro Tip" sections link to Urban Standard and Bo | Not stated - no publication or last-upda | False | False |

**7. Why competitors do not completely solve it.** It would have to take scope (which rooms, what finish level), size (square footage per room, not per house), and location, and return one defensible number with a stated range - which means (a) room-level rather than whole-house rates, since the fetched spread between a bedroom and a kitchen is roughly three-to-one, (b) real local multipliers beyond NYC, which no fetched page provides, (c) a visible, dated methodology, since four of seven fetched pages state no source and one is eight years old, (d) the contingency and permit/approval steps that only the two NYC pages mention, and (e) no requirement to surrender contact details to see the number. The gap is not more content; it is the arithmetic and the localisation that every page leaves to the reader. Existing tools: Partially: Block Renovation has an on-page renovation cost calculator ("Get an initial estimate based on your space and project details"), NerdWallet links out to a "home improvement cost calculator", and HomeAdvisor and This Old House offer zip-code quote forms that route to contractors rather than compute. None of the fetched pages returns a computed project number without entering a sales funnel. The input file records six tool hits across 19 results, but one of those "tool" results is a 2018 Goodreads author blog post.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.53, tool hits 6, vendor share 0.0, unclassified 0.21, dominant site thisoldhouse.com (0.11); classes: editorial 9, tool 5, unclassified 4, community 1. Queries checked: best renovation cost per square foot; renovation cost per square foot.

**9. Proposed free resource.** A scoped estimator: rooms, square footage per room, finish level and location produce one number with a stated
range, dated methodology and local multipliers, with contingency and permit steps, and no contact-detail gate.


**10. Why users would choose it.** The arithmetic and localisation every fetched page leaves to the reader. Inference.


**11. Indexable useful pages / tools.** Room x finish x city pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** Low. Inference.


**13. Advertising suitability.** Very high CPC category (home services); lead-gen dominated. Inference.


**14. Other monetization.** Contractor leads (the incumbents' model), financing affiliates. Hypothesis.


**15. Maintenance requirements.** National layer rebuildable from published sources; the scarce input is local bid data the marketplaces generate and
do not publish.
 Data source and reproducibility (from the competitor record): Largely reproducible, and visibly recycled. sofi.com cites "Angi.com" and "HomeGuide.com"; thisoldhouse.com cites "estimates from Angi" plus "two nationwide homeowner surveys conducted in 2026"; brickunderground.com, blockrenovation.com and the Goodreads post state no source at all. The only claimed primary data is HomeAdvisor's - "We surveyed over 10,000 real customers about their project costs" - together with public inputs it names including "the U.S. Bureau of Labor Statistics". A new entrant could rebuild the national layer from published sources; the genuinely scarce input is actual local bid data, which the marketplaces generate as a by-product and none of them publishes.

**16. AI threat.** High: an assistant restates the ranges and does the multiplication; only transaction-grounded local data resists. Inference.
 Competitor-record view: inference: low for what these pages currently publish and moderate for what they withhold. A national per-square-foot range restated from two named public sources is precisely what a chat assistant produces, and the assistant will also do the multiplication the reader is currently left to do. What survives is anything grounded in transaction data - real bids in a specific market at a specific date - and the contractor introduction itself. Note also that brickunderground.com openly says the requested metric is weak ("per-square-foot averages are slightly less predictive"), which means the durable answer is a scoped estimate, not a rate.

**17. Search-engine direct-answer threat.** High for national ranges. Inference.


**18. Primary risk.** Saturated lead-gen category (HomeAdvisor, Angi, Block) with calculators already present (6 tool hits in the checked
results); the differentiating data is proprietary to the incumbents.


**19. Evidence quality / confidence.** Fragmentation 0.53 measured with 6 tool hits; 7 of 8 pages fetched; demand weak (63 members, head term at 1 on the chain).
 Open questions from the competitor record: Angi returned 403 and is cited as the upstream source by two other fetched pages; how its numbers are produced, and how many downstream pages depend on them, is unknown. | No fetched page states local multipliers outside NYC, so whether city-level per-square-foot data exists at scale anywhere is unknown. | HomeAdvisor states a survey of "over 10,000 real customers" but not the sample per project type or region, so the precision of its ranges is unknown. | Block Renovation and HomeAdvisor both broker actual jobs; whether their published ranges are derived from their own bid data is not stated on the fetched pages.

**20. Verdict.** Top 20 as a documented example of "competitors exist but withhold the number"; not a finalist given incumbents
and data access.



## 17. pet bird care  (`c_40830266`, domain pets)

**1. Problem.** A prospective or new bird owner wants to choose a species that fits their household and know what it costs and
how to handle it. Species shortlists sit on a feed brand's page and are contradicted by owners on social media;
handling rules are duplicated across three sites; zoonosis warnings sit on a government page; cost and local
availability appear nowhere although members ask for price repeatedly.


**2. Representative searches (measured; most-corroborated task-intent members first).** `best pet birds for beginners`; `best pet birds for beginners in india`; `pet bird types list`; `how to pet bird`; `best pet birds for kids`; `which pet bird is best for home`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 9 of 42 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 42 member queries, 26 with task intent (0.62 share), 3 of them volunteered by the engines on bare probes; intent mix decide 13, informational 13, calculate 4, find 4; subdomains touched: birds_small_pets. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_40520272` with 41 members (40 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** No fetched page covers more than one slice. Species shortlisting sits on kaytee.com (six named species) and, contested, in the Instagram comment thread ('Conures ARE NOT for Beginners'). Handling technique sits on petmd.com and chewy.com, which agree on the same rule ('the only areas you should pet your bird' - head and neck) and on zupreem.com as a five-step procedure. Health and zoonosis risk sits on cdc.gov. Supplies sit on the two brand sites, attached to their own products. Cost, local availability, and legality appear on none of the pages fetched, although the cluster's member queries repeatedly ask for price ('pet bird cost', 'pet birds price in india') and the input records four 'calculate' intents.

**6. Major competitors (8 reviewed, 6 pages fetched, 2 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| cdc.gov | official | ok | Government public-health information; no pricing, subscription, ads or affiliate | "April 15, 2024" | no | no |
| kaytee.com | unclassified | ok | Brand content for a bird-products manufacturer; the page recommends its own bran | unknown - no published or updated date s | no | no |
| chewy.com | unclassified | ok | Retailer content marketing with in-page commerce - product cards with ratings an | "Updated Jun. 3, 2025" | no | no (not required to read; an account is needed for the store functions) |
| petmd.com | unclassified | ok | Ad/affiliate-supported publisher content with retailer product links (bird treat | "Updated Mar. 3, 2025" and "Reviewed by  | no | no |
| zupreem.com | unclassified | ok | Brand content for a bird-food manufacturer promoting its own product categories; | Published "August 30, 2023"; Modified "F | no | no |
| instagram.com | unclassified | ok | Social platform; the fetched content shows a brand comment promoting a product ( | "July 16, 2025" | unknown (app install not required to view the web page, but a login is prompted) | yes - "Log in to like or comment." and "Sign up for Instagram to stay in the loop" |
| fishlore.com | community | failed: HTTP | unknown | unknown | unknown | unknown |
| cdfa.ca.gov | official | failed: PDF  | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** A species-matching decision tool that takes the household's constraints (noise tolerance, hours away, children, space, budget, country) and returns a ranked species shortlist with a total cost of ownership - purchase price, cage, food, and avian vet - plus a first-90-days handling and taming plan and a local avian-vet and rescue lookup. It would have to reconcile the disagreement the Instagram thread shows between marketing shortlists and owner experience, and cover the non-US demand visible in the member queries. Existing tools: None found in the checked results - the input file records tool_hits 0 and vendor_share 0.0, and every page fetched was an article, a brand care page, a government advisory or a social post; no quiz, selector, cost calculator or database was found. Member queries explicitly ask for artefacts that do not appear ('pet bird checklist', 'pet bird template', 'random pet bird generator').

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.39, tool hits 0, vendor share 0.0, unclassified 0.44, dominant site kaytee.com (0.11); classes: unclassified 8, editorial 5, community 2, official 2, marketplace 1. Queries checked: best pet birds for beginners; how to pet bird.

**9. Proposed free resource.** A species-matching selector: household constraints (noise, hours away, children, space, budget, country) produce a
ranked shortlist with total cost of ownership, a first-90-days handling plan, and a local avian-vet and rescue
lookup; reconciling brand shortlists with owner experience.


**10. Why users would choose it.** A decision with a cost attached instead of a brand's six-species list. Inference.


**11. Indexable useful pages / tools.** Per species x constraint pages, per-country cost pages, per-vet locality pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** Low after the decision; care-plan pages recur. Inference.


**13. Advertising suitability.** Pet audience with ongoing supply purchases; moderate CPC; brand sponsorship. Inference.


**14. Other monetization.** Supply affiliates, vet-finder leads, insurance affiliates. Hypothesis.


**15. Maintenance requirements.** Species table stable; prices and local availability need first-party collection.
 Data source and reproducibility (from the competitor record): inference: highly reproducible. The core dataset is a species table - size, lifespan, noise, talking ability, handling tolerance, space and cost - and the fetched pages already expose fragments of it (kaytee.com states 'Finches have a short 5-year lifespan'). Avian-vet and zoonosis guidance is public-domain government material (cdc.gov). The two genuinely non-reproducible assets are the named-DVM review on petmd.com and the owner community whose dissent appears in the Instagram comments; price and local availability would require ongoing first-party collection, and no fetched page supplies them.

**16. AI threat.** Informational half fully replaceable; live local data, trust signatures and community dissent resist. Inference.
 Competitor-record view: inference: low for the informational half, higher for the situated half. 'Where do I pet a bird' and 'which species suits a beginner' are stable, low-stakes explainers that a chat assistant answers at least as well as any page fetched, and better where the user's constraints matter. What survives is anything requiring live local data (breeder and rescue availability, current prices, an avian vet nearby), anything requiring trust signatures (vet-reviewed, CDC), and the community dissent that tells a buyer which published shortlist is wrong.

**17. Search-engine direct-answer threat.** High for "best pet bird for beginners". Inference.


**18. Primary risk.** Low-stakes explainer category where engines and assistants answer the entry queries; the resilient data (prices,
local availability) is expensive to collect.


**19. Evidence quality / confidence.** Fragmentation 0.39 measured; 6 of 8 pages fetched; demand weak (42-member cluster, members mix US and Indian markets).
 Open questions from the competitor record: The CDFA PDF is one of the two official results and could not be parsed; whether it contains a structured care or disease reference is unknown. | The Fishlore thread returned 403 twice, so the community half of this SERP - which the Instagram comments suggest disagrees with the brand pages - is only partially evidenced. | Many member queries are India-specific ('pet birds price in india', 'best pet birds for home in india') but the checked results are US sites; whether a separate competitor set serves that market is unknown, since the evidence basis is a US-only search tool. | Cost appears in the member queries but on none of the pages fetched; where users currently get price data is unknown.

**20. Verdict.** Top 20 as an example of the selector-plus-cost pattern; not a finalist.



## 18. roof leaking  (`c_56316349`, domain home_repair)

**1. Problem.** A homeowner has a leak and needs to locate it, contain it, repair it by roof type, and know the cost. Diagnosis
appears in passing on two contractor blogs, emergency containment on two near-duplicate pages, permanent repair
split by roof type across vendor and contractor pages, cost on one page as an unsourced single-market range; every
commercial page ends at one metro's phone number.


**2. Representative searches (measured; most-corroborated task-intent members first).** `roof leaking fix`; `roof leaking repair cost`; `how to repair leaking roof`; `how to repair a leaking metal roof`; `how much does it cost to repair leaking roof`; `how to stop a metal roof from leaking`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term 'roof leaking' at 4.7 on the chained Trends scale (root = 100, rounding error ±2.8%); 16 of 77 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 77 member queries, 36 with task intent (0.47 share), 5 of them volunteered by the engines on bare probes; intent mix informational 41, how_to 14, fix 11, calculate 4; subdomains touched: roof_exterior. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_28899711` with 11 members (11 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The cluster's answer is split across three layers, each on different sites. Diagnosis - that water enters far from where it drips - appears only in passing on goldengrouproofing.com and reconroof.com. Emergency containment from inside is on harbertroofing.com and goldshieldexteriors.com, which are near-duplicates of each other and both state their own fixes are temporary. Permanent repair is split by roof type: metal-roof fastener failure is on apmhexseal.com (vendor framing) and 12stonesroofing.com (five methods plus cost ranges), while shingle-roof permanent repair is largely absent from the pages fetched. Peer experience with conflicting remedies is on garagejournal.com, but that thread is from 2010-2019 and ends unresolved. Cost - one of the cluster's four calculate-intent members and several cost-related member queries - appears on exactly one fetched page (12stonesroofing.com), as a broad range tied to one Texas market. Every commercial page ends at a phone number in a single metro area.

**6. Major competitors (8 reviewed, 7 pages fetched, 1 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| garagejournal.com | community | ok | Advertising; the page shows banner ads and invites registration to remove them,  | Thread started August 23, 2010; most rec | no | no to read; an account is required to post replies, and registration is promoted as the way to avoid ads. |
| goldengrouproofing.com | editorial | ok | Lead generation for the company's own roofing repair and replacement work, with  | unknown - no published or updated date s | no | no |
| apmhexseal.com | editorial | ok | Manufacturer content marketing for its own sealing hardware (Seelskrews, Seelbol | Published February 16, 2022 per the page | no | unknown - not stated on the page. |
| reconroof.com | editorial | ok | Lead generation for Recon Roofing & Construction's services, with a contact form | Published April 28, 2025; last modified  | no | unknown - not stated on the page. |
| harbertroofing.com | editorial | ok | Lead generation for Harbert Roofing's repair services, with a phone call-to-acti | Published February 20, 2026 per the page | no | no |
| goldshieldexteriors.com | editorial | ok | Lead generation with multiple calls-to-action for free quotes, emergency service | Published November 3, 2025 per the page. | no | unknown - not stated on the page. |
| 12stonesroofing.com | editorial | ok | Lead generation for 12 Stones Roofing & Construction with a free-estimate call-t | unknown - no published or updated date s | no | no |
| smi.engin.umich.edu | editorial | failed: DNS  | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** One destination would have to (a) locate the leak - take the interior drip position, roof type, pitch and recent weather and narrow the roof-side entry point, which is the step every fetched page names as the hard part and none performs; (b) branch by roof type, since the metal-roof and shingle-roof answers in these results do not overlap; (c) separate the emergency containment decision from the permanent repair decision explicitly, rather than selling one as the other; (d) produce a local cost estimate and a material quantity, which only one fetched page approaches and only as a national-sounding range from one market; and (e) hand off to a contractor the user can actually hire, since each commercial page serves one metro (Boston, Des Moines, Redding, Clark County WA, Pasadena TX). No fetched page does more than two of these. Existing tools: none found in the checked results - the cluster's fragmentation record reports tool_hits 0, and all eight selected results were forum threads or articles; no calculator, estimator, database or leak-locating tool appeared, despite four calculate-intent members and member queries such as "roof leaking repair cost" and "how much are roof leaks".

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.56, tool hits 0, vendor share 0.0, unclassified 0.33, dominant site smi.engin.umich.edu (0.17); classes: editorial 9, unclassified 6, marketplace 2, community 1. Queries checked: how to stop a metal roof from leaking; roof leaking fix.

**9. Proposed free resource.** A leak locator and repair planner: interior drip position, roof type, pitch and recent weather narrow the entry
point; explicit emergency-versus-permanent branching; material quantity and a local cost estimate; hand-off to
contractors by service area.


**10. Why users would choose it.** Diagnosis and a number instead of seven cause lists and a phone number. Inference.


**11. Indexable useful pages / tools.** Roof type x symptom x region pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** Low. Inference.


**13. Advertising suitability.** Very high CPC (roofing leads), which is also why the SERP is contractor content. Inference.


**14. Other monetization.** Contractor leads, material affiliates. Hypothesis.


**15. Maintenance requirements.** Advice static; cost data must be assembled from quotes, not scraped.
 Data source and reproducibility (from the competitor record): inference: there is no proprietary dataset in this cluster. The repair knowledge is commodity trade knowledge reproduced nearly verbatim across sites (the two 'from the inside' pages are structurally interchangeable). The only candidate data assets are (1) repair cost by roof type, damage type and geography - present on exactly one fetched page as an unsourced single-market range, and rebuildable only by collecting contractor quotes rather than from any public source; and (2) contractor coverage by service area, which the fetched pages expose one company at a time. Both would have to be assembled, not scraped.

**16. AI threat.** High for the advice layer; local pricing and a hireable contractor resist. Inference.
 Competitor-record view: inference: low for the article layer and moderate for two things. The cause lists, emergency steps and when-to-call-a-pro criteria that make up seven of the eight selected results are exactly what a chat assistant answers, and an assistant can condition on the user's roof type and symptom where these static pages cannot. What resists substitution: the garagejournal.com thread's adversarial first-hand disagreement about what actually held over years, and current local pricing plus a hireable local contractor - the endpoint every commercial page in this cluster is built to capture.

**17. Search-engine direct-answer threat.** High. Inference.


**18. Primary risk.** Lead-gen incumbents (HomeAdvisor, Angi) dominate the monetisable step and the differentiating cost data is not
publicly available; the dominant checked site (3 of 18 results) could not be fetched.


**19. Evidence quality / confidence.** Fragmentation 0.56 measured, no tool hits; 7 of 8 pages fetched; demand weak (77 members, head term at 4.7 on the chain).
 Open questions from the competitor record: smi.engin.umich.edu holds 3 of 18 checked results - the largest single-site share in this cluster - and did not resolve on two attempts, so the most SERP-dominant site here is entirely unevidenced. A University of Michigan engineering subdomain ranking for consumer roof-repair queries is itself unexplained and worth verifying. | The selection rule (tool/vendor/official first, then highest-ranked community and editorial) excluded lowes.com, which holds 2 of 18 checked results as a marketplace and ranks 2nd and 3rd on the two checked queries, and excluded statefarm.com and servpro.com. The insurance and restoration angle on roof leaks is therefore unexamined in this pass. | Cost is a stated intent in this cluster (4 calculate-intent members; members include "roof leaking repair cost" and "roof leak repair cost"), but only one fetched page states any figures and only for metal roofs in one Texas market. Whether a defensible national cost dataset can be assembled is unknown. | Flat roofs are prominent in the cluster label and members ("flat roof leaking at edge", "how to fix a leaking flat roof", "flat roof leaking in winter") but the two checked queries were metal-roof and generic, so no fetched page addresses flat roofs directly; flat-roof competition is unmeasured.

**20. Verdict.** Top 20 as a high-CPC example; not a finalist because the product would be another article set without the cost data.



## 19. houdini render setting  (`c_96742201`, domain three_d_animation)

**1. Problem.** A Houdini user needs the render settings and node paths for their renderer and version, and fixes for black
frames, missing FBX skeletons and single-frame sequences. The vendor doc is pinned to one version and one
renderer; renderer-specific docs returned dead or unrendered pages; speed tuning sits on a cloud vendor's blog;
five of twenty checked results are videos; the only tools are paid one-off assets.


**2. Representative searches (measured; most-corroborated task-intent members first).** `how to render in houdini`; `import fbx into houdini`; `houdini fbx character import`; `houdini import fbx animation`; `houdini karma render tutorial`; `how to render in houdini karma`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 5 of 26 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 26 member queries, 21 with task intent (0.81 share), 5 of them volunteered by the engines on bare probes; intent mix how_to 18, informational 5, convert 3; subdomains touched: vfx_sim. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_56233557` with 24 members (23 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The pieces sit on different sites and in different formats. Conceptual setup - camera, lights, render node, IPR preview, 'If you don't have lights in the scene the render will come out black.' - is on sidefx.com, pinned to 'Houdini 22.0' and written for Mantra. Speed tuning with actual menu paths ('Rendering > Mantra > Sampling steps', 'Set Your Cache Limit') is on vagon.io, last updated December 2023 and ending in a pitch for cloud machines. Renderer-specific settings for Arnold and RenderMan are supposedly on help.autodesk.com and rmanwiki-26.pixar.com, but the first returned a 404 and the second returned an unrendered index. The demonstrations users evidently prefer are on YouTube - five of the twenty checked results - and could not be read. Paid one-off import assets sit on three separate Gumroad seller subdomains. The one community thread that loaded answers a different question than it ranked for. A user therefore assembles concept from the docs, current settings from a blog, and the actual click path from video.

**6. Major competitors (8 reviewed, 4 pages fetched, 4 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| sidefx.com | vendor | ok | Vendor documentation supporting a paid 3D application; no pricing, subscription, | Versioned rather than dated - the page i | True | unknown |
| help.autodesk.com | vendor | failed: the  | unknown | unknown | unknown | unknown |
| vagon.io | editorial | ok | Content marketing for the site's own paid cloud-rendering service, with account- | Published on April 25, 2021 and Updated  | unknown | unknown |
| youtube.com | community | failed: only | unknown | unknown | unknown | unknown |
| rmanwiki-26.pixar.com | unclassified | ok | unknown - no pricing, subscription, ads or affiliate links appeared on the page. | unknown - no last-updated date or versio | unknown | unknown |
| julianabreu.gumroad.com | marketplace | failed: HTTP | unknown from the page; inference: a Gumroad product page is a paid or pay-what-y | unknown | unknown | unknown |
| doduyanh.gumroad.com | marketplace | failed: only | unknown from the page; inference: a Gumroad listing is a paid or name-your-price | unknown | unknown | unknown |
| steamcommunity.com | community | ok | unknown - no pricing, subscription, ads or affiliate links appeared on the page; | Dated posts from 2019 - '4 Δεκ 2019, 9:1 | True | True |

**7. Why competitors do not completely solve it.** A goal-indexed, version-aware reference that starts from what the user is trying to produce - a still, a sequence, a fire sim, hair, an imported FBX character with its animation intact - and gives the node path, the parameter values and the current-version screenshot for the renderer they actually use (Karma and Karma XPU first, since that is what the cluster asks for, then Arnold, Redshift, RenderMan), together with a diagnostic path for the common failure states (black frame, missing skeleton on FBX import, sequence rendering only one frame). It would also have to state which Houdini version each instruction was verified against, because the vendor docs are version-pinned and the blog advice is not dated to a version at all. Existing tools: None found in the checked results. There is no calculator, database or interactive settings tool anywhere in the twenty checked results - the input file records tool_hits of 0. The closest things to tooling are paid Houdini Digital Assets sold on Gumroad seller subdomains (an Alembic quick-import HDA, and two further Gumroad listings not fetched) and Vagon's cloud-rendering service, which changes where the render runs rather than what the settings should be.

**8. Fragmentation evidence (measured on 20 web-search proxy results for 2 queries).** Fragmented share 0.35, tool hits 0, vendor share 0.4, unclassified 0.05, dominant site sidefx.com (0.35); classes: vendor 8, community 6, marketplace 4, editorial 1, unclassified 1. Queries checked: how to render in houdini; import fbx into houdini.

**9. Proposed free resource.** A goal-indexed, version-stamped reference: what you are producing, which renderer, get node paths, parameter
values and current screenshots, plus a failure-state diagnostic, each entry naming the version it was verified on.


**10. Why users would choose it.** The parameter in the version they have, on screen, instead of a video and a stale doc. Inference.


**11. Indexable useful pages / tools.** Goal x renderer x version pages: hundreds to low thousands. Hypothesis.


**12. Repeat-use mechanism.** Moderate for professionals. Inference.


**13. Advertising suitability.** Small professional audience; sponsorship over display. Inference.


**14. Other monetization.** Asset sales, training referrals, cloud-render affiliates. Hypothesis.


**15. Maintenance requirements.** High: content is re-derived by running proprietary software per version.
 Data source and reproducibility (from the competitor record): The underlying facts are the behaviour of proprietary software - Houdini and its render engines - so nothing here can be rebuilt from an open dataset; it can only be re-derived by running the software and recording what each parameter does, version by version. That makes the content expensive to produce but also means the vendor is not the only possible source: the docs page carries no date, the third-party pages carry dates but no version label, and the version-to-behaviour mapping that users actually need is not published as data anywhere in the checked results.

**16. AI threat.** Moderate: concepts replaceable, version-specific paths and screenshots less so. Inference.
 Competitor-record view: Moderate. A chat assistant handles the conceptual questions - what a render node is, why a frame is black, what sampling trades against - at least as well as these pages. It is weaker exactly where this cluster hurts: naming the parameter in the version the user has installed, and showing where it is on screen, which is why five of twenty checked results are video. An assistant also cannot deliver a working HDA. A destination built on verified, version-stamped parameter paths plus failure diagnosis is resilient; one built on explaining rendering concepts is not.

**17. Search-engine direct-answer threat.** Low. Inference.


**18. Primary risk.** Niche audience and expensive content production; 4 of 8 pages failed so the competitor picture is half-evidenced.


**19. Evidence quality / confidence.** Fragmentation 0.35 measured; 4 of 8 pages failed; demand weak (26-member cluster).
 Open questions from the competitor record: Whether the Karma and Karma XPU settings the cluster keeps asking about are documented anywhere that ranks - the top-ranked vendor page fetched covers Mantra, and the one Karma video could not be read. | What is actually inside the three Gumroad listings (two of which failed to load) and whether paid assets are substituting for missing free documentation on FBX and Alembic import. | Why a 404 page holds rank 2 for 'how to render in houdini' - whether the Autodesk documentation moved or the search proxy's index is stale. | How often Houdini version changes invalidate published instructions, which determines whether a version-stamped reference could be maintained at reasonable cost.

**20. Verdict.** Top 20 as the documentation-gap archetype for professional software; not a finalist for audience size.



## 20. zapier excel integration  (`c_01459513`, domain productivity_office)

**1. Problem.** A user wants to know whether app X connects to app Y through an automation platform, on which plan, with which
triggers, and what to do when no connector exists. Partial hand-picked lists sit on vendor blogs; the build path
sits in the platform's developer docs; per-vendor walkthroughs are tied to one product; the head term's Excel
half was answered by no fetched page.


**2. Representative searches (measured; most-corroborated task-intent members first).** `list of zapier integrations`; `zapier api integration tutorial`; `how to create a zapier integration`; `zapier api integration alternatives`; `zapier api integration best practices`; `best office software zapier integration`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 2 of 22 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 22 member queries, 12 with task intent (0.55 share), 5 of them volunteered by the engines on bare probes; intent mix informational 9, decide 2, generate 2, how_to 2; subdomains touched: automation. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_51119440` with 24 members (23 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The cluster's answer is split three ways and no fetched page holds more than one part. (a) What integrations exist - only partial, hand-picked lists were retrievable (livesession.io: 11 apps in 6 categories; cloudtalk.io: 10 apps), both on vendor blogs that lead with their own product; the authoritative catalogue lives on zapier.com/apps, which ranked 5th-7th on 'list of zapier integrations' in the input file but fell outside the selection rule and was not fetched. (b) How to build an integration against your own API - docs.zapier.com gives the official concept-by-concept path (auth, triggers, actions, testing, versions, public vs private). (c) What it actually looks like against a real API - only vendor-specific walkthroughs (help.dealmaker.tech, commonroom.io), each tied to one product's tokens and endpoints. The head term 'zapier excel integration' was not answered by any fetched page: no page fetched named Excel or Office 365.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| help.followupboss.com | vendor | failed: HTTP | unknown | unknown | unknown | unknown |
| docs.zapier.com | vendor | ok | Vendor documentation for Zapier's own developer platform; no pricing, subscripti | no published or last-updated date shown  | False | True |
| help.dealmaker.tech | vendor | ok | Vendor help content supporting a paid product; no pricing, ads or affiliate link | no published or last-updated date shown  | False | True |
| commonroom.io | vendor | ok | Vendor documentation; page states the integration is 'available on all plans' bu | page shows 'Last updated Sep 16th, 2026' | False | True |
| help.letsdeel.com | vendor | failed: HTTP | unknown | unknown | unknown | unknown |
| block.fiverr.com | tool | failed: 301  | unknown | unknown | unknown | unknown |
| livesession.io | editorial | ok | Vendor-owned blog: the page lists its own product among the picks, quotes subscr | page shows 'Published April 29, 2025 - U | False | False |
| cloudtalk.io | editorial | ok | Vendor-owned blog leading with its own product; quotes per-seat subscription pri | page shows 'Santiago Montaldo Updated on | False | False |

**7. Why competitors do not completely solve it.** A searchable, current app-pair index (does app X connect to app Y, on which trigger/action, on which Zapier plan, and what breaks) joined to a per-pair setup walkthrough and to a build path for apps with no existing connector. Concretely it would need: live coverage of the catalogue the pages size at 7,000-9,000 apps rather than a ten-item editorial pick; per-pair trigger/action detail; plan and cost implications, since commonroom.io warns the Zapier route is 'a costly experience'; and an escape hatch to the developer-platform route when no connector exists. Existing tools: None found among the fetched pages. The only non-article result selected was a Fiverr gig listing (fetch failed), which sells human labour rather than an answer. Zapier's own app directory (zapier.com/apps) appears in the input file's SERP but was outside the selection rule and unverified here.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.56, tool hits 1, vendor share 0.28, unclassified 0.11, dominant site zapier.com (0.22); classes: editorial 9, vendor 5, unclassified 2, tool 1, community 1. Queries checked: list of zapier integrations; zapier api integration tutorial.

**9. Proposed free resource.** A searchable app-pair index (does X connect to Y, trigger and action detail, plan and cost implications, known
breakages) joined to per-pair setup walkthroughs and an escape hatch to the developer route.


**10. Why users would choose it.** Live pair-level facts instead of ten-item editorial picks. Inference.


**11. Indexable useful pages / tools.** Pair pages across a catalogue the fetched pages size at 7,000 to 9,000 apps: tens of thousands. Hypothesis.


**12. Repeat-use mechanism.** Moderate (each new automation). Inference.


**13. Advertising suitability.** B2B software audience with high CPC. Inference.


**14. Other monetization.** Platform affiliates, consultant leads. Hypothesis.


**15. Maintenance requirements.** Catalogue changes continuously; vendor help centres blocked automated fetching (3 of 8 pages).
 Data source and reproducibility (from the competitor record): inference: the connector catalogue itself is public and machine-readable in principle (Zapier's public app directory plus each vendor's own integration docs, e.g. the Common Room and DealMaker pages fetched here), so a third-party index of app pairs and their triggers/actions looks reproducible; what is not reproducible is anything behind a Zapier account, and the editorial rankings are opinion rather than data. Practical obstacle observed in this task: 3 of 8 selected pages (help.followupboss.com, help.letsdeel.com, fiverr.com) returned HTTP 403 to an automated fetch, so bulk scraping of vendor help centres is likely to be blocked.

**16. AI threat.** Conceptual half replaceable; live pair support on a given plan resists. Inference.
 Competitor-record view: inference: low for the conceptual half of the cluster - 'what is zapier integration', 'how does a Zap work' and 'name some popular integrations' are answered as well by a chat assistant as by the editorial pages fetched. Resilience is higher for the volatile half: whether a specific app pair is supported today, on which plan, and with which triggers, is a live fact that an assistant would answer from stale memory, and it is exactly the half none of the fetched pages covers.

**17. Search-engine direct-answer threat.** Moderate. Inference.


**18. Primary risk.** The platform's own app directory (ranked 5th to 7th on the list query, not fetched) almost certainly already
serves the pair question; this is a vendor-owned catalogue.


**19. Evidence quality / confidence.** Fragmentation 0.56 measured; 5 of 8 pages fetched; the checked queries did not cover the Excel half; demand weak.
 Open questions from the competitor record: zapier.com's own pages (developer-platform, homepage, /apps) held ranks 5, 6 and 7 on 'list of zapier integrations' and rank 6 on the tutorial query - 4 of 18 results - but were categorised editorial in the input file and fell outside the selection rule; whether zapier.com/apps already is the single destination is unverified | the head term is 'zapier excel integration' yet neither checked query mentions Excel or Office, and no fetched page named Excel; whether the Excel/Office 365 side of the cluster is equally fragmented is untested | three vendor pages returned 403, so their monetisation, currency and coverage are unknown; a browser-based check could confirm whether they are richer than the SERP titles suggest | commonroom.io states the Zapier route is 'a costly experience' - whether per-task Zapier pricing is a recurring complaint across the cluster is unknown from the fetched pages

**20. Verdict.** Last of the Top 20: real fragmentation in editorial, but the vendor's directory is the likely complete answer.



## 21. does notion work with microsoft  (`c_10039327`, domain productivity_office)

**1. Problem.** Whether a note-taking app works inside a Microsoft 365 tenant (sign-in, SSO, connectors, plans) and what
Microsoft's own equivalent is. The compatibility half is answered only by first-party admin documentation on paid
tiers; the substitution half only by vendor-owned listicles; no page serves both, and the directory sites failed to load.


**2. Representative searches (measured; most-corroborated task-intent members first).** `does notion work with microsoft`; `microsoft app similar to notion`; `software similar to notion`; `microsoft alternative to notion`; `what is microsoft loop vs notion`; `what microsoft app is like notion`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 3 of 31 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 31 member queries, 19 with task intent (0.61 share), 10 of them volunteered by the engines on bare probes; intent mix alternative 15, informational 12, compatibility 2, how_to 1; subdomains touched: notes_pkm. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_82393641` with 9 members (10 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** This cluster contains two questions that the checked results answer on entirely different sites. The compatibility question ('does notion work with microsoft') is answered only by first-party pages: notion.com for the Windows app, Microsoft-account sign-up and Teams/OneDrive/SharePoint connectors, and learn.microsoft.com for Entra ID SAML SSO and provisioning - an admin-level answer requiring paid tiers on both sides. The substitution question ('microsoft app similar to notion', 15 of 31 cluster members) is answered only by third-party editorial: zapier.com names Microsoft Loop for Microsoft 365 users, clickup.com is the only fetched page covering both OneNote and Loop with features and pricing, and ones.com asks the exact question but omits Loop from its assessment and concludes with a non-answer. The two directory-style sites that might have merged both views (g2.com, alternativeto.net) both refused automated fetches, and four of the eighteen results are g2.com alternatives pages for note apps other than Notion.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| notion.com | vendor | ok | Vendor marketing post for a freemium SaaS; no prices are stated on the page, whi | page shows 'Published May 27, 2025 in No | True | True |
| apps.microsoft.com | vendor | failed: near | unknown | unknown | True | unknown |
| learn.microsoft.com | vendor | ok | First-party vendor documentation; no ads or affiliate links, but it presupposes  | page metadata shows 'ms.date: 2025-03-25 | False | True |
| g2.com | tool | failed: HTTP | unknown | unknown | unknown | unknown |
| alternativeto.net | tool | failed: HTTP | unknown | unknown | unknown | unknown |
| zapier.com | editorial | ok | Editorial content on an automation vendor's blog; quotes per-seat and per-month  | page states 'This article was originally | False | False |
| clickup.com | editorial | ok | Vendor-owned blog with repeated calls to action to its own signup ('Try the #1 N | page shows 'Jun 08, 2025' with the title | False | False |
| ones.com | editorial | ok | Vendor-owned blog; no prices quoted, but repeated signup calls to action ('Try O | page shows 'Published: February 27, 2026 | False | False |

**7. Why competitors do not completely solve it.** A single page holding, for a named Microsoft 365 tenant tier, both halves at once: (1) a current interop matrix - Windows app, Microsoft-account sign-in, Entra SSO and SCIM, Teams/OneDrive/SharePoint connectors, and which Notion plan each requires; and (2) a like-for-like Loop vs OneNote vs Lists vs Notion capability comparison including what Microsoft's tools cannot do, with migration implications. No fetched page carried both, and no fetched page carried the two halves for the same reader persona - the compatibility pages address tenant admins while the comparison pages address individual users. Existing tools: None verified. Three results categorised as tools in the input file - g2.com (4 of 18 results), alternativeto.net and the Microsoft Store listing - all failed to return usable content (two 403s and one near-empty JavaScript page), so whether any of them answers the Microsoft-specific question is unconfirmed. Among pages that did load, all five were articles or documentation; none was interactive.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.28, tool hits 5, vendor share 0.33, unclassified 0.11, dominant site g2.com (0.22); classes: vendor 6, editorial 5, tool 5, unclassified 2. Queries checked: does notion work with microsoft; microsoft app similar to notion.

**9. Proposed free resource.** An interop matrix per tenant tier plus a like-for-like Loop, OneNote, Lists and Notion comparison with migration notes.


**10. Why users would choose it.** Both halves for the same reader. Inference.


**11. Indexable useful pages / tools.** Per app-pair and per feature comparison pages: hundreds. Hypothesis.


**12. Repeat-use mechanism.** Low. Inference.


**13. Advertising suitability.** B2B software, high CPC. Inference.


**14. Other monetization.** Software affiliates. Hypothesis.


**15. Maintenance requirements.** Moderate; first-party sources carry update metadata.
 Data source and reproducibility (from the competitor record): inference: the factual core is highly reproducible from public first-party sources - Microsoft's Entra SaaS-app gallery documentation (which even exposes its own ms.date and updated_at metadata and links its source on GitHub), Notion's own help and connections pages, and both vendors' public pricing pages. What is not reproducible is the review and voting data behind g2.com and alternativeto.net. Practical obstacle: 3 of 8 selected pages could not be fetched automatically, and the Microsoft Store listing rendered empty without JavaScript.

**16. AI threat.** High: the substitution half is a one-sentence answer. Inference.
 Competitor-record view: inference: low for the substitution half - 'what is Microsoft's version of Notion' is a single-sentence answer (Loop, with OneNote and Lists adjacent) that a chat assistant gives better than a vendor-owned listicle, and the cluster's own member queries are phrased as direct questions. Resilience is higher for the compatibility half: which connectors exist, on which plan, and the exact SAML attributes and URL patterns are fast-moving and must be exact, and Microsoft's own tutorial showed an update timestamp of 2026-06-15 - an assistant answering from memory would misstate these.

**17. Search-engine direct-answer threat.** High. Inference.


**18. Primary risk.** Vendor-owned answers and review directories (G2, AlternativeTo, both blocked) already cover it.


**19. Evidence quality / confidence.** Fragmentation 0.28 measured with 5 tool hits; 3 of 8 pages failed; demand weak.
 Open questions from the competitor record: g2.com supplies 4 of 18 results but blocked both fetch attempts; whether its alternatives pages actually surface Microsoft products, and how it monetises sponsored placement, is unknown | christine-payton.com appeared on both checked queries (2 of 18) with a title promising exactly the missing comparison - Notion vs Microsoft Loop and Lists - but fell outside the 8-site selection rule and was not fetched | notion.com holds 3 of 18 results; only its blog post was fetched, so the connections/onedrive and Teams AI-connector pages may already answer more of the compatibility question than recorded here | no fetched page stated whether Microsoft Loop is included in a standard Microsoft 365 subscription, which is the pivotal cost fact for the 'microsoft alternative to notion free' member queries

**20. Verdict.** Reviewed, not advanced: vendor-owned and AI-exposed.



## 22. door won't close  (`c_43126978`, domain home_repair)

**1. Problem.** A door does not close or latch. The head term is ambiguous across mechanical repair, fire-door and self-closing
legal obligations, appliance doors and a productivity metaphor; the repair content is a contractor's cause list;
four of eight pages failed to fetch.


**2. Representative searches (measured; most-corroborated task-intent members first).** `door won't close`; `door won't close all the way`; `door won't close at top`; `should i close all doors`; `door won't close how to fix`; `how to fix door that won't close all the way`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 4 of 21 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 21 member queries, 18 with task intent (0.86 share), 5 of them volunteered by the engines on bare probes; intent mix fix 16, informational 3, how_to 1, decide 1; subdomains touched: doors_windows. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_02995314` with 21 members (23 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** No single fetched page answers the cluster. The physical diagnosis-and-fix content lives on a commercial door installer's blog (doorslosangeles.com: an 11-step cause list from hinges to strike plates to warped slabs). The legal/landlord angle - who must fix a door that does not self-close and latch, and how to escalate - lives on nyc.gov and is valid only in New York City. The fire-safety framing sits behind a Canterbury City Council PDF that did not render. Peer experience sits on Bunnings Workshop, Quora and Facebook, none of which returned usable answers in this pass; the one Facebook post that did return shows a question with no answer. Separately, the cluster's second checked query, "should i close all doors", pulls in an entirely different meaning - a productivity SaaS essay on facilethings.com - showing the head term is ambiguous across at least three unrelated intents (mechanical repair, fire/building code, life advice).

**6. Major competitors (8 reviewed, 4 pages fetched, 4 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| nyc.gov | official | ok | unknown - government information page; the fetched page states no pricing, subsc | unknown - no page-level published or upd | no | no |
| canterbury.gov.uk | official | failed: PDF  | unknown | unknown - the URL path contains "2023-11 | unknown | unknown |
| ushl.samsung.com | vendor | failed: DNS  | unknown | unknown | unknown | unknown |
| workshop.bunnings.com.au | community | failed: page | unknown | unknown | unknown | unknown |
| doorslosangeles.com | editorial | ok | Lead generation for the site's own door sales and installation service; the page | Published 2025-07-11 per the page; readi | no | no |
| facilethings.com | editorial | ok | Paid SaaS promotion; the page repeatedly calls the reader to a free trial of the | unknown - no publication or update date  | no for the article; yes an account for the promoted product - the page promotes registering for a 30-day free trial. | no to read the article; the promoted product requires registration. |
| quora.com | community | failed: HTTP | unknown | unknown | unknown | unknown |
| facebook.com | community | ok | unknown - no pricing, subscription or affiliate links were visible in the retrie | unknown - no date visible in the retriev | no | unknown - the post text was visible without signing in, but no statement about account requirements appeared. |

**7. Why competitors do not completely solve it.** A symptom-driven diagnostic that takes what the user observes (rubs at the top, does not latch, swings open on its own, new pre-hung door, seasonal) and the door context (interior/exterior, hinge type, hollow or solid, rental or owned) and returns one ranked cause, the specific adjustment, the exact parts and tools, and a decision point for when the frame is racked or the slab is warped and a professional or replacement is the answer. It would also have to resolve the ambiguity in the head term up front (repair vs. fire-door/self-closing obligation vs. should-I-close-interior-doors), and for renters route to the local obligation and complaint path rather than to a DIY fix. None of the fetched pages does more than one of these. Existing tools: none found in the checked results - the cluster's fragmentation record reports tool_hits 0, and every page fetched was an article, an official information page, a PDF or a social/forum post; no calculator, database or diagnostic tool was found.

**8. Fragmentation evidence (measured on 19 web-search proxy results for 2 queries).** Fragmented share 0.58, tool hits 0, vendor share 0.05, unclassified 0.05, dominant site nyc.gov (0.21); classes: community 7, official 5, editorial 4, unclassified 1, vendor 1, reference 1. Queries checked: door won't close; should i close all doors.

**9. Proposed free resource.** A symptom-driven door diagnostic with renter routing to local obligations.


**10. Why users would choose it.** One ranked cause and adjustment instead of an eleven-step list. Inference.


**11. Indexable useful pages / tools.** Symptom x door type pages, per-jurisdiction obligation pages: hundreds. Hypothesis.


**12. Repeat-use mechanism.** Low. Inference.


**13. Advertising suitability.** Home-improvement audience, moderate CPC. Inference.


**14. Other monetization.** Hardware affiliates, handyman leads. Hypothesis.


**15. Maintenance requirements.** Low; jurisdiction compilation is the only data work.
 Data source and reproducibility (from the competitor record): inference: there is no proprietary dataset anywhere in this cluster. The repair content is generic carpentry knowledge, freely reproducible. The one piece of structured data is municipal law (NYC Housing Maintenance Code section 27-2041.1 on nyc.gov, and whatever the Canterbury fire-door sheet contains), which is public and rebuildable, but is jurisdiction-by-jurisdiction and would have to be compiled per locality - that compilation, not the repair advice, is the only defensible data asset visible here.

**16. AI threat.** Very high: cause lists conditioned on symptoms are exactly what an assistant does. Inference.
 Competitor-record view: inference: low for the advice layer. Every fetched page's substance - cause lists, adjustment steps, when to call a pro - is exactly what a chat assistant answers well, and an assistant can condition on the user's symptoms where a static list cannot. Resilience is higher for two things no assistant supplies: a verified local obligation and complaint path for renters (the nyc.gov role), and the physical work itself, which is where every commercial page funnels the reader.

**17. Search-engine direct-answer threat.** High. Inference.


**18. Primary risk.** No data asset; ambiguous demand.


**19. Evidence quality / confidence.** Fragmentation 0.58 measured but on an ambiguous term; 4 of 8 pages failed; demand weak.
 Open questions from the competitor record: Four of eight selected pages failed to fetch (canterbury.gov.uk PDF unreadable, ushl.samsung.com DNS failure, Bunnings body truncated twice, Quora 403), so the community and official halves of this cluster are under-evidenced; a second pass with a different fetch path is needed before concluding what the forums actually contain. | The head term is ambiguous across at least three intents. Unknown from the fetched pages: what share of the 21 cluster members mean mechanical repair versus fire-code versus the productivity/metaphor sense - the input file's intent counts (fix 16 of 21) suggest repair dominates, but the checked queries included "should i close all doors", which pulled only non-repair results. | Whether a searcher arriving at nyc.gov actually converts to a 311 complaint, and how many jurisdictions publish an equivalent page, is unknown - only NYC and Canterbury appeared in the checked results. | The Samsung refrigerator-door result ranking for a generic "door won't close" query suggests appliance doors are part of this demand; how large that slice is, is unknown from the fetched pages.

**20. Verdict.** Reviewed, not advanced.



## 23. printer offline error  (`c_89685491`, domain it_support)

**1. Problem.** A printer shows offline. The OS layer, the brand layer and the lived-case layer sit on three sites; vendor fixes
delegate to installed utilities; nothing connects the symptom to the reader's configuration.


**2. Representative searches (measured; most-corroborated task-intent members first).** `printer offline error`; `printer offline issue`; `printer offline fix`; `printer offline problem how to fix`; `printer offline problem`; `best offline printer`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term placed at ~0 but with ±101.1% error, so unreliable; 9 of 42 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 42 member queries, 25 with task intent (0.6 share), 4 of them volunteered by the engines on bare probes; intent mix fix 17, informational 14, commercial_nav 3, alternative 3; subdomains touched: printers_drivers. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_58582848` with 42 members (42 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The answer is split by layer, and no fetched page holds more than one layer. The OS layer is on support.microsoft.com, which scopes itself with "Applies To: Windows 11 Windows 10" and hands off the automated part to a local app. The brand layer is on support.hp.com, which adds a device-specific "Diagnose & Fix" utility but ends with "Still need help? Try these solutions". The lived-case layer is the Microsoft Q&A thread, one user's situation plus a generic step list. The Canon equivalent could not be read at all. The hardware-avoidance layer - buy a printer that cannot go offline - lives on entirely different pages (compandsave.com's five non-wireless picks, rtings.com's "183 Printers bought and tested"), and rtings' page is about wireless printers rather than offline-capable ones. Nothing fetched connects the symptom to a specific cause on the reader's own setup.

**6. Major competitors (8 reviewed, 5 pages fetched, 3 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| learn.microsoft.com | vendor | ok | Not stated on the page - no pricing, subscription, ads or affiliate links were f | Post-level timestamps only: question pos | False | False |
| support.hp.com | vendor | ok | Not stated - no pricing, subscription, ads or affiliate links appeared in the fe | No publication or last-updated date show | True | True |
| support.usa.canon.com | vendor | failed: page | unknown | unknown | unknown | unknown |
| support.microsoft.com | vendor | ok | Not stated - no pricing, subscription, ads or affiliate links appeared in the fe | Not stated - no last-updated or publicat | False | False |
| compandsave.com | editorial | ok | Commerce - listed prices such as "Starting from $168.18" and "$429.99", with in- | "Apr 29, 2025" shown near the author byl | False | False |
| dkoms.co.uk | editorial | failed: page | Affiliate - the shell discloses "As an Amazon Associate DKOMS earns commission f | Not stated - the footer shows "© 2026 DK | unknown | unknown |
| rtings.com | editorial | ok | Membership plus affiliate: "Supported by you via membership, and when you purcha | Not stated - no last-updated or publicat | False | unknown |
| justanswer.com | community | failed: HTTP | unknown | unknown | unknown | unknown |

**7. Why competitors do not completely solve it.** One destination would have to (a) identify the reader's actual configuration - OS build, connection type, printer brand and model - rather than assuming it, (b) run or guide the machine-local checks that the vendor pages delegate to installed utilities (spooler state, port and IP, driver, sleep/Wi-Fi behaviour), (c) branch on the answer instead of listing every step, (d) cover brands in one place, since today HP, Canon and Microsoft each hold a separate fragment, and (e) tell the reader when the cause is structural - a printer that keeps dropping off the network - and carry them into the replacement decision that currently sits on unrelated commerce pages. Existing tools: None found in the checked results. The input file records "tool_hits": 0 for this cluster, and no fetched page is a diagnostic tool in its own right; the closest are two vendor-owned local utilities that must be installed or invoked on the user's machine - HP's "Diagnose & Fix" ("Open or download the application on Windows or Mac") and the Windows troubleshooter ("start by running the automated printer troubleshooter in the Get Help app") - plus RTINGS' printer test database, which answers the buying question rather than the error.

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.5, tool hits 0, vendor share 0.22, unclassified 0.06, dominant site ebay.com (0.11); classes: editorial 5, community 4, vendor 4, marketplace 3, reference 1, unclassified 1. Queries checked: best offline printer; printer offline error.

**9. Proposed free resource.** A configuration-branched diagnostic across brands with a replacement decision path.


**10. Why users would choose it.** Branching instead of every step. Inference.


**11. Indexable useful pages / tools.** Brand x OS x symptom pages: hundreds. Hypothesis.


**12. Repeat-use mechanism.** Low to moderate. Inference.


**13. Advertising suitability.** Consumer hardware, moderate CPC. Inference.


**14. Other monetization.** Printer and ink affiliates. Hypothesis.


**15. Maintenance requirements.** Low.
 Data source and reproducibility (from the competitor record): Two different data types. The troubleshooting content is public and reproducible - the same step sequences appear across Microsoft, HP and a Q&A thread, none of which claims proprietary data. The two genuinely hard-to-rebuild assets are the vendor diagnostics, which need privileged local access to spooler, driver and device state, and RTINGS' "183 Printers bought and tested", which represents purchased hardware and a physical test bench. Neither is a scrape target.

**16. AI threat.** Very high for text; the resilient part requires local execution the web cannot do. Inference.
 Competitor-record view: inference: low for the text, higher for anything touching the machine. Every step list fetched here is generic and already within a chat assistant's reach; what an assistant cannot do is read the user's actual spooler, port, IP and driver state, which is precisely where these pages stop and where the unresolved cases live. Durable value sits in local execution and in measured hardware data, not in another written checklist.

**17. Search-engine direct-answer threat.** High. Inference.


**18. Primary risk.** Vendors own the fix and the diagnostics; the durable part needs local software.


**19. Evidence quality / confidence.** Fragmentation 0.50 measured; 3 of 8 pages failed; demand weak.
 Open questions from the competitor record: Canon's article could not be read ("CSS Error"), so whether brand-specific vendor pages go materially beyond the Windows steps is unknown. | JustAnswer returned 403 and holds 2 of 18 checked results; its business model, paywall behaviour and answer quality are unknown from this task. | No fetched page states how often the offline symptom is OS-side versus printer-side versus network-side, so the relative size of each fragment is unknown. | The cluster mixes a fix intent with a buying intent ("best offline printer"); no fetched page serves both, and whether the same users hold both intents is unknown.

**20. Verdict.** Reviewed, not advanced.



## 24. performance review comments  (`c_17253937`, domain careers_jobs)

**1. Problem.** A manager needs finished, rating-consistent review wording grounded in specific evidence. Phrase banks on HR-software
blogs hand personalisation back to the reader; rating-anchored wording is on one university page; the member queries
ask for a generator by name.


**2. Representative searches (measured; most-corroborated task-intent members first).** `performance review comments`; `performance reviews for employees`; `performance review comments for employees`; `performance review examples for employees`; `performance review comments examples`; `good performance review comments`

**3. Search-volume evidence.** None measured on the free route. Relative signals only: head term not yet placed on the Trends chain; 7 of 30 members were suggested by two or more engines.

**4. Aggregate cluster demand (measured, pass 1).** 30 member queries, 30 with task intent (1.0 share), 5 of them volunteered by the engines on bare probes; intent mix optimize 20, generate 5, decide 1, how_to 1; subdomains touched: monitors, workplace. Member counts reflect one seed per subdomain. After pass 2 (two seeds per subdomain, re-clustered) the same problem maps to cluster `c_86051261` with 56 members (32 of the analysed members vote for it; a low vote means the recursive split ladder broke the pass-1 cluster apart, not that demand fell).

**5. Existing search workflow (from fetched pages).** The answer is split three ways and no fetched page holds more than one part. Phrase banks by competency live on HR-software blogs (Quantum Workplace 171, Rippling 88, PerformYard 200+ with role cuts, Factorial 1000+ across ~22 categories, Profit.co 50, BetterUp 31 for problem solving alone). Rating-anchored wording - comments matched to a Poor-to-Outstanding scale - appears only on the university HR page (southeastern.edu), which is also the only fetched page with no commercial motive. Process and preparation guidance, how to run the conversation at all, sits on hr.mit.edu and is tied to one employer's cycle. The personalisation step is on none of them: every fetched page explicitly hands it back to the reader (Southeastern: tailored to the individual employee; Quantum Workplace: inspiration, not a script; PerformYard: what you actually know about this specific person).

**6. Major competitors (8 reviewed, 8 pages fetched, 0 failed).**

| site | kind | fetch | business model | currency | install | account |
|---|---|---|---|---|---|---|
| southeastern.edu | official | ok | inference-free reading of the page: none evident - the fetch found no references | unknown - no publication or last-updated | no | no |
| hr.mit.edu | official | ok | inference-free reading of the page: none evident - no pricing, subscription, ads | unknown - no date is shown on the page. | no | unknown - the page is readable without login, but it points to MIT-internal systems and an MIT web course that would require institutional access. |
| quantumworkplace.com | editorial | ok | Content marketing for the publisher's own performance-review software - the fetc | Page shows an update date near the bylin | no | no for the article itself; downloadable templates may require registration (unknown). |
| factorialhr.com | editorial | ok | Content marketing for the publisher's HR software - the fetch found repeated dem | Page shows a date at the start of the ar | no | no to read; email is solicited for a newsletter. |
| rippling.com | editorial | ok | Content marketing for the publisher's performance management product - multiple  | Page shows an update date at the top. | no | no |
| performyard.com | editorial | ok | Content marketing for the publisher's paid performance-management platform - the | Page shows a last-updated date and a nam | no | no to read the article; the platform behind the calls to action has a log-in. |
| profit.co | editorial | ok | Content marketing for the publisher's performance-management software - free-tri | Page shows a publication/update date. | no | no to read; a free trial is offered and the PDF download may be email-gated (unknown). |
| betterup.com | editorial | ok | Content marketing for the publisher's coaching platform - soft demo calls to act | Page shows a 2022 publication date and,  | no | no |

**7. Why competitors do not completely solve it.** One site would have to take the manager's raw inputs - employee role, rating scale in use, the employer's competency list, goals set last cycle, and a few sentences of observed evidence - and emit finished, rating-consistent review text in the employer's own section structure, with a strengths/improvement balance, a defensible-language check for HR risk, and a self-review variant for employees. In other words the destination has to own the two steps the phrase banks refuse: matching wording to a rating level, and grounding it in specific evidence about one person. Existing tools: none found in the checked results - the input file records tool_hits 0 and vendor_share 0.0 across 18 checked results, and all eight fetched pages are articles or guides. The only generation capability visible is a vendor feature referenced inside an article (PerformYard's AI review assist), not a page a searcher can use; whether it is usable without a paid account is unknown. Note that the cluster's member queries explicitly ask for one (performance review comments generator, ai performance review generator for employees, free ai performance review generator for employees).

**8. Fragmentation evidence (measured on 18 web-search proxy results for 2 queries).** Fragmented share 0.72, tool hits 0, vendor share 0.0, unclassified 0.17, dominant site quantumworkplace.com (0.11); classes: editorial 13, unclassified 3, official 2. Queries checked: performance review comments; performance review problem solving comments.

**9. Proposed free resource.** A review-writing assistant taking role, rating scale, competencies and observed evidence, emitting finished text with
a defensible-language check.


**10. Why users would choose it.** It does the step every page refuses. Inference.


**11. Indexable useful pages / tools.** Competency x rating x role pages: thousands. Hypothesis.


**12. Repeat-use mechanism.** Periodic (review cycles). Inference.


**13. Advertising suitability.** HR-software audience, high CPC; the incumbents are those vendors' content teams. Inference.


**14. Other monetization.** HRIS affiliates, premium templates. Hypothesis.


**15. Maintenance requirements.** Minimal.
 Data source and reproducibility (from the competitor record): inference: fully reproducible. Every fetched page is authored prose with no underlying dataset, no licensing and no proprietary corpus; the largest bank in the cluster (1000+ phrases) is a volume claim, not an asset. A competitor could regenerate an equivalent or larger bank, organised by competency, rating level and role, in a day. That means the phrase bank itself cannot be the business - the defensible layer would be the employer-specific context (rating scale, competency framework, form structure) and the evidence a manager supplies.

**16. AI threat.** Maximal: the proposed product is a chat assistant use case and general assistants already do it. Inference.
 Competitor-record view: inference: low for the content, higher for the workflow. A chat assistant answers the head term better than any fetched page - it emits phrases for any competency and then writes the personalised paragraph, which is precisely the gap every page leaves open. What an assistant does not have by default is the employer's rating scale and competency framework, the manager's notes in a structured place, the review form to write into, and any record-keeping or consistency check across a team. Any durable product here lives in that integration and compliance layer, not in the wording.

**17. Search-engine direct-answer threat.** High. Inference.


**18. Primary risk.** No defensible layer outside an employer's own system.


**19. Evidence quality / confidence.** Fragmentation 0.72 measured, 8 of 8 pages fetched; demand weak.
 Open questions from the competitor record: Whether the searchers are managers writing about others or employees writing self-reviews - the cluster's member queries contain both (performance review manager comments, performance review tips for employees), and the two need different products. | Whether HRIS vendors already ship review generators behind a login: PerformYard's article references an AI review-assist feature, but nothing about its scope, price or quality is visible on the fetched pages. | Whether output format matters enough to monetise - none of the fetched pages exports into an employer's review form, and it is unknown whether that friction is real for searchers. | Whether HR-risk wording review (defensible, evidence-linked language) is something buyers would pay for; no fetched page addresses it.

**20. Verdict.** Reviewed, not advanced: the most AI-exposed cluster in the set despite the highest fragmentation.


