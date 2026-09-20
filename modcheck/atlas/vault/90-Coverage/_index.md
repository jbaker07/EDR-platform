---
type: "coverage"
generated_at: "2026-09-20T09:11:07+00:00"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Coverage, with denominators

Seven separate measures. None of them is 'supported'. A capability record, a linked note, or a passing schema establishes nothing about behaviour.

## 1. Surface inventory

- artifacts classified: 144 / 144 (0 unclassified); groups {"minecraft_processed": 1, "minecraft": 6, "minecraft_library": 74, "mixin_runtime_extras": 1, "loader_runtime_dep": 1, "build_tooling": 12, "fabric_loader": 1, "mixin_runtime": 1, "fabric_api_module": 47}
- classes in the processed compile jar: 11383 (7301 top-level) in 565 packages, 150385 declared members -- all inventoried (``extracted/minecraft_surface.json.gz``)
- depth-4 packages: 134; with at least one Fabric API hook: 88
- data pack entry types inventoried: 44 (from the jar)
- Fabric API modules inventoried: 47 / 47
- resolved environment recorded: compile 114, runtime 126, test 134 entries, all hashed; processed-vs-cache jar diff: 363 classes

## 2. Extraction

- mixin classes: 512 found by annotation / 512 declared in configs; extraction failures: 0
- injections: 742 by injector {"Inject": 421, "WrapOperation": 115, "Redirect": 60, "ModifyExpressionValue": 45, "ModifyArg": 32, "ModifyVariable": 26, "ModifyReturnValue": 20, "WrapMethod": 15, "WrapWithCondition": 6, "ModifyReceiver": 1, "Overwrite": 1}
- Fabric API classes with public surface extracted: 399 / 399
- vanilla types that are hook targets: 893; every one has its declared members in the surface

## 3. Workflow information

- workflow families authored: 26 across 8 / 8 areas: {'behaviour': 4, 'content': 4, 'engineering': 4, 'integration': 3, 'multiplayer': 2, 'presentation': 4, 'state': 2, 'world': 3}
- families with interaction analysis: 12 / 26

## 4. Interaction contracts

- typed edges: 7825 -- by relation {"injects_into": 544, "wraps": 197, "reads": 1431, "calls": 5088, "publishes_event": 242, "callback_of": 189, "writes": 36, "replaces": 1, "registers_into": 97}
- by evidence class: {"direct_reference": 7094, "declared": 489, "static_inference": 242}
- resolution of injection selectors: injects_into {"exact": 93, "name_only": 441, "ambiguous": 5, "selector_unsupported": 5}; wraps {"exact": 33, "name_only": 163, "ambiguous": 1}; replaces {"exact": 1}
- resolution of injection points (@At): {"exact": 332, "inherited_exact": 49, "selector_unsupported": 4}
- resolution of calls / reads / writes: {"exact": 4373, "inherited_exact": 715} / {"exact": 1422, "inherited_exact": 9} / {"exact": 36}
- events with an identified publisher: 187 / 189
- events with an analyst-stated subscriber contract: 15 / 189
- shared targets (potential interactions, no verdict): 34; same-point overlap: not computed
- curated interaction records in the store: 0

## 5. Executable analysis (ModCheck)

- capability records: 9; failures with a detector: 1 / 3
- static mixin-collision analysis over arbitrary mod jars: NOT implemented (the extractor and resolver exist under atlas/extract; the jvm inspector does not call them; the required order is index -> exact resolution -> applicability -> composition rule)

## 6. Runtime and transformation evidence

- observed edges: 0. No game has run.
- executed_transformation: 14 scenarios run through the pinned Mixin transformer (9 transformed, 5 refused at transformation); transformer evidence, not Minecraft evidence

## 7. Creation / maintenance automation

- deterministic generators: 6 mechanisms (Fabric only); the reference lantern was hand-authored, not generated
- agent execution: boundary exists, unavailable here

## Per-system state

Columns are independent: *inventoried* = every class and member of the package is in the surface; *inspected* = hooked types with their edges resolved; *contract-mapped* = analyst contracts whose publication site is in the package; *analysed* = an analyst system note exists; *in ModCheck* = a request analysis names it; *validated* = the validation scope of that implementation.

| package | inventoried | inspected | contract-mapped | analysed | in ModCheck | validated |
|---|---|---|---|---|---|---|
| `com.mojang.blaze3d` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.audio` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.buffers` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.font` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.framegraph` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.pipeline` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.blaze3d.platform` | all classes | 13 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.blaze3d.resource` | all classes | no hooks | no | no | no | none |
| `com.mojang.blaze3d.systems` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.blaze3d.vertex` | all classes | 5 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.math` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.realmsclient` | all classes | no hooks | no | no | no | none |
| `com.mojang.realmsclient.client` | all classes | no hooks | no | no | no | none |
| `com.mojang.realmsclient.dto` | all classes | no hooks | no | no | no | none |
| `com.mojang.realmsclient.exception` | all classes | no hooks | no | no | no | none |
| `com.mojang.realmsclient.gui` | all classes | no hooks | no | no | no | none |
| `com.mojang.realmsclient.util` | all classes | no hooks | no | no | no | none |
| `com.mojang.renderpearl` | all classes | no hooks | no | no | no | none |
| `com.mojang.renderpearl.api` | all classes | 7 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.renderpearl.backend` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `com.mojang.renderpearl.frontend` | all classes | no hooks | no | no | no | none |
| `com.mojang.renderpearl.util` | all classes | no hooks | no | no | no | none |
| `net.minecraft` | all classes | 8 hooked types, edges resolved | no | edges only | request.crystal_caves; request.crystal_caves; request.crystal_caves; request.lantern_moth; request.lantern_moth; request.lantern_moth; request.port_1_21_mod; request.rain_lantern (implemented, hand-authored); request.rain_lantern (implemented, hand-authored); request.rain_lantern (implemented, hand-authored); request.rain_lantern (implemented, hand-authored); request.rain_lantern (implemented, hand-authored); request.team_counter; request.team_counter; request.team_counter; request.villager_fear; request.villager_fear | JUnit with fakes, gradle build; no game run |
| `net.minecraft.advancements` | all classes | 6 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.advancements.predicates` | all classes | no hooks | no | no | no | none |
| `net.minecraft.advancements.triggers` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client` | all classes | 12 hooked types, edges resolved | 1 contracts | edges only | request.lantern_moth; request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.client.animation` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.color` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.data` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.entity` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.gui` | all classes | 67 hooked types, edges resolved | no | system note | request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.client.input` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.main` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.model` | all classes | 4 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.multiplayer` | all classes | 18 hooked types, edges resolved | 1 contracts | system note | no | none |
| `net.minecraft.client.particle` | all classes | 7 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.player` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.profiling` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.quickplay` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.renderer` | all classes | 87 hooked types, edges resolved | no | system note | request.lantern_moth | none |
| `net.minecraft.client.resources` | all classes | 20 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.searchtree` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.server` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.sounds` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.telemetry` | all classes | no hooks | no | no | no | none |
| `net.minecraft.client.tutorial` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.client.waypoints` | all classes | no hooks | no | no | no | none |
| `net.minecraft.commands` | all classes | 3 hooked types, edges resolved | 1 contracts | edges only | no | none |
| `net.minecraft.commands.arguments` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.commands.execution` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.commands.functions` | all classes | no hooks | no | no | no | none |
| `net.minecraft.commands.synchronization` | all classes | no hooks | no | no | no | none |
| `net.minecraft.core` | all classes | 33 hooked types, edges resolved | no | edges only | request.crystal_caves; request.rain_lantern (implemented, hand-authored) | JUnit with fakes, gradle build; no game run |
| `net.minecraft.core.cauldron` | all classes | no hooks | no | no | no | none |
| `net.minecraft.core.component` | all classes | 15 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.core.dispenser` | all classes | no hooks | no | no | no | none |
| `net.minecraft.core.particles` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.core.registries` | all classes | 3 hooked types, edges resolved | no | system note | request.crystal_caves; request.rain_lantern (implemented, hand-authored) | JUnit with fakes, gradle build; no game run |
| `net.minecraft.data` | all classes | 9 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.advancements` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.info` | all classes | no hooks | no | no | no | none |
| `net.minecraft.data.loot` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.metadata` | all classes | no hooks | no | no | no | none |
| `net.minecraft.data.recipes` | all classes | 15 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.registries` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.structures` | all classes | no hooks | no | no | no | none |
| `net.minecraft.data.tags` | all classes | 4 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.data.worldgen` | all classes | no hooks | no | no | no | none |
| `net.minecraft.gametest` | all classes | no hooks | no | no | no | none |
| `net.minecraft.gametest.framework` | all classes | 5 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.gizmos` | all classes | no hooks | no | no | no | none |
| `net.minecraft.locale` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.nbt` | all classes | 8 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.nbt.visitors` | all classes | no hooks | no | no | no | none |
| `net.minecraft.network` | all classes | 15 hooked types, edges resolved | no | edges only | request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.network.chat` | all classes | 12 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.network.codec` | all classes | 4 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.network.protocol` | all classes | 34 hooked types, edges resolved | no | system note | request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.network.syncher` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.realms` | all classes | no hooks | no | no | no | none |
| `net.minecraft.recipebook` | all classes | no hooks | no | no | no | none |
| `net.minecraft.references` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.resources` | all classes | 12 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server` | all classes | 13 hooked types, edges resolved | 4 contracts | system note | request.crystal_caves; request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.server.advancements` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.bossevents` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.chase` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.commands` | all classes | 6 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server.dedicated` | all classes | 4 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server.dialog` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.gui` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.jsonrpc` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server.level` | all classes | 14 hooked types, edges resolved | 4 contracts | system note | request.rain_lantern (implemented, hand-authored); request.team_counter | JUnit with fakes, gradle build; no game run |
| `net.minecraft.server.network` | all classes | 11 hooked types, edges resolved | no | system note | no | none |
| `net.minecraft.server.notifications` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server.packs` | all classes | 29 hooked types, edges resolved | no | system note | request.crystal_caves | none |
| `net.minecraft.server.permissions` | all classes | 4 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.server.players` | all classes | 4 hooked types, edges resolved | 3 contracts | edges only | no | none |
| `net.minecraft.server.rcon` | all classes | no hooks | no | no | no | none |
| `net.minecraft.server.waypoints` | all classes | no hooks | no | no | no | none |
| `net.minecraft.sounds` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.stats` | all classes | no hooks | no | no | no | none |
| `net.minecraft.tags` | all classes | 12 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util` | all classes | 18 hooked types, edges resolved | no | edges only | request.port_1_21_mod | none |
| `net.minecraft.util.context` | all classes | no hooks | no | no | no | none |
| `net.minecraft.util.datafix` | all classes | 3 hooked types, edges resolved | no | system note | request.port_1_21_mod | none |
| `net.minecraft.util.debug` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.debugchart` | all classes | no hooks | no | no | no | none |
| `net.minecraft.util.eventlog` | all classes | no hooks | no | no | no | none |
| `net.minecraft.util.filefix` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.monitoring` | all classes | no hooks | no | no | no | none |
| `net.minecraft.util.parsing` | all classes | no hooks | no | no | no | none |
| `net.minecraft.util.profiling` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.random` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.thread` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.valueproviders` | all classes | 2 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.util.worldupdate` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world` | all classes | 10 hooked types, edges resolved | no | edges only | request.crystal_caves; request.lantern_moth; request.lantern_moth; request.rain_lantern (implemented, hand-authored); request.villager_fear; request.villager_fear | JUnit with fakes, gradle build; no game run |
| `net.minecraft.world.attribute` | all classes | 7 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.clock` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world.damagesource` | all classes | 1 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.effect` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.entity` | all classes | 32 hooked types, edges resolved | 1 contracts | system note | request.lantern_moth; request.villager_fear | none |
| `net.minecraft.world.flag` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.food` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world.inventory` | all classes | 5 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.item` | all classes | 42 hooked types, edges resolved | no | system note | request.villager_fear | none |
| `net.minecraft.world.level` | all classes | 143 hooked types, edges resolved | 1 contracts | system note | request.crystal_caves; request.lantern_moth; request.rain_lantern (implemented, hand-authored) | JUnit with fakes, gradle build; no game run |
| `net.minecraft.world.phys` | all classes | 3 hooked types, edges resolved | no | edges only | no | none |
| `net.minecraft.world.scores` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world.ticks` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world.timeline` | all classes | no hooks | no | no | no | none |
| `net.minecraft.world.waypoints` | all classes | no hooks | no | no | no | none |
