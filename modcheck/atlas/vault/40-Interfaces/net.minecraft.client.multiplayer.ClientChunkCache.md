---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientChunkCache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientChunkCache

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `drop` | `@Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;dro` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `replaceWithPacketData` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `replaceWithPacketData` | `@Inject at NEW net/minecraft/world/level/chunk/LevelChunk` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateViewRadius` | `@Inject at INVOKE Lnet/minecraft/client/multiplayer/ClientChunkCache$Storage;inR` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ClientChunkCache extends net.minecraft.world.level.chunk.ChunkSource {
    private static final org.slf4j.Logger LOGGER;
    private final net.minecraft.world.level.chunk.LevelChunk emptyChunk;
    private final net.minecraft.world.level.lighting.LevelLightEngine lightEngine;
    private volatile net.minecraft.client.multiplayer.ClientChunkCache$Storage storage;
    private final net.minecraft.client.multiplayer.ClientLevel level;
    public net.minecraft.client.multiplayer.ClientChunkCache(net.minecraft.client.multiplayer.ClientLevel, int);
    public net.minecraft.world.level.lighting.LevelLightEngine getLightEngine();
    private static boolean isValidChunk(net.minecraft.world.level.chunk.LevelChunk, int, int);
    public void drop(net.minecraft.world.level.ChunkPos);
    public net.minecraft.world.level.chunk.LevelChunk getChunk(int, int, net.minecraft.world.level.chunk.status.ChunkStatus, boolean);
    public net.minecraft.world.level.BlockGetter getLevel();
    public void replaceBiomes(int, int, net.minecraft.network.FriendlyByteBuf);
    public net.minecraft.world.level.chunk.LevelChunk replaceWithPacketData(int, int, net.minecraft.network.protocol.game.ClientboundLevelChunkPacketData);
    public void tick(java.util.function.BooleanSupplier, boolean);
    public void updateViewCenter(int, int);
    public void updateViewRadius(int);
    private static int calculateStorageRange(int);
    public java.lang.String gatherStats();
    public int getLoadedChunksCount();
    public void onLightUpdate(net.minecraft.world.level.LightLayer, net.minecraft.core.SectionPos);
    public it.unimi.dsi.fastutil.longs.LongOpenHashSet addedEmptySections();
    public it.unimi.dsi.fastutil.longs.LongOpenHashSet removedEmptySections();
    public it.unimi.dsi.fastutil.longs.LongOpenHashSet addedLoadedChunks();
    public it.unimi.dsi.fastutil.longs.LongOpenHashSet removedLoadedChunks();
    public void flipUpdateTrackingSets();
    public void onSectionEmptinessChanged(int, int, int, boolean);
    public net.minecraft.world.level.chunk.ChunkAccess getChunk(int, int, net.minecraft.world.level.chunk.status.ChunkStatus, boolean);
    static {};
}
```
