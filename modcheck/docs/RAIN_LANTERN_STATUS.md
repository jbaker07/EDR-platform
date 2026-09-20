# rain_charged_lantern_v2 — status

Six things are kept separate below because collapsing any two of them is how
"it builds" becomes "it works".

## Per requirement

| Requirement | Implementation | Startup connection | Tested outside the game | Observed in game |
|---|---|---|---|---|
| `accumulate_while_raining` | `LanternBlockEntity.serverTick`, `LanternCharge.step` | `LanternBlock.getTicker` → the game ticks each loaded lantern | yes — 11 tests | **no** |
| `retain_charge_across_reload` | `LanternBlockEntity.loadAdditional`/`saveAdditional` | block entity type registered in `RainLanternContent.register()` | contract only — 5 tests | **no** |
| `show_remaining_charge` | `client/LanternHud` + `ChargeCache` | `client` entrypoint → `LanternHud.register()` | yes — 12 tests | **no** |
| `server_configurable_rate` | `RainLanternContent.CHARGE_RATE` gamerule | static init, forced from `register()` | bounds only — 3 tests | **no** |
| resources and wiring | blockstate, models, loot table, lang | asserted against the real files | yes — 11 tests | **no** |
| join: charge reaches the client | `RainLanternNetwork` + client receiver | type on `main`, receiver on `client` | cache side only | **no** |

## What is established, level by level

| Level | State |
|---|---|
| API/mechanism established | yes — every signature read from the resolved jars, hashes recorded |
| code written | yes — hand-authored, see `reference/rainlantern/PROVENANCE.md` |
| wiring connected | yes — both entrypoints call registration; manifest declares both |
| build passed | yes — `rainlantern-1.0.0.jar`, Java 25.0.4.1, Gradle 9.5.1 |
| behaviour tested outside the game | partly — 40 tests, scope below |
| behaviour observed in the game | **no** — blocked, no installation |
| product-generated | **no** — this is developer-authored reference work |

## What the 40 tests do not reach

They exercise pure logic. Everything that needs Minecraft's own machinery is
untested here, and saying so is not a formality — each of these is a real way
the feature could be broken while every test passes:

- that Minecraft calls `loadAdditional`/`saveAdditional` at all, and that a
  chunk round-trips on disk;
- that `isRainingAt(pos.above())` means what the proposed contract says in
  every biome and weather state;
- that the ticker is actually installed for the block entity type;
- that a payload is encoded, sent, received and decoded;
- that `centeredText` puts legible text on screen at any GUI scale;
- that `hitResult` names the block the player means.

Eleven of the forty read the actual resource and source files rather than
asserting on strings written elsewhere, because every failure they catch is
silent rather than loud: a block with no blockstate renders as the
missing-texture cube, one with no loot table drops nothing, and a registration
method nothing calls leaves a mod that loads and does nothing. None of those
produce a compile error. Both were mutation-checked — removing
`RainLanternContent.register()` and deleting the loot table each fail the test
that exists to catch them.

The serialization tests stand in for the load/save *contract* — same key, same
defaulting, same clamping — because `ValueInput`/`ValueOutput` cannot be
constructed outside the game. A change to either side shows up as a
disagreement; the game calling them at all does not.

## Runtime gate

Blocked. No Minecraft installation, no licence. The natural next step is
Fabric's own `fabric-gametest-api-v1`, which is on the resolved classpath and
would cover the load/save round trip and the ticker installation inside a
headless server — but it still needs the game.
