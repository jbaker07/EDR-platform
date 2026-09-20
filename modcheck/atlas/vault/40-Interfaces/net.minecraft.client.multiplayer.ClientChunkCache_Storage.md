---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientChunkCache$Storage"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientChunkCache$Storage

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` final; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `inRange` | `(II)Z` | exact | invokevirtual@11 in `ClientChunkCacheMixin.onUpdateLoadDistance` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (13 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final UPDATE_TRACKING_BUFFERS : I
private final chunks : Ljava/util/concurrent/atomic/AtomicReferenceArray;
private final addedEmptySections : [Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
private final removedEmptySections : [Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
private final addedLoadedChunks : [Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
private final removedLoadedChunks : [Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
private updatingSetsIndex : I
private final chunkRadius : I
private final viewRange : I
private viewCenterX : I
private viewCenterZ : I
private chunkCount : I
final synthetic this$0 : Lnet/minecraft/client/multiplayer/ClientChunkCache;
private <init>(Lnet/minecraft/client/multiplayer/ClientChunkCache;I)V
private getIndex(II)I
private replace(ILnet/minecraft/world/level/chunk/LevelChunk;)V
private drop(ILnet/minecraft/world/level/chunk/LevelChunk;)V
public onSectionEmptinessChanged(IIIZ)V
private onChunkRemoved(Lnet/minecraft/world/level/chunk/LevelChunk;)V
private onChunkAdded(Lnet/minecraft/world/level/chunk/LevelChunk;)V
private refreshEmptySections(Lnet/minecraft/world/level/chunk/LevelChunk;)V
private markSectionEmpty(J)V
private markSectionNotEmpty(J)V
private inRange(II)Z
public getChunk(I)Lnet/minecraft/world/level/chunk/LevelChunk;
private dumpChunks(Ljava/lang/String;)V
```
