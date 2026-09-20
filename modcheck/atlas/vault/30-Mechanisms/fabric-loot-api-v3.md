---
type: "mechanism"
module: "fabric-loot-api-v3"
version: "4.0.8+4068fd645d"
sha256: "569540023c6d19e4b4854e14ea4bb99aed401946d5470f761ac1f1388cc7de2c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-loot-api-v3

**Version** `4.0.8+4068fd645d` -- **artifact sha256** `569540023c6d19e4b4854e14ea4bb99aed401946d5470f761ac1f1388cc7de2c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-loot-api-v3.mixins.json"]`
- access widener: `fabric-loot-api-v3.classtweaker`
- mixin classes: 7 found by annotation, 7 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.ALL_LOADED|LootTableEvents.ALL_LOADED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.MODIFY|LootTableEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.MODIFY_DROPS|LootTableEvents.MODIFY_DROPS]]
- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.REPLACE|LootTableEvents.REPLACE]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]].`lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;loadFromResource(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/server/packs/resources/Resource;)Lcom/mojang/datafixers/util/Either;` (exact) | both | 1000 (default) | `ResourceManagerRegistryLoadTaskMixin.modifyLootTable` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]].`reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `ReloadableServerRegistriesMixin.modifyLootTables` |
| [[40-Interfaces/net.minecraft.world.level.storage.loot.LootTable|LootTable]].`getRandomItemsRaw` | `(Lnet/minecraft/world/level/storage/loot/LootContext;Ljava/util/function/Consumer;)V` | exact | @WrapMethod | - | both | 3000 | `LootTableMixin.fabric$modifyDrops` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.FabricLootPoolBuilder|FabricLootPoolBuilder]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.FabricLootTableBuilder|FabricLootTableBuilder]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.LootTableEvents|LootTableEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.loot.v3.LootTableSource|LootTableSource]] (enum, 7 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
