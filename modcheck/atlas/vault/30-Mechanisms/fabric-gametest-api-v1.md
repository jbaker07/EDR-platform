---
type: "mechanism"
module: "fabric-gametest-api-v1"
version: "4.0.32+3434d6d95d"
sha256: "1bd8282a95da3822a15d7468d3542f493b9186d214ef10f938b0006319409647"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-gametest-api-v1

**Version** `4.0.32+3434d6d95d` -- **artifact sha256** `1bd8282a95da3822a15d7468d3542f493b9186d214ef10f938b0006319409647`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-registry-sync-v0": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.gametest.FabricGameTestModInitializer"]}`
- mixin configs: `["fabric-gametest-api-v1.mixins.json"]`
- access widener: `fabric-gametest-api-v1.classtweaker`
- mixin classes: 4 found by annotation, 4 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.gametest.framework.GameTestServer|GameTestServer]].`isDedicatedServer` | `()Z` | name_only | @Inject | HEAD | both | 1000 (default) | `GameTestServerMixin.isDedicated` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`lambda$load$2` | `(Ljava/util/List;Ljava/util/Map;Ljava/lang/Void;)Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | @Inject | HEAD | both | 1000 (default) | `RegistryDataLoaderMixin.beforeFreeze` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | exact | @Inject | HEAD | both | 1000 (default) | `RegistryDataLoaderMixin.loadFromResources` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @ModifyExpressionValue | INVOKE `Lnet/minecraft/server/Eula;hasAgreedToEULA()Z` (exact) | server | 1000 (default) | `MainMixin.isEulaAgreedTo` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @Inject | INVOKE_ASSIGN `Lnet/minecraft/server/packs/repository/ServerPacksSource;createPackRepository(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;)Lnet/minecraft/server/packs/repository/PackRepository;` (exact) | server | 1000 (default) | `MainMixin.main` |
| [[40-Interfaces/net.minecraft.server.Main|Main]].`main` | `([Ljava/lang/String;)V` | name_only | @Inject | INVOKE `Lorg/slf4j/Logger;error(Lorg/slf4j/Marker;Ljava/lang/String;Ljava/lang/Throwable;)V` (exact) | server | 1000 (default) | `MainMixin.exitOnError` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager|StructureTemplateManager]].`<init>` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/core/HolderGetter;)V` | name_only | @Inject | INVOKE `Lcom/google/common/collect/ImmutableList$Builder;add(Ljava/lang/Object;)Lcom/google/common/collect/ImmutableList$Builder;` (exact) | both | 1000 (default) | `StructureTemplateManagerMixin.addFabricTemplateProvider` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.gametest.v1.CustomTestMethodInvoker|CustomTestMethodInvoker]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gametest.v1.GameTest|GameTest]] (annotation, 12 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
