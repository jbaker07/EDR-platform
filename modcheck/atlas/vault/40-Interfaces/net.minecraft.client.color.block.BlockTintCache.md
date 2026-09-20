---
type: "interface"
fqcn: "net.minecraft.client.color.block.BlockTintCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.color.block.BlockTintCache

System: [[20-Systems/net.minecraft.client.color|net.minecraft.client.color]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/function/ToIntFunction;)V` | exact | invokespecial@11 in `ClientLevelMixin.lambda$new$0` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `invalidateAll` | `()V` | exact | invokevirtual@35 in `ClientLevelMixin.onReloadColor` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `invalidateForChunk` | `(II)V` | exact | invokevirtual@45 in `ClientLevelMixin.onResetChunkColor` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (5 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MAX_CACHE_ENTRIES : I
private final latestChunkOnThread : Ljava/lang/ThreadLocal;
private final cache : Lit/unimi/dsi/fastutil/longs/Long2ObjectLinkedOpenHashMap;
private final lock : Ljava/util/concurrent/locks/ReentrantReadWriteLock;
private final source : Ljava/util/function/ToIntFunction;
public <init>(Ljava/util/function/ToIntFunction;)V
public getColor(Lnet/minecraft/core/BlockPos;)I
public invalidateForChunk(II)V
public invalidateAll()V
private findOrCreateChunkCache(II)Lnet/minecraft/client/color/block/BlockTintCache$CacheData;
```
