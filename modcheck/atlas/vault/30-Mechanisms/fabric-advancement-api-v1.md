---
type: "mechanism"
module: "fabric-advancement-api-v1"
version: "1.0.0+38c7a2d55d"
sha256: "89e2094ca63a5e3e4687ebd5530eb56a3f051fc568b7ab60243b69a6c8c3e74e"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-advancement-api-v1

**Version** `1.0.0+38c7a2d55d` -- **artifact sha256** `89e2094ca63a5e3e4687ebd5530eb56a3f051fc568b7ab60243b69a6c8c3e74e`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-advancement-api-v1.mixins.json"]`
- access widener: `fabric-advancement-api-v1.classtweaker`
- mixin classes: 3 found by annotation, 3 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.ALL_LOADED|AdvancementEvents.ALL_LOADED]]
- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.MODIFY|AdvancementEvents.MODIFY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents.REPLACE|AdvancementEvents.REPLACE]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.advancements.Advancement_Builder|Advancement$Builder]].`addCriterion` | `(Ljava/lang/String;Lnet/minecraft/advancements/triggers/Criterion;)Lnet/minecraft/advancements/Advancement$Builder;` | exact | @Inject | HEAD | both | 1000 (default) | `AdvancementBuilderMixin.addModifiedCriterion` |
| [[40-Interfaces/net.minecraft.advancements.Advancement_Builder|Advancement$Builder]].`build` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/advancements/AdvancementHolder;` | exact | @ModifyReceiver | INVOKE `Lcom/google/common/collect/ImmutableMap$Builder;buildOrThrow()Lcom/google/common/collect/ImmutableMap;` (exact) | both | 1000 (default) | `AdvancementBuilderMixin.useModifiedCriteria` |
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]].`lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;loadFromResource(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/server/packs/resources/Resource;)Lcom/mojang/datafixers/util/Either;` (exact) | both | 1000 (default) | `ResourceManagerRegistryLoadTaskMixin.modifyAdvancement` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerRegistries|ReloadableServerRegistries]].`reload` | `(Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/resources/RegistryDataLoader;load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` (exact) | both | 1000 (default) | `ReloadableServerRegistriesMixin.modifyAdvancements` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.AdvancementEvents|AdvancementEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.AdvancementSource|AdvancementSource]] (enum, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.advancement.v1.FabricAdvancementBuilder|FabricAdvancementBuilder]] (interface, 11 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
