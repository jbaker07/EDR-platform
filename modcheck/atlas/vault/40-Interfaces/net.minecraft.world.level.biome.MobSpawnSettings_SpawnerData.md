---
type: "interface"
fqcn: "net.minecraft.world.level.biome.MobSpawnSettings$SpawnerData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.MobSpawnSettings$SpawnerData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/util/valueprovi` | exact | invokespecial@34 in `BiomeModifications.lambda$addSpawn$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/world/entity/EntityType;` | exact | invokevirtual@1 in `BiomeModificationContext$MobSpawnSettingsContext.lambda$removeSpawnsOf | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `type` | `()Lnet/minecraft/world/entity/EntityType;` | exact | invokevirtual@99 in `BiomeSelectors.lambda$spawnsOneOf$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/world/entity/EntityType;
private final count : Lnet/minecraft/util/valueproviders/IntProvider;
public static final CODEC : Lcom/mojang/serialization/MapCodec;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/util/valueproviders/IntProvider;)V
public toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public type()Lnet/minecraft/world/entity/EntityType;
public count()Lnet/minecraft/util/valueproviders/IntProvider;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
