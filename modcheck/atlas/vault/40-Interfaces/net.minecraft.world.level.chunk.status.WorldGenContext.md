---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.status.WorldGenContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.status.WorldGenContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `level()Lnet/minecraft/server/level/ServerLevel;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.chunk.status.WorldGenContext extends java.lang.Record {
    private final net.minecraft.server.level.ServerLevel level;
    private final net.minecraft.world.level.chunk.ChunkGenerator generator;
    private final net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager structureManager;
    private final net.minecraft.server.level.ThreadedLevelLightEngine lightEngine;
    private final java.util.concurrent.Executor mainThreadExecutor;
    private final net.minecraft.world.level.chunk.LevelChunk$UnsavedListener unsavedListener;
    public net.minecraft.world.level.chunk.status.WorldGenContext(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.chunk.ChunkGenerator, net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager, net.minecraft.server.level.ThreadedLevelLightEngine, java.util.concurrent.Executor, net.minecraft.world.level.chunk.LevelChunk$UnsavedListener);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.server.level.ServerLevel level();
    public net.minecraft.world.level.chunk.ChunkGenerator generator();
    public net.minecraft.world.level.levelgen.structure.templatesystem.StructureTemplateManager structureManager();
    public net.minecraft.server.level.ThreadedLevelLightEngine lightEngine();
    public java.util.concurrent.Executor mainThreadExecutor();
    public net.minecraft.world.level.chunk.LevelChunk$UnsavedListener unsavedListener();
}
```
