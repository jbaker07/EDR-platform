---
type: "interface"
fqcn: "net.minecraft.world.level.biome.Climate$ParameterList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.Climate$ParameterList

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@104 in `NetherBiomeData.withModdedBiomeEntries` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `values` | `()Ljava/util/List;` | exact | invokevirtual@18 in `NetherBiomeData.withModdedBiomeEntries` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (2 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final values : Ljava/util/List;
private final index : Lnet/minecraft/world/level/biome/Climate$RTree;
public static codec(Lcom/mojang/serialization/MapCodec;)Lcom/mojang/serialization/Codec;
public <init>(Ljava/util/List;)V
private <init>(Ljava/util/List;I)V
public rebuildWithChildrenPerNode(I)Lnet/minecraft/world/level/biome/Climate$ParameterList;
public values()Ljava/util/List;
public findValue(Lnet/minecraft/world/level/biome/Climate$TargetPoint;)Ljava/lang/Object;
public findValueBruteForce(Lnet/minecraft/world/level/biome/Climate$TargetPoint;)Ljava/lang/Object;
public findValueIndex(Lnet/minecraft/world/level/biome/Climate$TargetPoint;)Ljava/lang/Object;
protected findValueIndex(Lnet/minecraft/world/level/biome/Climate$TargetPoint;Lnet/minecraft/world/level/biome/Climate$DistanceMetric;)Ljava/lang/Object;
private static synthetic lambda$codec$0(Lcom/mojang/serialization/MapCodec;Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
```
