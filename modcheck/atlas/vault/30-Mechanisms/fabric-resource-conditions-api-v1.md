---
type: "mechanism"
module: "fabric-resource-conditions-api-v1"
version: "6.1.5+3434d6d95d"
sha256: "1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-resource-conditions-api-v1

**Version** `6.1.5+3434d6d95d` -- **artifact sha256** `1d7d9bea7e90eacfb57ac9e6d1f09036c8f118fc5ab50328f55c4d0b8519782c`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.resource.conditions.ResourceConditionsImpl"]}`
- mixin configs: `["fabric-resource-conditions-api-v1.mixins.json"]`
- access widener: `fabric-resource-conditions-api-v1.classtweaker`
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.data.DataProvider|DataProvider]].`lambda$static$0` | `(Lit/unimi/dsi/fastutil/objects/Object2IntOpenHashMap;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `DataProviderMixin.fabric_injectResourceConditionsSortOrder` |
| [[40-Interfaces/net.minecraft.resources.RegistryLoadTask_PendingRegistration|RegistryLoadTask$PendingRegistration]].`loadFromResource` | `(Lcom/mojang/serialization/Decoder;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/server/packs/resources/Resource;)Lcom/mojang/datafixers/util/Either;` | name_only | @Inject | INVOKE `Lcom/mojang/serialization/Decoder;parse(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` (exact) | both | 1000 (default) | `RegistryLoadTaskPendingRegistrationMixin.loadFromResource` |
| [[40-Interfaces/net.minecraft.resources.ResourceManagerRegistryLoadTask|ResourceManagerRegistryLoadTask]].`lambda$load$2` | `(Lnet/minecraft/resources/FileToIdConverter;Lnet/minecraft/resources/RegistryOps;Lnet/minecraft/resources/Identifier;Lnet/minecraft/server/packs/resources/Resource;)Lnet/minecraft/resources/RegistryLoadTask$PendingRegistration;` | name_only | @ModifyExpressionValue | NEW `net/minecraft/resources/RegistryLoadTask$PendingRegistration` (exact) | both | 1000 (default) | `ResourceManagerRegistryLoadTaskMixin.load` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`loadResources` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/LayeredRegistryAccess;Ljava/util/List;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/server/permissions/PermissionSet;Ljava/util/concurrent/Executor;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject | HEAD | both | 1000 (default) | `ReloadableServerResourcesMixin.hookReload` |
| [[40-Interfaces/net.minecraft.server.packs.repository.Pack|Pack]].`readPackMetadata` | `(Lnet/minecraft/server/packs/PackLocationInfo;Lnet/minecraft/server/packs/repository/Pack$ResourcesSupplier;Lnet/minecraft/server/packs/metadata/pack/PackFormat;Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server/packs/repository/Pack$Metadata;` | name_only | @ModifyVariable | STORE | both | 1000 (default) | `PackMixin.applyOverlayConditions` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]].`lambda$prepare$0` | `(Ljava/util/Map;Lnet/minecraft/resources/Identifier;Ljava/lang/Object;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `SimpleJsonResourceReloadListenerMixin.skipData` |
| [[40-Interfaces/net.minecraft.server.packs.resources.SimpleJsonResourceReloadListener|SimpleJsonResourceReloadListener]].`prepare` | `?` | ambiguous | @WrapOperation | INVOKE `Lcom/mojang/serialization/Codec;parse(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult;` (inherited_exact) | both | 1000 (default) | `SimpleJsonResourceReloadListenerMixin.applyResourceConditions` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceCondition|ResourceCondition]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditionType|ResourceConditionType]] (interface, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.resource.conditions.v1.ResourceConditions|ResourceConditions]] (class, 17 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
