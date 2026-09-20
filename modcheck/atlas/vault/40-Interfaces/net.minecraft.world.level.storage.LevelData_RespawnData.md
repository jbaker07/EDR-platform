---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelData$RespawnData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelData$RespawnData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `pos` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@14 in `ServerPlayerMixin.onSetSpawnPoint` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (7 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final globalPos : Lnet/minecraft/core/GlobalPos;
private final yaw : F
private final pitch : F
public static final DEFAULT : Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/core/GlobalPos;FF)V
public static of(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/BlockPos;FF)Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public dimension()Lnet/minecraft/resources/ResourceKey;
public pos()Lnet/minecraft/core/BlockPos;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public globalPos()Lnet/minecraft/core/GlobalPos;
public yaw()F
public pitch()F
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
