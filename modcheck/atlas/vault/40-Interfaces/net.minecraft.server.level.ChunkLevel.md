---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkLevel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkLevel

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fullStatus` | `(I)Lnet/minecraft/server/level/FullChunkStatus;` | exact | invokestatic@4 in `ChunkHolderMixin.decreaseLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (6 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final FULL_CHUNK_LEVEL : I
private static final BLOCK_TICKING_LEVEL : I
private static final ENTITY_TICKING_LEVEL : I
private static final FULL_CHUNK_STEP : Lnet/minecraft/world/level/chunk/status/ChunkStep;
public static final RADIUS_AROUND_FULL_CHUNK : I
public static final MAX_LEVEL : I
public <init>()V
public static generationStatus(I)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static getStatusAroundFullChunk(ILnet/minecraft/world/level/chunk/status/ChunkStatus;)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static getStatusAroundFullChunk(I)Lnet/minecraft/world/level/chunk/status/ChunkStatus;
public static byStatus(Lnet/minecraft/world/level/chunk/status/ChunkStatus;)I
public static fullStatus(I)Lnet/minecraft/server/level/FullChunkStatus;
public static byStatus(Lnet/minecraft/server/level/FullChunkStatus;)I
public static isEntityTicking(I)Z
public static isBlockTicking(I)Z
public static isLoaded(I)Z
static <clinit>()V
```
