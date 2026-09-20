---
type: "request"
id: "request.rain_lantern"
canonical: "rain_charged_lantern_v2"
kind: "creator_request"
family: "workflow:wf.content.block_entity"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# rain_charged_lantern_v2 -- per-lantern charge that survives a reload, shown for the one you look at

**Canonical request.** `rain_charged_lantern_v2` (creator_request), `modcheck/evaluation/requests/rain_charged_lantern_v2.yaml`, sha256 `341762c83b5b2e04`, revision of `rain_charged_lantern`; requirements: `accumulate_while_raining`, `retain_charge_across_reload`, `show_remaining_charge`, `server_configurable_rate`

## Request

Quoted from the canonical record: "Revision of rain_charged_lantern. Charge is per lantern rather than per world, and the HUD shows the charge of the lantern the player is looking at. Everything else is unchanged." The revision replaced an accepted requirement deliberately: a single world-scoped value satisfied the old reload criterion and cannot satisfy the new one.

## Approved behaviour (the request's own words or acceptance criteria)

- accumulate_while_raining (server): while it is raining at a lantern's position, THAT lantern's stored charge increases on a fixed schedule. Acceptance: two lanterns, one under rain and one sheltered; after N ticks only the exposed one's charge has increased.
- retain_charge_across_reload (server, scope: block): each lantern's own charge survives saving and reloading the world, independently of every other lantern's. Acceptance: charges C1 != C2 are still C1 and C2 after save and reload.
- show_remaining_charge (client): the player sees the charge of the lantern they are looking at; looking away or at another lantern shows that one's or nothing. A single number that does not change between lanterns is a failure.
- server_configurable_rate (server): an operator changes the charge rate without editing or rebuilding the mod; after a restart the observed rate matches.

## Proposed contract (implementer's decisions, not creator intent)

- Charge is a non-negative integer capped at 1000; the request states no maximum.
- The rate is charge points per charging interval, not per tick or per second.
- The charging interval is every 20 ticks: game time, so slower on a loaded server, which is correct.
- Exposure is Level.isRainingAt(pos.above()): vanilla's own sky/heightmap/biome test at the block above the lantern, so a lantern under a slab is sheltered ([[40-Interfaces/net.minecraft.world.level.Level|Level]] declares isRainingAt(BlockPos); `extracted/minecraft_surface.json.gz`).
- A placed lantern starts at zero; breaking discards the charge; the item does not carry it.
- A change is sent to the players tracking the lantern's chunk, not to everyone and not only to the viewer.
- A client entry older than 200 ticks is unknown and the HUD shows nothing rather than a stale number.
- Looking at anything that is not a lantern shows nothing.

## Analyst assumptions

- None beyond the proposed contract. The earlier version of this note (commit 104d5a3e) described a per-level rain flag with level-scoped SavedData and a payload on change; that was the discarded scaffold's design, not the request, and it was wrong on both the scope and the attribution below.

## Preservation obligations

- Vanilla lanterns, weather and every other block; the mod adds a block, it alters nothing.
- Existing worlds load unchanged when the mod is added; removing it leaves an unknown block entity, which vanilla tolerates.

## Affected systems

- net.minecraft.world.level -- the block (BaseEntityBlock), its block entity and ticker; the weather query on Level; block-entity persistence through ValueInput/ValueOutput.
- net.minecraft.server.level -- ServerLevel ticks loaded block entities; ServerPlayer is the sync recipient; game rules are read from ServerLevel.
- net.minecraft.network.protocol -- the custom payload carrying (pos, charge).
- net.minecraft.client.gui -- the HUD element; hitResult selects the viewed lantern.
- net.minecraft.core.registries -- BLOCK, ITEM, BLOCK_ENTITY_TYPE registrations ([[30-Mechanisms/Registries|Registries]]).

## Implementation candidates

- Implemented (reference/rainlantern, hand-authored, see reference/rainlantern/PROVENANCE.md): LanternBlock extends BaseEntityBlock and installs a BlockEntityTicker; LanternBlockEntity.serverTick steps LanternCharge every interval when isRainingAt(pos.above()) and marks dirty; loadAdditional/saveAdditional persist the charge per block entity; RainLanternNetwork sends LanternChargePayload to PlayerLookup.tracking(chunk) on change; RainLanternClient receives into ChargeCache; LanternHud reads the viewed block from hitResult and renders through HudElementRegistry; RainLanternContent registers block, item, block entity type and the CHARGE_RATE game rule via GameRuleBuilder ([[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]]).
- Alternative A (rejected by the revision): one level-scoped value in SavedData stepped from END_LEVEL_TICK (`capability/persist_state.fabric_saveddata`, `capability/subscribe_event.fabric_server_tick`). Satisfied v1's criterion; fails v2's two-lantern criterion. This was what the ModCheck scaffold generated.
- Alternative B: block-entity attachment ([[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]]) with syncWith instead of an explicit payload. Fewer lines; the sync predicate cannot express 'players tracking the chunk' (`extracted/fabric_api.json#fabric-data-attachment-api-v1` exposes all / targetOnly / allButTarget), so the proposed recipients rule would not hold.

## Data / control / state dependencies

- Data: the per-block charge (block entity NBT via ValueOutput); the CHARGE_RATE game rule value; the client ChargeCache keyed by BlockPos with a tick stamp.
- Control: vanilla's block-entity ticking (no Fabric event); ServerPlayNetworking send on change; ClientPlayConnectionEvents disconnect clears the cache ([[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents.DISCONNECT|DISCONNECT]]); HUD element registration ([[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]).
- State: server-authoritative per block entity; client copy per connection, expiring at 200 ticks.

## Interactions with the selected environment

- The ticker runs only for loaded chunks; an unloaded lantern does not charge, which the request does not address and the proposed contract accepts silently -- flagged here, not decided.
- isRainingAt depends on biome precipitation and sky exposure; another mod altering weather or biome rain changes charging correctly because the lantern reads, it does not predict.
- Payload recipients are the chunk's trackers; a player who starts tracking later receives nothing until the next change (the same join gap as the previous analysis, now stated for the right mechanism: the entity/chunk tracking events in [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] could fill it).
- No mixin: the mod subscribes and registers only, so it shares no target with any other mod's injections (`extracted/edges.json#shared_targets` is irrelevant to it).

## Alternatives and tradeoffs

- Item-carried charge (breaking keeps the charge in the dropped item): excluded by the proposed contract, not by the request.

## Implementation work

- Done: the ten hand-authored classes, resources (blockstate, models, loot table, lang), manifest, and 40 JUnit tests (LanternChargeTest 13, ChargeCacheTest 11, ChargeSerializationTest 5, ResourcesAndWiringTest 11); gradle build produced rainlantern-1.0.0.jar.
- Generated by ModCheck: none of the mod's code. The earlier scaffold's five generated classes (level-scoped) were replaced, not filled in; build.gradle, settings.gradle, gradle.properties and the wrapper properties came from modcheck scaffold.
- Remaining: late-tracker sync; a headless run for every runtime-only claim below.

## Verification obligations

- Established: compiles against the pinned corpus; charging rules, bounds, per-lantern independence, save/load contract (same key, defaulting, clamping), target switching, stale and missing client state -- by JUnit with fakes; resources and wiring asserted against the real files and mutation-checked (removing register() and deleting the loot table each fail a test).
- Not established (needs the game): that Minecraft calls loadAdditional/saveAdditional and round-trips the chunk; that the ticker is installed for the type; that isRainingAt(pos.above()) means what the contract says in every biome and weather state; that the payload is encoded, sent, received and decoded; that the HUD text renders at any GUI scale; that hitResult names the block the player means.

## Unresolved

- [[80-Unresolved/q.lantern_runtime_gate|q.lantern_runtime_gate]]
- [[80-Unresolved/q.lantern_rain_semantics|q.lantern_rain_semantics]]
- [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]
- [[80-Unresolved/q.runtime_performance|q.runtime_performance]]

## Evidence

- [[00-Scope/Sources|fabric_game_rule_api_v1_jar]]
- [[00-Scope/Sources|fabric_networking_api_v1_jar]]
- [[00-Scope/Sources|fabric_rendering_v1_jar]]
- [[00-Scope/Sources|minecraft_26_3_merged_jar]]
- `extracted/minecraft_surface.json.gz`
- `extracted/resolved_environment.json`
- [[40-Interfaces/net.minecraft.world.level.Level|Level]]
- [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]]
- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]]
- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]]

## Status

- analysed: True
- implemented: reference/rainlantern, hand-authored (not product-generated)
- validated_scope: gradle build and 40 JUnit tests with fakes; resources mutation-checked; no game run
