# Relative demand on the Google Trends chain (free route)

Every value is the 12-month mean Google Trends interest of the exact phrase, chained onto one scale where the root anchor "sourdough starter" = 100. Each placement carries the integer-rounding error of the comparison it was read from. This is a measured relative signal for the exact head phrase only: it does not sum the hundreds of long-tail phrasings in a cluster, and it is not a monthly search count. To convert any row to an absolute number, one calibration point is enough: look up one reference phrase in a keyword tool that reports monthly volume, divide by its value here, and multiply every other row by that factor (error about a factor of two for neighbouring rows, larger for rows marked unreliable).

## Top-20 head terms and related phrasings

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

## Reference phrases on the same scale

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

