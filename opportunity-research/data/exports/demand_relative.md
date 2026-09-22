# Relative demand signals (repaired statement, 2026-09-21)

## What each number is, and is not

- **Discovery counts.** Phrase counts in this store (850,100 distinct autocomplete phrases; per-cluster member counts;
  per-task assignment counts) are counts of suggestions the engines returned to our probes. They are not searches,
  people, sessions or monthly demand, and they are shaped by our probe design (14 intent prefixes per seed).
- **Google Trends chain values.** 763 head phrases were placed on one relative scale by chained five-term comparisons
  (root "sourdough starter" = 100). A value is the 12-month mean interest of that exact phrase relative to the root,
  carrying integer-rounding error plus unmodelled error from anchor drift, phrase scope (exact phrase only, not the
  long tail) and Trends' own sampling. Use: ordering and seasonality of head phrases. Do not use: conversion to
  monthly searches. The earlier advice to convert the whole scale with one keyword-volume anchor is withdrawn: a
  calibrated model needs matched scopes, several validation anchors, holdout checks and an error model beyond rounding.
- **Near-zero values** mean the exact phrase reads below Trends' reporting floor against its anchor; they are not
  proof of no demand for the task, whose demand lives in many phrasings.
- **Brand phrase versus problem phrase.** "pcpartpicker" (12.0) against "cpu motherboard compatibility" (0.5) says
  the brand phrase is searched more often than that one generic phrase. It does not measure the share of people with
  the problem who use the incumbent; the earlier inference to that effect is withdrawn.

## Head phrases placed (unchanged values, relabelled)

| phrase | chain value (root = 100) | error shown by the chain |
|---|---|---|
| sourdough starter | 100 | root |
| password manager | 84.5 | ±2% |
| circuit breaker | 51.4 | ±3% |
| docker compose | 31.8 | ±2% |
| github actions | 27.7 | ±4% |
| cities skylines | 16.2 | ±5% |
| pcpartpicker | 12.0 | ±2% |
| excel formulas | 10.5 | ±7% |
| chess openings | 9.5 | ±6% |
| roof leaking | 4.7 | ±3% |
| obd2 codes | 1.5 | ±3% |
| garden pruning | 1.1 | ±4% |
| renovation cost per square foot | 0.9 | ±5% |
| how to move abroad | 0.7 | ±38% |
| best settings for rivals | 0.6 | ±36% |
| stl files for 3d printing | 0.6 | ±18% |
| cpu motherboard compatibility | 0.5 | ±10% |

Full placements with anchors and reads: `data/trends_chain_state.json` (metrics table, provider `google_trends`).

## What is required for measured demand

Actual keyword metrics from an authorised provider or a Keyword Planner export, per shortlisted task, stored with:
provider, retrieval date, geography, language, network, monthly time series where supplied, and the provider's
variant-group information. US and global are kept separate; ranges are kept as ranges. Identical volumes do not
imply duplicate queries, and our token canonicalisation is not the provider's close-variant grouping, so unresolved
overlap is reported, never summed into a false total. Paid-search competition and CPC are stored but are not organic
ranking difficulty or publisher RPM. The request lists are in `data/exports/keyword_requests/`.
