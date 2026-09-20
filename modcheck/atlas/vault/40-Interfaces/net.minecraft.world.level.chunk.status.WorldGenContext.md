---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.status.WorldGenContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.status.WorldGenContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@33 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@60 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@128 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (6 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final level : Lnet/minecraft/server/level/ServerLevel;
private final generator : Lnet/minecraft/world/level/chunk/ChunkGenerator;
private final structureManager : Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;
private final lightEngine : Lnet/minecraft/server/level/ThreadedLevelLightEngine;
private final mainThreadExecutor : Ljava/util/concurrent/Executor;
private final unsavedListener : Lnet/minecraft/world/level/chunk/LevelChunk$UnsavedListener;
public <init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Lnet/minecraft/server/level/ThreadedLevelLightEngine;Ljava/util/concurrent/Executor;Lnet/minecraft/world/level/chunk/LevelChunk$UnsavedListener;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public level()Lnet/minecraft/server/level/ServerLevel;
public generator()Lnet/minecraft/world/level/chunk/ChunkGenerator;
public structureManager()Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;
public lightEngine()Lnet/minecraft/server/level/ThreadedLevelLightEngine;
public mainThreadExecutor()Ljava/util/concurrent/Executor;
public unsavedListener()Lnet/minecraft/world/level/chunk/LevelChunk$UnsavedListener;
```
