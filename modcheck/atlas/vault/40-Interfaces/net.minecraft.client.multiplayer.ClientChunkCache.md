---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientChunkCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientChunkCache

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `net/minecraft/world/level/chunk/ChunkSource`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `drop` | `(Lnet/minecraft/world/level/ChunkPos;)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `replaceWithPacketData` | `(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketDat` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `replaceWithPacketData` | `(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketDat` | name_only | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateViewRadius` | `(I)V` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `level` | `Lnet/minecraft/client/multiplayer/ClientLevel;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |

## Declared members (5 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final emptyChunk : Lnet/minecraft/world/level/chunk/LevelChunk;
private final lightEngine : Lnet/minecraft/world/level/lighting/LevelLightEngine;
private storage : Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;
private final level : Lnet/minecraft/client/multiplayer/ClientLevel;
public <init>(Lnet/minecraft/client/multiplayer/ClientLevel;I)V
public getLightEngine()Lnet/minecraft/world/level/lighting/LevelLightEngine;
private static isValidChunk(Lnet/minecraft/world/level/chunk/LevelChunk;II)Z
public drop(Lnet/minecraft/world/level/ChunkPos;)V
public getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/LevelChunk;
public getLevel()Lnet/minecraft/world/level/BlockGetter;
public replaceBiomes(IILnet/minecraft/network/FriendlyByteBuf;)V
public replaceWithPacketData(IILnet/minecraft/network/protocol/game/ClientboundLevelChunkPacketData;)Lnet/minecraft/world/level/chunk/LevelChunk;
public tick(Ljava/util/function/BooleanSupplier;Z)V
public updateViewCenter(II)V
public updateViewRadius(I)V
private static calculateStorageRange(I)I
public gatherStats()Ljava/lang/String;
public getLoadedChunksCount()I
public onLightUpdate(Lnet/minecraft/world/level/LightLayer;Lnet/minecraft/core/SectionPos;)V
public addedEmptySections()Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
public removedEmptySections()Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
public addedLoadedChunks()Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
public removedLoadedChunks()Lit/unimi/dsi/fastutil/longs/LongOpenHashSet;
public flipUpdateTrackingSets()V
public onSectionEmptinessChanged(IIIZ)V
public synthetic getChunk(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraft/world/level/chunk/ChunkAccess;
static <clinit>()V
```
