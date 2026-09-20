---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkLevel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkLevel

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fullStatus(I)Lnet/minecraft/server/level/FullChunkStatus;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.level.ChunkLevel {
    private static final int FULL_CHUNK_LEVEL;
    private static final int BLOCK_TICKING_LEVEL;
    private static final int ENTITY_TICKING_LEVEL;
    private static final net.minecraft.world.level.chunk.status.ChunkStep FULL_CHUNK_STEP;
    public static final int RADIUS_AROUND_FULL_CHUNK;
    public static final int MAX_LEVEL;
    public net.minecraft.server.level.ChunkLevel();
    public static net.minecraft.world.level.chunk.status.ChunkStatus generationStatus(int);
    public static net.minecraft.world.level.chunk.status.ChunkStatus getStatusAroundFullChunk(int, net.minecraft.world.level.chunk.status.ChunkStatus);
    public static net.minecraft.world.level.chunk.status.ChunkStatus getStatusAroundFullChunk(int);
    public static int byStatus(net.minecraft.world.level.chunk.status.ChunkStatus);
    public static net.minecraft.server.level.FullChunkStatus fullStatus(int);
    public static int byStatus(net.minecraft.server.level.FullChunkStatus);
    public static boolean isEntityTicking(int);
    public static boolean isBlockTicking(int);
    public static boolean isLoaded(int);
    static {};
}
```
