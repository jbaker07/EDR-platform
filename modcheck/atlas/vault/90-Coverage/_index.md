---
type: "coverage"
generated_at: "2026-09-20T08:10:20+00:00"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Coverage, with denominators

Seven separate measures. None of them is 'supported'. A capability record, a linked note, or a passing schema establishes nothing about behaviour.

## 1. Surface inventory

- artifacts classified: 68 / 68 (0 unclassified)
- Minecraft packages (depth 4) inventoried: 111 / 111, 10715 top-level classes
- data pack entry types inventoried: 44 (from the jar)
- Fabric API modules inventoried: 47 / 47

## 2. Extraction

- mixin classes declared in mixin configs: 513; with @Mixin targets extracted: 511 (classes under mixin packages in total, including nested and helper classes: 600)
- Fabric API classes with public surface extracted: 394 / 399
- vanilla types that are hook targets: 628
- vanilla packages with at least one hook: 63 / 111
- hooked types that live outside the merged jar (libraries): 2 -- no members, by construction
- vanilla classes whose members were extracted: 628 / 10715 (the 626 hook targets plus BuiltInRegistries, Registries)
- edge targets cross-checked against those members: 1539 declared on the type, 123 declared on a superclass, 75 unresolved / 1737 (unresolved = interface-declared, `java.lang.Object`, wildcard `<clinit>*`, or Fabric interface-injected members; see unresolved)

## 3. Workflow information

- workflow families authored: 26 across 8 / 8 areas: {'behaviour': 4, 'content': 4, 'engineering': 4, 'integration': 3, 'multiplayer': 2, 'presentation': 4, 'state': 2, 'world': 3}
- families with interaction analysis: 12 / 26

## 4. Interaction contracts

- typed edges: 3203 -- by relation {"injects_into": 479, "calls": 1975, "reads": 168, "publishes_event": 235, "callback_of": 189, "wraps": 60, "registers_into": 97}
- by evidence class: {"direct_reference": 2779, "static_inference": 235, "declared": 189}
- events with an identified publisher: 182 / 189
- events with an analyst-stated subscriber contract: 15 / 189
- curated interaction records in the store: 0

## 5. Executable analysis (ModCheck)

- capability records: 9; failures with a detector: 1 / 3
- static mixin-collision analysis over arbitrary mod jars: NOT implemented (the jvm inspector lists mixin classes; it does not read their targets)

## 6. Runtime evidence

- observed edges: 0. No game has run.

## 7. Creation / maintenance automation

- deterministic generators: 6 mechanisms (Fabric only)
- agent execution: boundary exists, unavailable here

## Per-system state

Columns are independent: *inspected* = members of its hooked types extracted; *contract-mapped* = analyst contracts whose publication site is in the package; *analysed* = an analyst system note exists; *in ModCheck* = a request analysis names it, marked if that request is implemented; *validated* = the validation scope of that implementation.

| package | inventoried | inspected | contract-mapped | analysed | in ModCheck | validated |
|---|---|---|---|---|---|---|
| `net.minecraft.advancements` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.advancements.predicates` | yes | no | no | no | no | none |
| `net.minecraft.advancements.triggers` | yes | no | no | no | no | none |
| `net.minecraft.client` | yes | members of 9/9 hooked types | 1 contracts | edges only | request.lantern_moth; request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.client.animation` | yes | no | no | no | no | none |
| `net.minecraft.client.color` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.client.data` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.client.entity` | yes | no | no | no | no | none |
| `net.minecraft.client.gui` | yes | members of 53/53 hooked types | no | system note | request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.client.input` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.client.main` | yes | no | no | no | no | none |
| `net.minecraft.client.model` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.client.multiplayer` | yes | members of 15/15 hooked types | 1 contracts | system note | no | none |
| `net.minecraft.client.particle` | yes | members of 6/6 hooked types | no | edges only | no | none |
| `net.minecraft.client.player` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.client.profiling` | yes | no | no | no | no | none |
| `net.minecraft.client.quickplay` | yes | no | no | no | no | none |
| `net.minecraft.client.renderer` | yes | members of 66/66 hooked types | no | system note | request.lantern_moth | none |
| `net.minecraft.client.resources` | yes | members of 13/13 hooked types | no | edges only | no | none |
| `net.minecraft.client.searchtree` | yes | no | no | no | no | none |
| `net.minecraft.client.server` | yes | no | no | no | no | none |
| `net.minecraft.client.sounds` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.client.telemetry` | yes | no | no | no | no | none |
| `net.minecraft.client.tutorial` | yes | no | no | no | no | none |
| `net.minecraft.client.waypoints` | yes | no | no | no | no | none |
| `net.minecraft.commands` | yes | members of 3/3 hooked types | 1 contracts | edges only | no | none |
| `net.minecraft.commands.arguments` | yes | no | no | no | no | none |
| `net.minecraft.commands.execution` | yes | no | no | no | no | none |
| `net.minecraft.commands.functions` | yes | no | no | no | no | none |
| `net.minecraft.commands.synchronization` | yes | no | no | no | no | none |
| `net.minecraft.core` | yes | members of 29/29 hooked types | no | edges only | request.crystal_caves | none |
| `net.minecraft.core.cauldron` | yes | no | no | no | no | none |
| `net.minecraft.core.component` | yes | members of 12/12 hooked types | no | edges only | no | none |
| `net.minecraft.core.dispenser` | yes | no | no | no | no | none |
| `net.minecraft.core.particles` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.core.registries` | yes | members of 2/2 hooked types | no | system note | request.crystal_caves | none |
| `net.minecraft.data` | yes | members of 6/6 hooked types | no | edges only | no | none |
| `net.minecraft.data.advancements` | yes | no | no | no | no | none |
| `net.minecraft.data.info` | yes | no | no | no | no | none |
| `net.minecraft.data.loot` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.data.metadata` | yes | no | no | no | no | none |
| `net.minecraft.data.recipes` | yes | members of 13/13 hooked types | no | edges only | no | none |
| `net.minecraft.data.registries` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.data.structures` | yes | no | no | no | no | none |
| `net.minecraft.data.tags` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.data.worldgen` | yes | no | no | no | no | none |
| `net.minecraft.gametest` | yes | no | no | no | no | none |
| `net.minecraft.gametest.framework` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.gizmos` | yes | no | no | no | no | none |
| `net.minecraft.locale` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.nbt` | yes | members of 5/5 hooked types | no | edges only | no | none |
| `net.minecraft.nbt.visitors` | yes | no | no | no | no | none |
| `net.minecraft.network` | yes | members of 13/13 hooked types | no | edges only | request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.network.chat` | yes | members of 9/9 hooked types | no | edges only | no | none |
| `net.minecraft.network.codec` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.network.protocol` | yes | members of 30/30 hooked types | no | system note | request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.network.syncher` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.realms` | yes | no | no | no | no | none |
| `net.minecraft.recipebook` | yes | no | no | no | no | none |
| `net.minecraft.references` | yes | no | no | no | no | none |
| `net.minecraft.resources` | yes | members of 12/12 hooked types | no | edges only | no | none |
| `net.minecraft.server` | yes | members of 12/12 hooked types | 4 contracts | system note | request.crystal_caves; request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.server.advancements` | yes | no | no | no | no | none |
| `net.minecraft.server.bossevents` | yes | no | no | no | no | none |
| `net.minecraft.server.chase` | yes | no | no | no | no | none |
| `net.minecraft.server.commands` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.server.dedicated` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.server.dialog` | yes | no | no | no | no | none |
| `net.minecraft.server.gui` | yes | no | no | no | no | none |
| `net.minecraft.server.jsonrpc` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.server.level` | yes | members of 13/13 hooked types | 4 contracts | system note | request.rain_lantern (implemented); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.server.network` | yes | members of 10/10 hooked types | no | system note | no | none |
| `net.minecraft.server.notifications` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.server.packs` | yes | members of 25/25 hooked types | no | system note | request.crystal_caves | none |
| `net.minecraft.server.permissions` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.server.players` | yes | members of 4/4 hooked types | 3 contracts | edges only | no | none |
| `net.minecraft.server.rcon` | yes | no | no | no | no | none |
| `net.minecraft.server.waypoints` | yes | no | no | no | no | none |
| `net.minecraft.sounds` | yes | no | no | no | no | none |
| `net.minecraft.stats` | yes | no | no | no | no | none |
| `net.minecraft.tags` | yes | members of 6/6 hooked types | no | edges only | no | none |
| `net.minecraft.util` | yes | members of 16/16 hooked types | no | edges only | request.port_1_21_mod | none |
| `net.minecraft.util.context` | yes | no | no | no | no | none |
| `net.minecraft.util.datafix` | yes | members of 2/2 hooked types | no | system note | request.port_1_21_mod | none |
| `net.minecraft.util.debug` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.util.debugchart` | yes | no | no | no | no | none |
| `net.minecraft.util.eventlog` | yes | no | no | no | no | none |
| `net.minecraft.util.filefix` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.util.monitoring` | yes | no | no | no | no | none |
| `net.minecraft.util.parsing` | yes | no | no | no | no | none |
| `net.minecraft.util.profiling` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.util.random` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.util.thread` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.util.valueproviders` | yes | no | no | no | no | none |
| `net.minecraft.util.worldupdate` | yes | no | no | no | no | none |
| `net.minecraft.world` | yes | members of 9/9 hooked types | no | edges only | request.crystal_caves; request.lantern_moth; request.lantern_moth; request.rain_lantern (implemented); request.villager_fear; request.villager_fear | JUnit with fakes, gradle build; no game run |
| `net.minecraft.world.attribute` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.world.clock` | yes | no | no | no | no | none |
| `net.minecraft.world.damagesource` | yes | members of 1/1 hooked types | no | edges only | no | none |
| `net.minecraft.world.effect` | yes | no | no | no | no | none |
| `net.minecraft.world.entity` | yes | members of 17/17 hooked types | 1 contracts | system note | request.lantern_moth; request.villager_fear | none |
| `net.minecraft.world.flag` | yes | members of 2/2 hooked types | no | edges only | no | none |
| `net.minecraft.world.food` | yes | no | no | no | no | none |
| `net.minecraft.world.inventory` | yes | members of 4/4 hooked types | no | edges only | no | none |
| `net.minecraft.world.item` | yes | members of 31/31 hooked types | no | system note | request.villager_fear | none |
| `net.minecraft.world.level` | yes | members of 106/106 hooked types | 1 contracts | system note | request.crystal_caves; request.lantern_moth; request.rain_lantern (implemented) | JUnit with fakes, gradle build; no game run |
| `net.minecraft.world.phys` | yes | members of 3/3 hooked types | no | edges only | no | none |
| `net.minecraft.world.scores` | yes | no | no | no | no | none |
| `net.minecraft.world.ticks` | yes | no | no | no | no | none |
| `net.minecraft.world.timeline` | yes | no | no | no | no | none |
| `net.minecraft.world.waypoints` | yes | no | no | no | no | none |
