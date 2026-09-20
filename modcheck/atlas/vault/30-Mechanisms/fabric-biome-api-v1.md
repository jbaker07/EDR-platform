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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] | `<init>` | injects_into `@Inject at RETURN` | both | `MinecraftServerMixin.finalizeWorldGen` |
| [[40-Interfaces/net.minecraft.world.level.biome.BiomeSource|BiomeSource]] | `possibleBiomes` | wraps `@Redirect at INVOKE Ljava/util/function/Supplier;get()Ljava/lang/Object;` | both | `BiomeSourceMixin.getBiomes` |
| [[40-Interfaces/net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList_Preset_1|MultiNoiseBiomeSourceParameterList$Preset$1]] | `apply` | injects_into `@Inject at RETURN` | both | `NetherBiomePresetMixin.apply` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] | `<clinit>` | injects_into `@Inject at TAIL` | both | `TheEndBiomeSourceMixin.modifyCodec` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] | `<init>` | injects_into `@Inject at RETURN` | both | `TheEndBiomeSourceMixin.init` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] | `create` | injects_into `@Inject at HEAD` | both | `TheEndBiomeSourceMixin.rememberLookup` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] | `create` | injects_into `@Inject at TAIL` | both | `TheEndBiomeSourceMixin.clearLookup` |
| [[40-Interfaces/net.minecraft.world.level.biome.TheEndBiomeSource|TheEndBiomeSource]] | `getNoiseBiome` | injects_into `@Inject at RETURN` | both | `TheEndBiomeSourceMixin.getWeightedEndBiome` |
| [[40-Interfaces/net.minecraft.world.level.levelgen.RandomState|RandomState]] | `createClimateSampler` | injects_into `@Inject at RETURN` | both | `RandomStateMixin.setSeed` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModification|BiomeModification]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModificationContext|BiomeModificationContext]] (interface, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeModifications|BiomeModifications]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeSelectionContext|BiomeSelectionContext]] (interface, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.BiomeSelectors|BiomeSelectors]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.ModificationPhase|ModificationPhase]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.NetherBiomes|NetherBiomes]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.biome.v1.TheEndBiomes|TheEndBiomes]] (class, 5 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
