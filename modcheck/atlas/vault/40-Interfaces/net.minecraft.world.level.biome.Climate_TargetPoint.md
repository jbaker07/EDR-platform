---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Climate$TargetPoint"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Climate$TargetPoint

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `continentalness` | `()J` | exact | invokevirtual@12 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `depth` | `()J` | exact | invokevirtual@22 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `erosion` | `()J` | exact | invokevirtual@17 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `humidity` | `()J` | exact | invokevirtual@7 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `temperature` | `()J` | exact | invokevirtual@2 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `weirdness` | `()J` | exact | invokevirtual@27 in `NetherBiomes.addNetherBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final temperature : J
private final humidity : J
private final continentalness : J
private final erosion : J
private final depth : J
private final weirdness : J
public <init>(JJJJJJ)V
 toParameterArray()[J
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public temperature()J
public humidity()J
public continentalness()J
public erosion()J
public depth()J
public weirdness()J
```
