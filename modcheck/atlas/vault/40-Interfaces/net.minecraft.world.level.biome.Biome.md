---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Biome"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Biome

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@6 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@10 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@10 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@10 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getAttributes` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@7 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getGenerationSettings` | `()Lnet/minecraft/world/level/biome/BiomeGenerationSettings;` | exact | invokevirtual@6 in `BiomeSelectionContext.hasFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getGenerationSettings` | `()Lnet/minecraft/world/level/biome/BiomeGenerationSettings;` | exact | invokevirtual@6 in `BiomeSelectionContext.hasPlacedFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getGenerationSettings` | `()Lnet/minecraft/world/level/biome/BiomeGenerationSettings;` | exact | invokevirtual@60 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getSpecialEffects` | `()Lnet/minecraft/world/level/biome/BiomeSpecialEffects;` | exact | invokevirtual@22 in `BiomeModificationContextImpl$EffectsContextImpl.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@19 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@32 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@45 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@18 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@32 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@45 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@18 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@31 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@51 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@18 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@31 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | getfield@44 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `attributes` | `Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | putfield@34 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `attributes` | `Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | putfield@35 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `attributes` | `Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | putfield@39 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | putfield@54 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | putfield@54 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | putfield@60 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `climateSettings` | `Lnet/minecraft/world/level/biome/Biome$ClimateSettings;` | exact | putfield@54 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (13 fields, 36 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DIRECT_CODEC : Lcom/mojang/serialization/Codec;
public static final NETWORK_CODEC : Lcom/mojang/serialization/Codec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final LIST_CODEC : Lcom/mojang/serialization/Codec;
private static final TEMPERATURE_NOISE : Lnet/minecraft/world/level/levelgen/synth/Noise;
public static final FROZEN_TEMPERATURE_NOISE : Lnet/minecraft/world/level/levelgen/synth/Noise;
public static final BIOME_INFO_NOISE : Lnet/minecraft/world/level/levelgen/synth/Noise;
private static final TEMPERATURE_CACHE_SIZE : I
private final climateSettings : Lnet/minecraft/world/level/biome/Biome$ClimateSettings;
private final generationSettings : Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
private final attributes : Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
private final specialEffects : Lnet/minecraft/world/level/biome/BiomeSpecialEffects;
private final temperatureCache : Ljava/lang/ThreadLocal;
private <init>(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;Lnet/minecraft/world/attribute/EnvironmentAttributeMap;Lnet/minecraft/world/level/biome/BiomeSpecialEffects;Lnet/minecraft/world/level/biome/BiomeGenerationSettings;)V
public hasPrecipitation()Z
public getPrecipitationAt(Lnet/minecraft/core/BlockPos;I)Lnet/minecraft/world/level/biome/Biome$Precipitation;
private getHeightAdjustedTemperature(Lnet/minecraft/core/BlockPos;I)F
private getTemperature(Lnet/minecraft/core/BlockPos;I)F
public shouldFreeze(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
public shouldFreeze(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;Z)Z
public coldEnoughToSnow(Lnet/minecraft/core/BlockPos;I)Z
public warmEnoughToRain(Lnet/minecraft/core/BlockPos;I)Z
public shouldMeltFrozenOceanIcebergSlightly(Lnet/minecraft/core/BlockPos;I)Z
public shouldSnow(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
public getGenerationSettings()Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
public getGrassColor(DD)I
private getBaseGrassColor()I
private getGrassColorFromTexture()I
public getFoliageColor()I
private getFoliageColorFromTexture()I
public getDryFoliageColor()I
private getDryFoliageColorFromTexture()I
public getBaseTemperature()F
public getAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
public getSpecialEffects()Lnet/minecraft/world/level/biome/BiomeSpecialEffects;
public getWaterColor()I
private synthetic lambda$new$0()Lit/unimi/dsi/fastutil/longs/Long2FloatLinkedOpenHashMap;
private static synthetic lambda$static$10()Lnet/minecraft/world/level/levelgen/synth/NoiseStack;
private static synthetic lambda$static$5(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$9(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;Lnet/minecraft/world/attribute/EnvironmentAttributeMap;Lnet/minecraft/world/level/biome/BiomeSpecialEffects;)Lnet/minecraft/world/level/biome/Biome;
private static synthetic lambda$static$8(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/level/biome/BiomeSpecialEffects;
private static synthetic lambda$static$7(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
private static synthetic lambda$static$6(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/level/biome/Biome$ClimateSettings;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$4(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/level/biome/BiomeSpecialEffects;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/biome/Biome;)Lnet/minecraft/world/level/biome/Biome$ClimateSettings;
static <clinit>()V
```
