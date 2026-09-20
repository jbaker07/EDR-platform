---
type: "interface"
fqcn: "net.minecraft.core.BlockPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.BlockPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `asLong()J` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `asLong()J` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `containing(Lnet/minecraft/core/Position;)Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `containing(DDD)Lnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `containing(DDD)Lnet/minecraft/core/BlockPos;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getX()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getY()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getZ()I` | `` | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `immutable()Lnet/minecraft/core/BlockPos;` | `` | both | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `immutable()Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `immutable()Lnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `immutable()Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `immutable()Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `offset(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `offset(III)Lnet/minecraft/core/BlockPos;` | `` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toShortString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `ZEROLnet/minecraft/core/BlockPos;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (101, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.BlockPos extends net.minecraft.core.Vec3i {
    public static final com.mojang.serialization.Codec<net.minecraft.core.BlockPos> CODEC;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.core.BlockPos> STREAM_CODEC;
    public static final net.minecraft.core.BlockPos ZERO;
    public static final int PACKED_HORIZONTAL_LENGTH;
    public static final int PACKED_Y_LENGTH;
    private static final long PACKED_X_MASK;
    private static final long PACKED_Y_MASK;
    private static final long PACKED_Z_MASK;
    private static final int Y_OFFSET;
    private static final int Z_OFFSET;
    private static final int X_OFFSET;
    public static final int MAX_HORIZONTAL_COORDINATE;
    public net.minecraft.core.BlockPos(int, int, int);
    public static long offset(long, net.minecraft.core.Direction);
    public static long offset(long, int, int, int);
    public static int getX(long);
    public static int getY(long);
    public static int getZ(long);
    public static net.minecraft.core.BlockPos of(long);
    public static net.minecraft.core.BlockPos containing(double, double, double);
    public static net.minecraft.core.BlockPos containing(net.minecraft.core.Position);
    public static net.minecraft.core.BlockPos min(net.minecraft.core.BlockPos, net.minecraft.core.BlockPos);
    public static net.minecraft.core.BlockPos max(net.minecraft.core.BlockPos, net.minecraft.core.BlockPos);
    public long asLong();
    public static long asLong(int, int, int);
    public static long getFlatIndex(long);
    public net.minecraft.core.BlockPos offset(int, int, int);
    public net.minecraft.core.BlockPos offset(net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos subtract(net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos multiply(int);
    public net.minecraft.core.BlockPos above();
    public net.minecraft.core.BlockPos above(int);
    public net.minecraft.core.BlockPos below();
    public net.minecraft.core.BlockPos below(int);
    public net.minecraft.core.BlockPos north();
    public net.minecraft.core.BlockPos north(int);
    public net.minecraft.core.BlockPos south();
    public net.minecraft.core.BlockPos south(int);
    public net.minecraft.core.BlockPos west();
    public net.minecraft.core.BlockPos west(int);
    public net.minecraft.core.BlockPos east();
    public net.minecraft.core.BlockPos east(int);
    public net.minecraft.core.BlockPos relative(net.minecraft.core.Direction);
    public net.minecraft.core.BlockPos relative(net.minecraft.core.Direction, int);
    public net.minecraft.core.BlockPos relative(net.minecraft.core.Direction$Axis, int);
    public net.minecraft.core.BlockPos rotate(net.minecraft.world.level.block.Rotation);
    public net.minecraft.core.BlockPos cross(net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos atY(int);
    public net.minecraft.core.BlockPos immutable();
    public net.minecraft.core.BlockPos$MutableBlockPos mutable();
    public net.minecraft.world.phys.Vec3 clampLocationWithin(net.minecraft.world.phys.Vec3);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> randomInCube(net.minecraft.util.RandomSource, int, net.minecraft.core.BlockPos, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> randomBetweenClosed(net.minecraft.util.RandomSource, int, int, int, int, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> withinManhattan(net.minecraft.core.BlockPos, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> withinClippedManhattan(net.minecraft.core.BlockPos, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> withinBoxByManhattanDistance(net.minecraft.core.BlockPos, int, int, int);
    private static java.lang.Iterable<net.minecraft.core.BlockPos> manhattanOrdered(net.minecraft.core.BlockPos, int, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenClosed(net.minecraft.world.phys.AABB);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenClosed(net.minecraft.core.BlockPos, net.minecraft.core.BlockPos);
    public static java.util.stream.Stream<net.minecraft.core.BlockPos> betweenClosedStream(net.minecraft.core.BlockPos, net.minecraft.core.BlockPos);
    public static java.util.stream.Stream<net.minecraft.core.BlockPos> betweenClosedStream(net.minecraft.world.level.levelgen.structure.BoundingBox);
    public static java.util.stream.Stream<net.minecraft.core.BlockPos> betweenClosedStream(net.minecraft.world.phys.AABB);
    public static java.util.stream.Stream<net.minecraft.core.BlockPos> betweenClosedStream(int, int, int, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenClosed(int, int, int, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> neighborColumn(int, int, int, int);
    public static java.lang.Iterable<net.minecraft.core.BlockPos$MutableBlockPos> spiralAround(net.minecraft.core.BlockPos, int, net.minecraft.core.Direction, net.minecraft.core.Direction);
    public static int breadthFirstTraversal(net.minecraft.core.BlockPos, int, int, java.util.function.BiConsumer<net.minecraft.core.BlockPos, java.util.function.Consumer<net.minecraft.core.BlockPos>>, java.util.function.Function<net.minecraft.core.BlockPos, net.minecraft.core.BlockPos$TraversalNodeStatus>);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenCornersInDirection(net.minecraft.world.phys.AABB, net.minecraft.world.phys.Vec3);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenCornersInDirection(net.minecraft.core.BlockPos, net.minecraft.core.BlockPos, net.minecraft.world.phys.Vec3);
    public static java.lang.Iterable<net.minecraft.core.BlockPos> betweenCornersInDirection(int, int, int, int, int, int, net.minecraft.world.phys.Vec3);
    public net.minecraft.core.Vec3i cross(net.minecraft.core.Vec3i);
    public net.minecraft.core.Vec3i relative(net.minecraft.core.Direction$Axis, int);
    public net.minecraft.core.Vec3i relative(net.minecraft.core.Direction, int);
    public net.minecraft.core.Vec3i relative(net.minecraft.core.Direction);
    public net.minecraft.core.Vec3i east(int);
    public net.minecraft.core.Vec3i east();
    public net.minecraft.core.Vec3i west(int);
    public net.minecraft.core.Vec3i west();
    public net.minecraft.core.Vec3i south(int);
    public net.minecraft.core.Vec3i south();
    public net.minecraft.core.Vec3i north(int);
    public net.minecraft.core.Vec3i north();
    public net.minecraft.core.Vec3i below(int);
    public net.minecraft.core.Vec3i below();
    public net.minecraft.core.Vec3i above(int);
    public net.minecraft.core.Vec3i above();
    public net.minecraft.core.Vec3i multiply(int);
    public net.minecraft.core.Vec3i subtract(net.minecraft.core.Vec3i);
    public net.minecraft.core.Vec3i offset(net.minecraft.core.Vec3i);
    public net.minecraft.core.Vec3i offset(int, int, int);
    private static java.util.Iterator lambda$betweenCornersInDirection$0(net.minecraft.core.Direction, net.minecraft.core.Direction, net.minecraft.core.Direction, int, int, int, int, int, int);
    private static void lambda$breadthFirstTraversal$0(java.util.Queue, int, net.minecraft.core.BlockPos);
    private static java.util.Iterator lambda$spiralAround$0(net.minecraft.core.Direction, net.minecraft.core.Direction, net.minecraft.core.BlockPos, int);
    private static java.util.Iterator lambda$neighborColumn$0(int, int, net.minecraft.core.Vec3i[], int, int, int, int);
    private static java.util.Iterator lambda$betweenClosed$0(int, int, int, int, int, int);
    private static java.util.Iterator lambda$manhattanOrdered$0(int, int, int, int, int, int, int);
    private static java.util.Iterator lambda$randomBetweenClosed$0(int, int, net.minecraft.util.RandomSource, int, int, int, int, int);
    private static java.util.stream.IntStream lambda$static$2(net.minecraft.core.BlockPos);
    private static com.mojang.serialization.DataResult lambda$static$0(java.util.stream.IntStream);
    private static net.minecraft.core.BlockPos lambda$static$1(int[]);
    static {};
}
```
