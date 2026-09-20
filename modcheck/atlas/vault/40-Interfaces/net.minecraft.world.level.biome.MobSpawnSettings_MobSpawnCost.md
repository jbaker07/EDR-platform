---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MobSpawnSettings$MobSpawnCost"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MobSpawnSettings$MobSpawnCost

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(DD)V` | exact | invokespecial@17 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.addMobCharge` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final energyBudget : D
private final charge : D
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(DD)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public energyBudget()D
public charge()D
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/MobSpawnSettings$MobSpawnCost;)Ljava/lang/Double;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/biome/MobSpawnSettings$MobSpawnCost;)Ljava/lang/Double;
static <clinit>()V
```
