---
type: "index"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Game systems (by package)

A package is a system boundary the game's own authors drew. The `hooked` column counts vanilla types in that package that at least one Fabric API module injects into, replaces, wraps, calls or reads -- the modification surface Fabric itself uses. A package with zero hooks is not untouchable; it is one no shipped Fabric module touches.

| package | classes | hooked types | note |
|---|---|---|---|
| `net.minecraft.advancements` | 16 | 4 | [[20-Systems/net.minecraft.advancements|note]] |
| `net.minecraft.advancements.predicates` | 90 | 0 |  |
| `net.minecraft.advancements.triggers` | 95 | 0 |  |
| `net.minecraft.client` | 94 | 9 | [[20-Systems/net.minecraft.client|note]] |
| `net.minecraft.client.animation` | 29 | 0 |  |
| `net.minecraft.client.color` | 31 | 3 | [[20-Systems/net.minecraft.client.color|note]] |
| `net.minecraft.client.data` | 47 | 3 | [[20-Systems/net.minecraft.client.data|note]] |
| `net.minecraft.client.entity` | 4 | 0 |  |
| `net.minecraft.client.gui` | 794 | 53 | [[20-Systems/net.minecraft.client.gui|note]] |
| `net.minecraft.client.input` | 13 | 3 | [[20-Systems/net.minecraft.client.input|note]] |
| `net.minecraft.client.main` | 16 | 0 |  |
| `net.minecraft.client.model` | 282 | 4 | [[20-Systems/net.minecraft.client.model|note]] |
| `net.minecraft.client.multiplayer` | 130 | 15 | [[20-Systems/net.minecraft.client.multiplayer|note]] |
| `net.minecraft.client.particle` | 230 | 6 | [[20-Systems/net.minecraft.client.particle|note]] |
| `net.minecraft.client.player` | 11 | 2 | [[20-Systems/net.minecraft.client.player|note]] |
| `net.minecraft.client.profiling` | 2 | 0 |  |
| `net.minecraft.client.quickplay` | 7 | 0 |  |
| `net.minecraft.client.renderer` | 1006 | 66 | [[20-Systems/net.minecraft.client.renderer|note]] |
| `net.minecraft.client.resources` | 205 | 13 | [[20-Systems/net.minecraft.client.resources|note]] |
| `net.minecraft.client.searchtree` | 10 | 0 |  |
| `net.minecraft.client.server` | 8 | 0 |  |
| `net.minecraft.client.sounds` | 30 | 1 | [[20-Systems/net.minecraft.client.sounds|note]] |
| `net.minecraft.client.telemetry` | 26 | 0 |  |
| `net.minecraft.client.tutorial` | 10 | 0 |  |
| `net.minecraft.client.waypoints` | 2 | 0 |  |
| `net.minecraft.commands` | 30 | 3 | [[20-Systems/net.minecraft.commands|note]] |
| `net.minecraft.commands.arguments` | 157 | 0 |  |
| `net.minecraft.commands.execution` | 27 | 0 |  |
| `net.minecraft.commands.functions` | 10 | 0 |  |
| `net.minecraft.commands.synchronization` | 21 | 0 |  |
| `net.minecraft.core` | 103 | 29 | [[20-Systems/net.minecraft.core|note]] |
| `net.minecraft.core.cauldron` | 4 | 0 |  |
| `net.minecraft.core.component` | 73 | 12 | [[20-Systems/net.minecraft.core.component|note]] |
| `net.minecraft.core.dispenser` | 27 | 0 |  |
| `net.minecraft.core.particles` | 22 | 1 | [[20-Systems/net.minecraft.core.particles|note]] |
| `net.minecraft.core.registries` | 20 | 2 | [[20-Systems/net.minecraft.core.registries|note]] |
| `net.minecraft.data` | 28 | 6 | [[20-Systems/net.minecraft.data|note]] |
| `net.minecraft.data.advancements` | 11 | 0 |  |
| `net.minecraft.data.info` | 12 | 0 |  |
| `net.minecraft.data.loot` | 26 | 2 | [[20-Systems/net.minecraft.data.loot|note]] |
| `net.minecraft.data.metadata` | 2 | 0 |  |
| `net.minecraft.data.recipes` | 28 | 13 | [[20-Systems/net.minecraft.data.recipes|note]] |
| `net.minecraft.data.registries` | 5 | 3 | [[20-Systems/net.minecraft.data.registries|note]] |
| `net.minecraft.data.structures` | 8 | 0 |  |
| `net.minecraft.data.tags` | 37 | 2 | [[20-Systems/net.minecraft.data.tags|note]] |
| `net.minecraft.data.worldgen` | 65 | 0 |  |
| `net.minecraft.gametest` | 2 | 0 |  |
| `net.minecraft.gametest.framework` | 78 | 3 | [[20-Systems/net.minecraft.gametest.framework|note]] |
| `net.minecraft.gizmos` | 20 | 0 |  |
| `net.minecraft.locale` | 4 | 1 | [[20-Systems/net.minecraft.locale|note]] |
| `net.minecraft.nbt` | 97 | 5 | [[20-Systems/net.minecraft.nbt|note]] |
| `net.minecraft.nbt.visitors` | 12 | 0 |  |
| `net.minecraft.network` | 56 | 13 | [[20-Systems/net.minecraft.network|note]] |
| `net.minecraft.network.chat` | 123 | 9 | [[20-Systems/net.minecraft.network.chat|note]] |
| `net.minecraft.network.codec` | 70 | 4 | [[20-Systems/net.minecraft.network.codec|note]] |
| `net.minecraft.network.protocol` | 368 | 30 | [[20-Systems/net.minecraft.network.protocol|note]] |
| `net.minecraft.network.syncher` | 13 | 1 | [[20-Systems/net.minecraft.network.syncher|note]] |
| `net.minecraft.realms` | 6 | 0 |  |
| `net.minecraft.recipebook` | 5 | 0 |  |
| `net.minecraft.references` | 5 | 0 |  |
| `net.minecraft.resources` | 24 | 12 | [[20-Systems/net.minecraft.resources|note]] |
| `net.minecraft.server` | 52 | 12 | [[20-Systems/net.minecraft.server|note]] |
| `net.minecraft.server.advancements` | 4 | 0 |  |
| `net.minecraft.server.bossevents` | 4 | 0 |  |
| `net.minecraft.server.chase` | 5 | 0 |  |
| `net.minecraft.server.commands` | 180 | 4 | [[20-Systems/net.minecraft.server.commands|note]] |
| `net.minecraft.server.dedicated` | 11 | 4 | [[20-Systems/net.minecraft.server.dedicated|note]] |
| `net.minecraft.server.dialog` | 41 | 0 |  |
| `net.minecraft.server.gui` | 6 | 0 |  |
| `net.minecraft.server.jsonrpc` | 95 | 2 | [[20-Systems/net.minecraft.server.jsonrpc|note]] |
| `net.minecraft.server.level` | 71 | 13 | [[20-Systems/net.minecraft.server.level|note]] |
| `net.minecraft.server.network` | 53 | 10 | [[20-Systems/net.minecraft.server.network|note]] |
| `net.minecraft.server.notifications` | 5 | 1 | [[20-Systems/net.minecraft.server.notifications|note]] |
| `net.minecraft.server.packs` | 110 | 25 | [[20-Systems/net.minecraft.server.packs|note]] |
| `net.minecraft.server.permissions` | 18 | 2 | [[20-Systems/net.minecraft.server.permissions|note]] |
| `net.minecraft.server.players` | 31 | 4 | [[20-Systems/net.minecraft.server.players|note]] |
| `net.minecraft.server.rcon` | 10 | 0 |  |
| `net.minecraft.server.waypoints` | 2 | 0 |  |
| `net.minecraft.sounds` | 6 | 0 |  |
| `net.minecraft.stats` | 14 | 0 |  |
| `net.minecraft.tags` | 36 | 6 | [[20-Systems/net.minecraft.tags|note]] |
| `net.minecraft.util` | 187 | 16 | [[20-Systems/net.minecraft.util|note]] |
| `net.minecraft.util.context` | 6 | 0 |  |
| `net.minecraft.util.datafix` | 444 | 2 | [[20-Systems/net.minecraft.util.datafix|note]] |
| `net.minecraft.util.debug` | 34 | 1 | [[20-Systems/net.minecraft.util.debug|note]] |
| `net.minecraft.util.debugchart` | 8 | 0 |  |
| `net.minecraft.util.eventlog` | 11 | 0 |  |
| `net.minecraft.util.filefix` | 73 | 2 | [[20-Systems/net.minecraft.util.filefix|note]] |
| `net.minecraft.util.monitoring` | 3 | 0 |  |
| `net.minecraft.util.parsing` | 59 | 0 |  |
| `net.minecraft.util.profiling` | 103 | 2 | [[20-Systems/net.minecraft.util.profiling|note]] |
| `net.minecraft.util.random` | 8 | 2 | [[20-Systems/net.minecraft.util.random|note]] |
| `net.minecraft.util.thread` | 18 | 1 | [[20-Systems/net.minecraft.util.thread|note]] |
| `net.minecraft.util.valueproviders` | 19 | 0 |  |
| `net.minecraft.util.worldupdate` | 14 | 0 |  |
| `net.minecraft.world` | 36 | 9 | [[20-Systems/net.minecraft.world|note]] |
| `net.minecraft.world.attribute` | 60 | 2 | [[20-Systems/net.minecraft.world.attribute|note]] |
| `net.minecraft.world.clock` | 13 | 0 |  |
| `net.minecraft.world.damagesource` | 13 | 1 | [[20-Systems/net.minecraft.world.damagesource|note]] |
| `net.minecraft.world.effect` | 24 | 0 |  |
| `net.minecraft.world.entity` | 1157 | 17 | [[20-Systems/net.minecraft.world.entity|note]] |
| `net.minecraft.world.flag` | 8 | 2 | [[20-Systems/net.minecraft.world.flag|note]] |
| `net.minecraft.world.food` | 7 | 0 |  |
| `net.minecraft.world.inventory` | 114 | 4 | [[20-Systems/net.minecraft.world.inventory|note]] |
| `net.minecraft.world.item` | 460 | 31 | [[20-Systems/net.minecraft.world.item|note]] |
| `net.minecraft.world.level` | 2176 | 106 | [[20-Systems/net.minecraft.world.level|note]] |
| `net.minecraft.world.phys` | 39 | 3 | [[20-Systems/net.minecraft.world.phys|note]] |
| `net.minecraft.world.scores` | 29 | 0 |  |
| `net.minecraft.world.ticks` | 20 | 0 |  |
| `net.minecraft.world.timeline` | 7 | 0 |  |
| `net.minecraft.world.waypoints` | 24 | 0 |  |
| `com.mojang.datafixers.types` | ? | 2 | [[20-Systems/com.mojang.datafixers.types|note]] |
| `com.mojang.renderpearl.api` | ? | 1 | [[20-Systems/com.mojang.renderpearl.api|note]] |
| `com.mojang.blaze3d.platform` | ? | 3 | [[20-Systems/com.mojang.blaze3d.platform|note]] |
| `net.minecraft` | ? | 5 | [[20-Systems/net.minecraft|note]] |
