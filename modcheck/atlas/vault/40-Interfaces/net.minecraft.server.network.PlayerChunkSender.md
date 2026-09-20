---
type: "interface"
fqcn: "net.minecraft.server.network.PlayerChunkSender"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.PlayerChunkSender

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `sendNextChunks` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (11 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final MIN_CHUNKS_PER_TICK : F
public static final MAX_CHUNKS_PER_TICK : F
private static final START_CHUNKS_PER_TICK : F
private static final MAX_UNACKNOWLEDGED_BATCHES : I
private final pendingChunks : Lit/unimi/dsi/fastutil/longs/LongSet;
private final memoryConnection : Z
private desiredChunksPerTick : F
private batchQuota : F
private unacknowledgedBatches : I
private maxUnacknowledgedBatches : I
public <init>(Z)V
public markChunkPendingToSend(Lnet/minecraft/world/level/chunk/LevelChunk;)V
public dropChunk(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/level/ChunkPos;)V
public sendNextChunks(Lnet/minecraft/server/level/ServerPlayer;)V
private static sendChunk(Lnet/minecraft/server/network/ServerGamePacketListenerImpl;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/LevelChunk;)V
private collectChunksToSend(Lnet/minecraft/server/level/ChunkMap;Lnet/minecraft/world/level/ChunkPos;)Ljava/util/List;
public onChunkBatchReceivedByClient(F)V
public isPending(J)Z
private static synthetic lambda$collectChunksToSend$0(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/LevelChunk;)I
static <clinit>()V
```
