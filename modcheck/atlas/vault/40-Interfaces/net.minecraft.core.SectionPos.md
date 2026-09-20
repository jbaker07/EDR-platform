---
type: "interface"
fqcn: "net.minecraft.core.SectionPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.SectionPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `net/minecraft/core/Vec3i`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `of` | `(J)Lnet/minecraft/core/SectionPos;` | exact | invokestatic@11 in `RenderRegionCacheMixin.copyDataForChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@20 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@30 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@41 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@52 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@63 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `sectionToBlockCoord` | `(I)I` | exact | invokestatic@74 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@15 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `x` | `()I` | exact | invokevirtual@47 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@25 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `y` | `()I` | exact | invokevirtual@58 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@36 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `z` | `()I` | exact | invokevirtual@69 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |

## Declared members (19 fields, 56 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SECTION_BITS : I
public static final SECTION_SIZE : I
public static final SECTION_BLOCK_COUNT : I
public static final SECTION_MASK : I
public static final SECTION_HALF_SIZE : I
public static final SECTION_MAX_INDEX : I
private static final PACKED_X_LENGTH : I
private static final PACKED_Y_LENGTH : I
private static final PACKED_Z_LENGTH : I
private static final PACKED_X_MASK : J
private static final PACKED_Y_MASK : J
private static final PACKED_Z_MASK : J
private static final Y_OFFSET : I
private static final Z_OFFSET : I
private static final X_OFFSET : I
private static final RELATIVE_X_SHIFT : I
private static final RELATIVE_Y_SHIFT : I
private static final RELATIVE_Z_SHIFT : I
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private <init>(III)V
public static of(III)Lnet/minecraft/core/SectionPos;
public static of(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/SectionPos;
public static of(Lnet/minecraft/world/level/ChunkPos;I)Lnet/minecraft/core/SectionPos;
public static of(Lnet/minecraft/world/level/entity/EntityAccess;)Lnet/minecraft/core/SectionPos;
public static of(Lnet/minecraft/core/Position;)Lnet/minecraft/core/SectionPos;
public static of(J)Lnet/minecraft/core/SectionPos;
public static bottomOf(Lnet/minecraft/world/level/chunk/ChunkAccess;)Lnet/minecraft/core/SectionPos;
public static offset(JLnet/minecraft/core/Direction;)J
public static offset(JIII)J
public static posToSectionCoord(D)I
public static blockToSectionCoord(I)I
public static blockToSectionCoord(D)I
public static sectionRelative(I)I
public static sectionRelativePos(Lnet/minecraft/core/BlockPos;)S
public static sectionRelativeX(S)I
public static sectionRelativeY(S)I
public static sectionRelativeZ(S)I
public relativeToBlockX(S)I
public relativeToBlockY(S)I
public relativeToBlockZ(S)I
public relativeToBlockPos(S)Lnet/minecraft/core/BlockPos;
public static sectionToBlockCoord(I)I
public static sectionToBlockCoord(II)I
public static x(J)I
public static y(J)I
public static z(J)I
public x()I
public y()I
public z()I
public minBlockX()I
public minBlockY()I
public minBlockZ()I
public maxBlockX()I
public maxBlockY()I
public maxBlockZ()I
public static blockToSection(J)J
public static getZeroNode(II)J
public static getZeroNode(J)J
public static sectionToChunk(J)J
public origin()Lnet/minecraft/core/BlockPos;
public center()Lnet/minecraft/core/BlockPos;
public chunk()Lnet/minecraft/world/level/ChunkPos;
public static asLong(Lnet/minecraft/core/BlockPos;)J
public static asLong(III)J
public asLong()J
public offset(III)Lnet/minecraft/core/SectionPos;
public blocksInside()Ljava/util/stream/Stream;
public static cube(Lnet/minecraft/core/SectionPos;I)Ljava/util/stream/Stream;
public static aroundChunk(Lnet/minecraft/world/level/ChunkPos;III)Ljava/util/stream/Stream;
public static betweenClosedStream(IIIIII)Ljava/util/stream/Stream;
public static aroundAndAtBlockPos(Lnet/minecraft/core/BlockPos;Lit/unimi/dsi/fastutil/longs/LongConsumer;)V
public static aroundAndAtBlockPos(JLit/unimi/dsi/fastutil/longs/LongConsumer;)V
public static aroundAndAtBlockPos(IIILit/unimi/dsi/fastutil/longs/LongConsumer;)V
public synthetic offset(III)Lnet/minecraft/core/Vec3i;
static <clinit>()V
```
