---
type: "interface"
fqcn: "net.minecraft.server.level.ServerPlayer$RespawnConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerPlayer$RespawnConfig

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `respawnData` | `()Lnet/minecraft/world/level/storage/LevelData$RespawnData;` | exact | invokevirtual@11 in `ServerPlayerMixin.onSetSpawnPoint` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (3 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final respawnData : Lnet/minecraft/world/level/storage/LevelData$RespawnData;
private final forced : Z
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lnet/minecraft/world/level/storage/LevelData$RespawnData;Z)V
private static getDimensionOrDefault(Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;)Lnet/minecraft/resources/ResourceKey;
public isSamePosition(Lnet/minecraft/server/level/ServerPlayer$RespawnConfig;)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public respawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public forced()Z
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
