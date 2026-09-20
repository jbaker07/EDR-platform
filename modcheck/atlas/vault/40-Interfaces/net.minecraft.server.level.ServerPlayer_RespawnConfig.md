---
type: "interface"
fqcn: "net.minecraft.server.level.ServerPlayer$RespawnConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerPlayer$RespawnConfig

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `respawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.server.level.ServerPlayer$RespawnConfig extends java.lang.Record {
    private final net.minecraft.world.level.storage.LevelData$RespawnData respawnData;
    private final boolean forced;
    public static final com.mojang.serialization.Codec<net.minecraft.server.level.ServerPlayer$RespawnConfig> CODEC;
    public net.minecraft.server.level.ServerPlayer$RespawnConfig(net.minecraft.world.level.storage.LevelData$RespawnData, boolean);
    private static net.minecraft.resources.ResourceKey<net.minecraft.world.level.Level> getDimensionOrDefault(net.minecraft.server.level.ServerPlayer$RespawnConfig);
    public boolean isSamePosition(net.minecraft.server.level.ServerPlayer$RespawnConfig);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.level.storage.LevelData$RespawnData respawnData();
    public boolean forced();
    private static com.mojang.datafixers.kinds.App lambda$static$0(com.mojang.serialization.codecs.RecordCodecBuilder$Instance);
    static {};
}
```
