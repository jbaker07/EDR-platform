---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.status.ChunkStatusTasks"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.status.ChunkStatusTasks

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `lambda$full$0` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.chunk.status.ChunkStatusTasks {
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.world.level.chunk.status.ChunkStatusTasks();
    private static boolean isLighted(net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> passThrough(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> generateStructureStarts(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> loadStructureStarts(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> generateStructureReferences(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> generateBiomes(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    private static java.util.Set<net.minecraft.core.Holder<net.minecraft.world.level.biome.Biome>> collectPossibleBiomes(net.minecraft.server.level.WorldGenRegion, int);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> buildTerrain(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> generateFeatures(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> initializeLight(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> light(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> generateSpawn(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    public static java.util.concurrent.CompletableFuture<net.minecraft.world.level.chunk.ChunkAccess> full(net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.world.level.chunk.status.ChunkStep, net.minecraft.util.StaticCache2D<net.minecraft.server.level.GenerationChunkHolder>, net.minecraft.world.level.chunk.ChunkAccess);
    private static void postLoadProtoChunk(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.storage.ValueInput$ValueInputList);
    private static net.minecraft.world.level.chunk.ChunkAccess lambda$full$0(net.minecraft.world.level.chunk.ChunkAccess, net.minecraft.world.level.chunk.status.WorldGenContext, net.minecraft.server.level.GenerationChunkHolder);
    private static void lambda$full$1(net.minecraft.world.level.chunk.ChunkAccess, net.minecraft.server.level.ServerLevel, net.minecraft.world.level.chunk.ProtoChunk, net.minecraft.world.level.chunk.LevelChunk);
    private static net.minecraft.world.level.chunk.ChunkAccess lambda$buildTerrain$0(net.minecraft.world.level.chunk.ChunkAccess);
    static {};
}
```
