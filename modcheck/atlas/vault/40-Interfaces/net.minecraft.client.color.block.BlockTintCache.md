---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockTintCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockTintCache

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/function/ToIntFunction;)V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `invalidateAll()V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `invalidateForChunk(II)V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.color.block.BlockTintCache {
    private static final int MAX_CACHE_ENTRIES;
    private final java.lang.ThreadLocal<net.minecraft.client.color.block.BlockTintCache$LatestCacheInfo> latestChunkOnThread;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectLinkedOpenHashMap<net.minecraft.client.color.block.BlockTintCache$CacheData> cache;
    private final java.util.concurrent.locks.ReentrantReadWriteLock lock;
    private final java.util.function.ToIntFunction<net.minecraft.core.BlockPos> source;
    public net.minecraft.client.color.block.BlockTintCache(java.util.function.ToIntFunction<net.minecraft.core.BlockPos>);
    public int getColor(net.minecraft.core.BlockPos);
    public void invalidateForChunk(int, int);
    public void invalidateAll();
    private net.minecraft.client.color.block.BlockTintCache$CacheData findOrCreateChunkCache(int, int);
}
```
