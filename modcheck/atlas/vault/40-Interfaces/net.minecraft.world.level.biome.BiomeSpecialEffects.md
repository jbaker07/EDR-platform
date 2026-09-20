---
type: "interface"
fqcn: "net.minecraft.world.level.biome.BiomeSpecialEffects"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.BiomeSpecialEffects

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| writes | `dryFoliageColorOverride` | `Ljava/util/Optional;` | exact | putfield@11 in `BiomeModificationContextImpl$EffectsContextImpl.setDryFoliageColorOverride | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `foliageColorOverride` | `Ljava/util/Optional;` | exact | putfield@11 in `BiomeModificationContextImpl$EffectsContextImpl.setFoliageColorOverride` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `grassColorModifier` | `Lnet/minecraft/world/level/biome/BiomeSpecialEffects$GrassColorModifie` | exact | putfield@11 in `BiomeModificationContextImpl$EffectsContextImpl.setGrassColorModifier` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `grassColorOverride` | `Ljava/util/Optional;` | exact | putfield@11 in `BiomeModificationContextImpl$EffectsContextImpl.setGrassColorOverride` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `waterColor` | `I` | exact | putfield@5 in `BiomeModificationContextImpl$EffectsContextImpl.setWaterColor` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final waterColor : I
private final foliageColorOverride : Ljava/util/Optional;
private final dryFoliageColorOverride : Ljava/util/Optional;
private final grassColorOverride : Ljava/util/Optional;
private final grassColorModifier : Lnet/minecraft/world/level/biome/BiomeSpecialEffects$GrassColorModifier;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(ILjava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Lnet/minecraft/world/level/biome/BiomeSpecialEffects$GrassColorModifier;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public waterColor()I
public foliageColorOverride()Ljava/util/Optional;
public dryFoliageColorOverride()Ljava/util/Optional;
public grassColorOverride()Ljava/util/Optional;
public grassColorModifier()Lnet/minecraft/world/level/biome/BiomeSpecialEffects$GrassColorModifier;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
