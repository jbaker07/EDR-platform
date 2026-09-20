---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Biome$ClimateSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Biome$ClimateSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(ZFLnet/minecraft/world/level/biome/Biome$TemperatureModifier;F)V` | exact | invokespecial@51 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `<init>` | `(ZFLnet/minecraft/world/level/biome/Biome$TemperatureModifier;F)V` | exact | invokespecial@51 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `<init>` | `(ZFLnet/minecraft/world/level/biome/Biome$TemperatureModifier;F)V` | exact | invokespecial@57 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifie | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `<init>` | `(ZFLnet/minecraft/world/level/biome/Biome$TemperatureModifier;F)V` | exact | invokespecial@51 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `downfall` | `()F` | exact | invokevirtual@48 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `downfall` | `()F` | exact | invokevirtual@48 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `downfall` | `()F` | exact | invokevirtual@54 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifie | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `hasPrecipitation` | `()Z` | exact | invokevirtual@21 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `hasPrecipitation` | `()Z` | exact | invokevirtual@21 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifie | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `hasPrecipitation` | `()Z` | exact | invokevirtual@21 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperature` | `()F` | exact | invokevirtual@22 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperature` | `()F` | exact | invokevirtual@34 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperatureModifie | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperature` | `()F` | exact | invokevirtual@34 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperatureModifier` | `()Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;` | exact | invokevirtual@35 in `BiomeModificationContextImpl$WeatherContextImpl.setPrecipitation` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperatureModifier` | `()Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;` | exact | invokevirtual@35 in `BiomeModificationContextImpl$WeatherContextImpl.setTemperature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperatureModifier` | `()Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;` | exact | invokevirtual@47 in `BiomeModificationContextImpl$WeatherContextImpl.setDownfall` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (5 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final hasPrecipitation : Z
private final temperature : F
private final temperatureModifier : Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;
private final downfall : F
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private <init>(ZFLnet/minecraft/world/level/biome/Biome$TemperatureModifier;F)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public hasPrecipitation()Z
public temperature()F
public temperatureModifier()Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;
public downfall()F
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$4(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;)Ljava/lang/Float;
private static synthetic lambda$static$3(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;)Lnet/minecraft/world/level/biome/Biome$TemperatureModifier;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;)Ljava/lang/Float;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/biome/Biome$ClimateSettings;)Ljava/lang/Boolean;
static <clinit>()V
```
