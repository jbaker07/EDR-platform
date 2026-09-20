---
type: "mechanism"
module: "fabric-biome-api-v1"
version: "20.0.9+74ed1ea55d"
sha256: "ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-biome-api-v1

**Version** `20.0.9+74ed1ea55d` -- **artifact sha256** `ce9698da6dd365c6ee59ad5f5e85602cefaaddaebf6d80fad76925e84cda5176`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=1.16.2"}`
- entrypoints: `null`
- mixin configs: `["fabric-biome-api-v1.mixins.json"]`
- access widener: `fabric-biome-api-v1.classtweaker`
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]].`<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/level/progress/LevelLoadListener;ZLnet/minecraft/server/notifications/NotificationManager;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `MinecraftServerMixin.finalizeWorldGen` |
| [[40-Interfaces/net.minecraft.world.level.biome.BiomeSource|BiomeSource]].`possibleBiomes` | `()Ljava/util/Set;` | name_only | @Redirect | INVOKE `Ljava/util/function/Supplier;get()Ljava/lang/Object;` (exact) | both | 1000 (default) | `BiomeSourceMixin.getBiomes` |
| [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset_1|MultiNoiseBiomeSourceParameterList$Preset$1]].`apply` | `(Ljava/util/function/Function;)Lnet/minecraft/world/level/biome/Climate$ParameterList;` | name_only | @Inject | RETURN | both | 1000 (default) | `NetherBiomePresetMixin.apply` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]].`<clinit>` | `()V` | exact | @Inject | TAIL | both | 1000 (default) | `TheEndBiomeSourceMixin.modifyCodec` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]].`<init>` | `(Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `TheEndBiomeSourceMixin.init` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]].`create` | `(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/TheEndBiomeSource;` | name_only | @Inject | HEAD | both | 1000 (default) | `TheEndBiomeSourceMixin.rememberLookup` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]].`create` | `(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/TheEndBiomeSource;` | name_only | @Inject | TAIL | both | 1000 (default) | `TheEndBiomeSourceMixin.clearLookup` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]].`getNoiseBiome` | `(IIILnet/minecraft/world/level/biome/Climate$Sampler;)Lnet/minecraft/core/Holder;` | name_only | @Inject | RETURN | both | 1000 (default) | `TheEndBiomeSourceMixin.getWeightedEndBiome` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.RandomState|RandomState]].`createClimateSampler` | `(Lnet/minecraft/world/level/levelgen/densityfunction/SamplerContext;)Lnet/minecraft/world/level/biome/Climate$Sampler;` | name_only | @Inject | RETURN | both | 1000 (default) | `RandomStateMixin.setSeed` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModification|BiomeModification]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModificationContext|BiomeModificationContext]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModifications|BiomeModifications]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext|BiomeSelectionContext]] (interface, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeSelectors|BiomeSelectors]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.ModificationPhase|ModificationPhase]] (enum, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.NetherBiomes|NetherBiomes]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.TheEndBiomes|TheEndBiomes]] (class, 5 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
