---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.status.ChunkStatus"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.status.ChunkStatus

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `EMPTY` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@17 in `ChunkAccessMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@87 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@15 in `AttachmentTargetInfo$ChunkTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@1 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@40 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@27 in `ChunkHolderMixin.updateFutures$fullToBlockTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@27 in `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/world/level/chunk/status/ChunkStatus;` | exact | getstatic@76 in `ChunkHolderMixin.decreaseLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (18 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_STRUCTURE_DISTANCE : I
private static final WORLDGEN_HEIGHTMAPS : Ljava/util/EnumSet;
public static final FINAL_HEIGHTMAPS : Ljava/util/EnumSet;
public static final EMPTY : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final STRUCTURE_STARTS : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final STRUCTURE_REFERENCES : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final BIOMES : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final TERRAIN : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final FEATURES : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final INITIALIZE_LIGHT : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final LIGHT : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final SPAWN : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final FULL : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final index : I
private final parent : Lnet/minecraft/world/level/chunk/status/ChunkStatus;
private final chunkType : Lnet/minecraft/world/level/chunk/status/ChunkType;
private final heightmapsAfter : Ljava/util/EnumSet;
private static register(Ljava/lang/String;Lnet/minecraft/world/level/chunk/status/ChunkStatus;Ljava/util/EnumSet;Lnet/minecraft/world/level/chunk/status/ChunkType;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static getStatusList()Ljava/util/List;
protected <init>(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Ljava/util/EnumSet;Lnet/minecraft/world/level/chunk/status/ChunkType;)V
public getIndex()I
public getParent()Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public getChunkType()Lnet/minecraft/world/level/chunk/status/ChunkType;
public static byName(Ljava/lang/String;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public heightmapsAfter()Ljava/util/EnumSet;
public isOrAfter(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
public isAfter(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
public isOrBefore(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
public isBefore(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Z
public static max(Lnet/minecraft/world/level/chunk/status/ChunkStatus;Lnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public toString()Ljava/lang/String;
public getName()Ljava/lang/String;
static <clinit>()V
```
