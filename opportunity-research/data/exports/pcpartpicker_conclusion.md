# Incumbent check: PCPartPicker (2026-09-21)

## What was attempted, and what blocked it (measured)

- The research environment's fetch tool returned HTTP 403 for pcpartpicker.com (homepage and FAQ).
- A direct request with a desktop browser user agent to three URLs (FAQ, motherboard product listing, a forum topic about
  BIOS updates) returned 403 with Cloudflare challenge tokens in the body: the site sits behind bot protection.
- web.archive.org is not reachable from this environment, and the web-search tool's 200-call session budget was spent.
- No page of pcpartpicker.com was therefore read in this project. Nothing below about its features is verified here.

## What is measured about it

- In the web-search proxy results for the two checked queries of the compatibility cluster ("cpu motherboard
  compatibility", "best cpu compatible with my motherboard"), pcpartpicker.com held 1 of 18 results, titled
  "PCPartPicker - Motherboard + CPU compatibility". Present, not dominant, in that proxy.
- A fetched editorial page (cgdirector.com) describes it as an aggregator of "noted compatibilities" and sends readers to
  board makers' CPU support lists for the authoritative answer.
- On the Google Trends chain (root "sourdough starter" = 100): the brand phrase "pcpartpicker" reads 12.0 (±2%); the
  problem phrase "cpu motherboard compatibility" reads 0.5 (±10%); "motherboard cpu support list" and "bios update for
  new cpu" read near zero with unreliable error. The brand is searched roughly 24 times more often than the generic
  problem phrase. Measured for these exact phrases only; long-tail product-name phrasings are not summed here.

## Background knowledge, stated as such (not verified in this project)

PCPartPicker's part list checks socket, chipset, form factor, memory type, cooler clearance and power. On some CPU and
motherboard pairings it shows a compatibility note saying a BIOS update may be required before the CPU is supported, and
that updating may need an older supported CPU. To my knowledge it does not show the minimum BIOS version for a specific
board model, does not reproduce board makers' per-model CPU support tables, and does not publish a standalone page per
CPU-board pair. Treat every sentence in this paragraph as a hypothesis to check in a browser.

## Conclusion (inference)

PCPartPicker is the dominant destination for "will these parts work together" and already answers the coarse form of the
#1 problem; the 24-to-1 brand-to-problem ratio says most people who have the problem go straight to it. What remains is
the fine form: the exact minimum BIOS version, the flash-before-install decision, and a search-landing page per pair,
which no checked result provides and which PCPartPicker, on general knowledge, only gestures at with a generic note.
That gap is real on the evidence but narrower than "no tool exists", and the incumbent holds the data pipeline that
could close it. The #1 recommendation therefore stands only as a narrow data gap beside a strong incumbent, with
head-phrase demand that is small on every free signal; its case rests on the long tail of product-name queries, which
this route cannot size.

## Browser check by the founder (2026-09-21, screenshots transcribed)

A part list with an MSI B450 TOMAHAWK MAX ATX AM4 motherboard and an AMD Ryzen 5 5600X showed an orange
compatibility banner ("Warning! These parts have potential issues") and, under "Potential Issues":

> "Warning: The MSI B450 TOMAHAWK MAX ATX AM4 Motherboard supports the AMD Ryzen 5 5600X 3.7 GHz 6-Core Processor
> with BIOS version Latest Beta BIOS. If the motherboard is using an older BIOS version, updating the BIOS will be
> necessary to support the CPU."

together with a "Motherboard Usage" diagram linking the CPU socket to the CPU with the warning marker. This is
measured fact from the incumbent's own page.

What it establishes: PCPartPicker holds a per-board, per-CPU BIOS-support field and surfaces it inside the builder.
The background-knowledge paragraph above, which assumed only a generic note, was wrong on that point. What the check
also shows: the value for this pairing is "Latest Beta BIOS", not a specific version string with a date, so the
precision of that field is unknown across pairings; and the information is shown inside a part list, not on a
page that a search for "b450 tomahawk max 5600x bios version" would land on.

## Revised conclusion (inference, after the browser check)

The core data gap claimed for the #1 is closed at the incumbent: the dominant destination already maps board × CPU to
a BIOS-support value and warns when an update is needed. What remains is (a) a precision gap, if many pairings read
"Latest Beta BIOS" rather than a version and date, (b) a board-first table of all supported CPUs with versions, and
(c) search-landing pages per pair. Those are a content and presentation layer, not a missing dataset, and the
incumbent could add them at will. A standalone compatibility site is therefore not justified on this evidence; the
#1 is withdrawn as a standalone opportunity and recorded as "gap closed by the incumbent, residual is SEO landing
pages". Checks 2 and 3 below remain useful only to size that residual.

## Ten-minute verification for a person with a browser (check 1 done; 2 to 4 optional)

1. Build a list on pcpartpicker.com with an AMD B450 board and a Ryzen 5 5600X (and an AM5 B650 board with the newest
   Ryzen). Copy the exact compatibility note text. Does it name a minimum BIOS version, or only "may need an update"?
2. Open one motherboard product page. Is there a CPU support list with BIOS versions, or only specifications?
3. Search Google for "b450 tomahawk max 5600x bios version" and "b650 bios version for 9800x3d". Record which sites hold
   the top ten and whether any page states the version. If pcpartpicker.com or a board maker's page answers it inline,
   the gap is closed; if forums and blogs answer it, the gap is open.
4. Optional: one month of keyword data for twenty board-name plus CPU-name phrases sizes the long tail.
