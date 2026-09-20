---
type: "interface"
fqcn: "net.minecraft.world.level.ChunkPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.ChunkPos

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `containing` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/ChunkPos;` | exact | invokestatic@9 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `pack` | `(Lnet/minecraft/core/BlockPos;)J` | exact | invokestatic@28 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@5 in `AttachmentTargetInfo$ChunkTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@47 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@3 in `ClientChunkCacheMixin.onUpdateLoadDistance` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@38 in `ClientLevelMixin.onResetChunkColor` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@12 in `AttachmentTargetInfo$ChunkTarget.getTarget` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@54 in `AttachmentTargetInfo$ChunkTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@8 in `ClientChunkCacheMixin.onUpdateLoadDistance` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@42 in `ClientLevelMixin.onResetChunkColor` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (16 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final x : I
private final z : I
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final SAFETY_MARGIN : I
public static final INVALID_CHUNK_POS : J
public static final ZERO : Lnet/minecraft/world/level/ChunkPos;
private static final COORD_BITS : J
private static final COORD_MASK : J
private static final REGION_BITS : I
public static final REGION_SIZE : I
private static final REGION_MASK : I
public static final REGION_MAX_INDEX : I
private static final HASH_A : I
private static final HASH_C : I
private static final HASH_Z_XOR : I
public <init>(II)V
public static containing(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/ChunkPos;
public static unpack(J)Lnet/minecraft/world/level/ChunkPos;
public static minFromRegion(II)Lnet/minecraft/world/level/ChunkPos;
public static maxFromRegion(II)Lnet/minecraft/world/level/ChunkPos;
public isValid()Z
public static isValid(II)Z
public pack()J
public static pack(II)J
public static fromSectionNode(J)J
public static pack(Lnet/minecraft/core/BlockPos;)J
public static getX(J)I
public static getZ(J)I
public hashCode()I
public static hash(II)I
public getMiddleBlockX()I
public getMiddleBlockZ()I
public getMinBlockX()I
public getMinBlockZ()I
public getMaxBlockX()I
public getMaxBlockZ()I
public getRegionX()I
public getRegionZ()I
public static getRegionX(J)I
public static getRegionZ(J)I
public getRegionLocalX()I
public getRegionLocalZ()I
public getBlockAt(III)Lnet/minecraft/core/BlockPos;
public getBlockX(I)I
public getBlockZ(I)I
public getMiddleBlockPosition(I)Lnet/minecraft/core/BlockPos;
public contains(Lnet/minecraft/core/BlockPos;)Z
public toString()Ljava/lang/String;
public getWorldPosition()Lnet/minecraft/core/BlockPos;
public getChessboardDistance(Lnet/minecraft/world/level/ChunkPos;)I
public getChessboardDistance(II)I
public distanceSquared(Lnet/minecraft/world/level/ChunkPos;)I
public distanceSquared(J)I
private distanceSquared(II)I
public static rangeClosed(Lnet/minecraft/world/level/ChunkPos;I)Ljava/util/stream/Stream;
public static rangeClosed(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/ChunkPos;)Ljava/util/stream/Stream;
public final equals(Ljava/lang/Object;)Z
public x()I
public z()I
private static synthetic lambda$static$2(Lnet/minecraft/world/level/ChunkPos;)Ljava/util/stream/IntStream;
private static synthetic lambda$static$0(Ljava/util/stream/IntStream;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1([I)Lnet/minecraft/world/level/ChunkPos;
static <clinit>()V
```
