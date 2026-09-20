---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientChunkCache$Storage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientChunkCache$Storage

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `inRange(II)Z` | `` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.client.multiplayer.ClientChunkCache$Storage {
    private static final int UPDATE_TRACKING_BUFFERS;
    private final java.util.concurrent.atomic.AtomicReferenceArray<net.minecraft.world.level.chunk.LevelChunk> chunks;
    private final it.unimi.dsi.fastutil.longs.LongOpenHashSet[] addedEmptySections;
    private final it.unimi.dsi.fastutil.longs.LongOpenHashSet[] removedEmptySections;
    private final it.unimi.dsi.fastutil.longs.LongOpenHashSet[] addedLoadedChunks;
    private final it.unimi.dsi.fastutil.longs.LongOpenHashSet[] removedLoadedChunks;
    private int updatingSetsIndex;
    private final int chunkRadius;
    private final int viewRange;
    private volatile int viewCenterX;
    private volatile int viewCenterZ;
    private int chunkCount;
    final net.minecraft.client.multiplayer.ClientChunkCache this$0;
    private net.minecraft.client.multiplayer.ClientChunkCache$Storage(net.minecraft.client.multiplayer.ClientChunkCache, int);
    private int getIndex(int, int);
    private void replace(int, net.minecraft.world.level.chunk.LevelChunk);
    private void drop(int, net.minecraft.world.level.chunk.LevelChunk);
    public void onSectionEmptinessChanged(int, int, int, boolean);
    private void onChunkRemoved(net.minecraft.world.level.chunk.LevelChunk);
    private void onChunkAdded(net.minecraft.world.level.chunk.LevelChunk);
    private void refreshEmptySections(net.minecraft.world.level.chunk.LevelChunk);
    private void markSectionEmpty(long);
    private void markSectionNotEmpty(long);
    private boolean inRange(int, int);
    public net.minecraft.world.level.chunk.LevelChunk getChunk(int);
    private void dumpChunks(java.lang.String);
}
```
