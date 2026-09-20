---
type: "interface"
fqcn: "net.minecraft.core.BlockPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.BlockPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `net/minecraft/core/Vec3i`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `asLong` | `()J` | exact | invokevirtual@223 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `asLong` | `()J` | exact | invokevirtual@16 in `RenderSectionRegionMixin.getBlockEntityRenderData` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `containing` | `(DDD)Lnet/minecraft/core/BlockPos;` | exact | invokestatic@8 in `BlockMarkerMixin.getParticleMaterialProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `containing` | `(DDD)Lnet/minecraft/core/BlockPos;` | exact | invokestatic@25 in `HopperBlockEntityMixin.hookExtract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `containing` | `(Lnet/minecraft/core/Position;)Lnet/minecraft/core/BlockPos;` | exact | invokestatic@44 in `CommandPermissionContext.get` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getX` | `()I` | inherited_exact | invokevirtual@131 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getX` | `()I` | inherited_exact | invokevirtual@140 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getY` | `()I` | inherited_exact | invokevirtual@150 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getY` | `()I` | inherited_exact | invokevirtual@160 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getZ` | `()I` | inherited_exact | invokevirtual@170 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getZ` | `()I` | inherited_exact | invokevirtual@180 in `RenderRegionCacheMixin.mapChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `immutable` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@47 in `BlockApiCacheImpl.<init>` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `immutable` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@5 in `ServerLevelMixin.fabric_registerCache` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `immutable` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@41 in `LevelExtractorMixin.captureViewBlockingPosition` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `immutable` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@6 in `CauldronStorage.get` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `immutable` | `()Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@20 in `ComposterWrapper.get` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `offset` | `(III)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@4 in `InteractionEventsRouter.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `offset` | `(III)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@56 in `InteractionEventsRouter.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `offset` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@44 in `EnchantmentMenuMixin.addEnchantingPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `offset` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@53 in `EnchantmentMenuMixin.addEnchantingPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@28 in `FlowingFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@16 in `LavaFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@63 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@22 in `CrafterBlockMixin.transferOrSpawnStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@33 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;` | exact | invokevirtual@18 in `HopperBlockEntityMixin.hookInsert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toShortString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@47 in `AttachmentTargetInfo$BlockEntityTarget.appendDebugInformation` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `toShortString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@21 in `DebugMessages.forGlobalPos` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@195 in `AoCalculator.compute` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@0 in `AttachmentTargetInfo$BlockEntityTarget.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@47 in `ExtendedBlockParticleOptionStreamCodec.decode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `STREAM_CODEC` | `Lnet/minecraft/network/codec/StreamCodec;` | exact | getstatic@44 in `ExtendedBlockParticleOptionStreamCodec.encode` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| reads | `ZERO` | `Lnet/minecraft/core/BlockPos;` | exact | getstatic@55 in `BlockApiLookupImpl.registerSelf` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| reads | `ZERO` | `Lnet/minecraft/core/BlockPos;` | exact | getstatic@12 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `ZERO` | `Lnet/minecraft/core/BlockPos;` | exact | getstatic@42 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (12 fields, 89 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final ZERO : Lnet/minecraft/core/BlockPos;
public static final PACKED_HORIZONTAL_LENGTH : I
public static final PACKED_Y_LENGTH : I
private static final PACKED_X_MASK : J
private static final PACKED_Y_MASK : J
private static final PACKED_Z_MASK : J
private static final Y_OFFSET : I
private static final Z_OFFSET : I
private static final X_OFFSET : I
public static final MAX_HORIZONTAL_COORDINATE : I
public <init>(III)V
public static offset(JLnet/minecraft/core/Direction;)J
public static offset(JIII)J
public static getX(J)I
public static getY(J)I
public static getZ(J)I
public static of(J)Lnet/minecraft/core/BlockPos;
public static containing(DDD)Lnet/minecraft/core/BlockPos;
public static containing(Lnet/minecraft/core/Position;)Lnet/minecraft/core/BlockPos;
public static min(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public static max(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public asLong()J
public static asLong(III)J
public static getFlatIndex(J)J
public offset(III)Lnet/minecraft/core/BlockPos;
public offset(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;
public subtract(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;
public multiply(I)Lnet/minecraft/core/BlockPos;
public above()Lnet/minecraft/core/BlockPos;
public above(I)Lnet/minecraft/core/BlockPos;
public below()Lnet/minecraft/core/BlockPos;
public below(I)Lnet/minecraft/core/BlockPos;
public north()Lnet/minecraft/core/BlockPos;
public north(I)Lnet/minecraft/core/BlockPos;
public south()Lnet/minecraft/core/BlockPos;
public south(I)Lnet/minecraft/core/BlockPos;
public west()Lnet/minecraft/core/BlockPos;
public west(I)Lnet/minecraft/core/BlockPos;
public east()Lnet/minecraft/core/BlockPos;
public east(I)Lnet/minecraft/core/BlockPos;
public relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos;
public relative(Lnet/minecraft/core/Direction;I)Lnet/minecraft/core/BlockPos;
public relative(Lnet/minecraft/core/Direction$Axis;I)Lnet/minecraft/core/BlockPos;
public rotate(Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/core/BlockPos;
public cross(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;
public atY(I)Lnet/minecraft/core/BlockPos;
public immutable()Lnet/minecraft/core/BlockPos;
public mutable()Lnet/minecraft/core/BlockPos$MutableBlockPos;
public clampLocationWithin(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/Vec3;
public static randomInCube(Lnet/minecraft/util/RandomSource;ILnet/minecraft/core/BlockPos;I)Ljava/lang/Iterable;
public static randomBetweenClosed(Lnet/minecraft/util/RandomSource;IIIIIII)Ljava/lang/Iterable;
public static withinManhattan(Lnet/minecraft/core/BlockPos;I)Ljava/lang/Iterable;
public static withinClippedManhattan(Lnet/minecraft/core/BlockPos;III)Ljava/lang/Iterable;
public static withinBoxByManhattanDistance(Lnet/minecraft/core/BlockPos;III)Ljava/lang/Iterable;
private static manhattanOrdered(Lnet/minecraft/core/BlockPos;IIII)Ljava/lang/Iterable;
public static betweenClosed(Lnet/minecraft/world/phys/AABB;)Ljava/lang/Iterable;
public static betweenClosed(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Ljava/lang/Iterable;
public static betweenClosedStream(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;)Ljava/util/stream/Stream;
public static betweenClosedStream(Lnet/minecraft/world/level/levelgen/structure/BoundingBox;)Ljava/util/stream/Stream;
public static betweenClosedStream(Lnet/minecraft/world/phys/AABB;)Ljava/util/stream/Stream;
public static betweenClosedStream(IIIIII)Ljava/util/stream/Stream;
public static betweenClosed(IIIIII)Ljava/lang/Iterable;
public static neighborColumn(IIII)Ljava/lang/Iterable;
public static spiralAround(Lnet/minecraft/core/BlockPos;ILnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;)Ljava/lang/Iterable;
public static breadthFirstTraversal(Lnet/minecraft/core/BlockPos;IILjava/util/function/BiConsumer;Ljava/util/function/Function;)I
public static betweenCornersInDirection(Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/phys/Vec3;)Ljava/lang/Iterable;
public static betweenCornersInDirection(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/Vec3;)Ljava/lang/Iterable;
public static betweenCornersInDirection(IIIIIILnet/minecraft/world/phys/Vec3;)Ljava/lang/Iterable;
public synthetic cross(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public synthetic relative(Lnet/minecraft/core/Direction$Axis;I)Lnet/minecraft/core/Vec3i;
public synthetic relative(Lnet/minecraft/core/Direction;I)Lnet/minecraft/core/Vec3i;
public synthetic relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Vec3i;
public synthetic east(I)Lnet/minecraft/core/Vec3i;
public synthetic east()Lnet/minecraft/core/Vec3i;
public synthetic west(I)Lnet/minecraft/core/Vec3i;
public synthetic west()Lnet/minecraft/core/Vec3i;
public synthetic south(I)Lnet/minecraft/core/Vec3i;
public synthetic south()Lnet/minecraft/core/Vec3i;
public synthetic north(I)Lnet/minecraft/core/Vec3i;
public synthetic north()Lnet/minecraft/core/Vec3i;
public synthetic below(I)Lnet/minecraft/core/Vec3i;
public synthetic below()Lnet/minecraft/core/Vec3i;
public synthetic above(I)Lnet/minecraft/core/Vec3i;
public synthetic above()Lnet/minecraft/core/Vec3i;
public synthetic multiply(I)Lnet/minecraft/core/Vec3i;
public synthetic subtract(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public synthetic offset(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public synthetic offset(III)Lnet/minecraft/core/Vec3i;
private static synthetic lambda$betweenCornersInDirection$0(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;IIIIII)Ljava/util/Iterator;
private static synthetic lambda$breadthFirstTraversal$0(Ljava/util/Queue;ILnet/minecraft/core/BlockPos;)V
private static synthetic lambda$spiralAround$0(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;I)Ljava/util/Iterator;
private static synthetic lambda$neighborColumn$0(II[Lnet/minecraft/core/Vec3i;IIII)Ljava/util/Iterator;
private static synthetic lambda$betweenClosed$0(IIIIII)Ljava/util/Iterator;
private static synthetic lambda$manhattanOrdered$0(IIIIIII)Ljava/util/Iterator;
private static synthetic lambda$randomBetweenClosed$0(IILnet/minecraft/util/RandomSource;IIIII)Ljava/util/Iterator;
private static synthetic lambda$static$2(Lnet/minecraft/core/BlockPos;)Ljava/util/stream/IntStream;
private static synthetic lambda$static$0(Ljava/util/stream/IntStream;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1([I)Lnet/minecraft/core/BlockPos;
static <clinit>()V
```
