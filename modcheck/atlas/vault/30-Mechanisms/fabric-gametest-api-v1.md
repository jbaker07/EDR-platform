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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.gametest.framework.GameTestServer|GameTestServer]] | `isDedicatedServer` | injects_into `@Inject at HEAD` | both | `GameTestServerMixin.isDedicated` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]] | `lambda$load$2(Ljava/util/List;Ljava/util/Map;Ljava/lang/Void;)Lnet/minecraft/core/RegistryAccess$Frozen;` | injects_into `@Inject at HEAD` | both | `RegistryDataLoaderMixin.beforeFreeze` |
| [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]] | `load(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/List;Ljava/util/List;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;` | injects_into `@Inject at HEAD` | both | `RegistryDataLoaderMixin.loadFromResources` |
| [[40-Interfaces/net.minecraft.server.Main|Main]] | `main` | injects_into `@Inject at INVOKE_ASSIGN Lnet/minecraft/server/packs/repository/ServerPacksSource;createPackRepository(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;)Lnet/minecraft/server/packs/repository/PackRepository;` | server | `MainMixin.main` |
| [[40-Interfaces/net.minecraft.server.Main|Main]] | `main` | injects_into `@Inject at INVOKE Lorg/slf4j/Logger;error(Lorg/slf4j/Marker;Ljava/lang/String;Ljava/lang/Throwable;)V` | server | `MainMixin.exitOnError` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager|StructureTemplateManager]] | `<init>` | injects_into `@Inject at INVOKE Lcom/google/common/collect/ImmutableList$Builder;add(Ljava/lang/Object;)Lcom/google/common/collect/ImmutableList$Builder;` | both | `StructureTemplateManagerMixin.addFabricTemplateProvider` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.gametest.v1.CustomTestMethodInvoker|CustomTestMethodInvoker]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.gametest.v1.GameTest|GameTest]] (interface, 12 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
