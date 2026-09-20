---
type: "interface"
fqcn: "net.minecraft.server.level.FullChunkStatus"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.FullChunkStatus

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isOrAfter` | `(Lnet/minecraft/server/level/FullChunkStatus;)Z` | exact | invokevirtual@58 in `ChunkHolderMixin.decreaseLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@20 in `ChunkHolderMixin.decreaseLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@28 in `ChunkHolderMixin.decreaseLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@83 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@94 in `ChunkStatusTasksMixin.onChunkLoad` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/server/level/FullChunkStatus;` | exact | invokestatic@0 in `ChunkHolderMixin.<clinit>` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/server/level/FullChunkStatus;` | exact | invokestatic@0 in `ChunkStatusTasksMixin.<clinit>` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `BLOCK_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@39 in `ChunkHolderMixin.updateFutures$fullToBlockTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `BLOCK_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@48 in `ChunkHolderMixin.updateFutures$fullToBlockTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `BLOCK_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@4 in `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `BLOCK_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@36 in `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `ENTITY_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@39 in `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `ENTITY_TICKING` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@48 in `ChunkHolderMixin.updateFutures$blockTickingToEntityTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@52 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@61 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@4 in `ChunkHolderMixin.updateFutures$fullToBlockTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `FULL` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@36 in `ChunkHolderMixin.updateFutures$fullToBlockTicking` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `INACCESSIBLE` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@6 in `ChunkHolderMixin.<init>` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `INACCESSIBLE` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@17 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `INACCESSIBLE` | `Lnet/minecraft/server/level/FullChunkStatus;` | exact | getstatic@49 in `ChunkHolderMixin.updateFutures$inaccessibleToFull` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (5 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final INACCESSIBLE : Lnet/minecraft/server/level/FullChunkStatus;
public static final FULL : Lnet/minecraft/server/level/FullChunkStatus;
public static final BLOCK_TICKING : Lnet/minecraft/server/level/FullChunkStatus;
public static final ENTITY_TICKING : Lnet/minecraft/server/level/FullChunkStatus;
private static final synthetic $VALUES : [Lnet/minecraft/server/level/FullChunkStatus;
public static values()[Lnet/minecraft/server/level/FullChunkStatus;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/server/level/FullChunkStatus;
private <init>(Ljava/lang/String;I)V
public isOrAfter(Lnet/minecraft/server/level/FullChunkStatus;)Z
private static synthetic $values()[Lnet/minecraft/server/level/FullChunkStatus;
static <clinit>()V
```
