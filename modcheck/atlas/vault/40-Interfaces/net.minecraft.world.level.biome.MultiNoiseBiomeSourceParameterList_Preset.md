---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MultiNoiseBiomeSourceParameterList$Preset

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `usedBiomes` | `()Ljava/util/stream/Stream;` | exact | invokevirtual@3 in `NetherBiomeData.canGenerateInNether` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NETHER` | `Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Pr` | exact | getstatic@0 in `NetherBiomeData.canGenerateInNether` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
private final provider : Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset$SourceProvider;
public static final NETHER : Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;
public static final OVERWORLD : Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;
private static final BY_NAME : Ljava/util/Map;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset$SourceProvider;)V
private static generateOverworldBiomes(Ljava/util/function/Function;)Lnet/minecraft/world/level/biome/Climate$ParameterList;
public usedBiomes()Ljava/util/stream/Stream;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Lnet/minecraft/resources/Identifier;
public provider()Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset$SourceProvider;
private static synthetic lambda$usedBiomes$0(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/resources/ResourceKey;
private static synthetic lambda$generateOverworldBiomes$0(Lcom/google/common/collect/ImmutableList$Builder;Ljava/util/function/Function;Lcom/mojang/datafixers/util/Pair;)V
private static synthetic lambda$static$4(Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1(Lnet/minecraft/resources/Identifier;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$2(Lnet/minecraft/resources/Identifier;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$3(Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
private static synthetic lambda$static$0(Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;)Lnet/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList$Preset;
static <clinit>()V
```
