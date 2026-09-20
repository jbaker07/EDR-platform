---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.status.ChunkStatus"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.status.ChunkStatus

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `EMPTYLnet/minecraft/world/level/chunk/status/ChunkStatus;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `FULLLnet/minecraft/world/level/chunk/status/ChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (34, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.chunk.status.ChunkStatus {
    public static final int MAX_STRUCTURE_DISTANCE;
    private static final java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types> WORLDGEN_HEIGHTMAPS;
    public static final java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types> FINAL_HEIGHTMAPS;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus EMPTY;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus STRUCTURE_STARTS;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus STRUCTURE_REFERENCES;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus BIOMES;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus TERRAIN;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus FEATURES;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus INITIALIZE_LIGHT;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus LIGHT;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus SPAWN;
    public static final net.minecraft.world.level.chunk.status.ChunkStatus FULL;
    public static final com.mojang.serialization.Codec<net.minecraft.world.level.chunk.status.ChunkStatus> CODEC;
    private final int index;
    private final net.minecraft.world.level.chunk.status.ChunkStatus parent;
    private final net.minecraft.world.level.chunk.status.ChunkType chunkType;
    private final java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types> heightmapsAfter;
    private static net.minecraft.world.level.chunk.status.ChunkStatus register(java.lang.String, net.minecraft.world.level.chunk.status.ChunkStatus, java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types>, net.minecraft.world.level.chunk.status.ChunkType);
    public static java.util.List<net.minecraft.world.level.chunk.status.ChunkStatus> getStatusList();
    protected net.minecraft.world.level.chunk.status.ChunkStatus(net.minecraft.world.level.chunk.status.ChunkStatus, java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types>, net.minecraft.world.level.chunk.status.ChunkType);
    public int getIndex();
    public net.minecraft.world.level.chunk.status.ChunkStatus getParent();
    public net.minecraft.world.level.chunk.status.ChunkType getChunkType();
    public static net.minecraft.world.level.chunk.status.ChunkStatus byName(java.lang.String);
    public java.util.EnumSet<net.minecraft.world.level.levelgen.Heightmap$Types> heightmapsAfter();
    public boolean isOrAfter(net.minecraft.world.level.chunk.status.ChunkStatus);
    public boolean isAfter(net.minecraft.world.level.chunk.status.ChunkStatus);
    public boolean isOrBefore(net.minecraft.world.level.chunk.status.ChunkStatus);
    public boolean isBefore(net.minecraft.world.level.chunk.status.ChunkStatus);
    public static net.minecraft.world.level.chunk.status.ChunkStatus max(net.minecraft.world.level.chunk.status.ChunkStatus, net.minecraft.world.level.chunk.status.ChunkStatus);
    public java.lang.String toString();
    public java.lang.String getName();
    static {};
}
```
