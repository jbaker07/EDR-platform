---
type: "interface"
fqcn: "net.minecraft.core.Direction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Direction

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get3DDataValue()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `get3DDataValue()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `get3DDataValue()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getApproximateNearest(FFF)Lnet/minecraft/core/Direction;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getAxis()Lnet/minecraft/core/Direction$Axis;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getAxis()Lnet/minecraft/core/Direction$Axis;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getAxisDirection()Lnet/minecraft/core/Direction$AxisDirection;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getOpposite()Lnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getOpposite()Lnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getOpposite()Lnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOpposite()Lnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getOpposite()Lnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getRotation()Lorg/joml/Quaternionf;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getUnitVec3i()Lnet/minecraft/core/Vec3i;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `name()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `toString()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values()[Lnet/minecraft/core/Direction;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values()[Lnet/minecraft/core/Direction;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `DOWNLnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `UPLnet/minecraft/core/Direction;` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (88, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.Direction extends java.lang.Enum<net.minecraft.core.Direction> implements net.minecraft.core.Directional, net.minecraft.util.StringRepresentable {
    public static final net.minecraft.core.Direction DOWN;
    public static final net.minecraft.core.Direction UP;
    public static final net.minecraft.core.Direction NORTH;
    public static final net.minecraft.core.Direction SOUTH;
    public static final net.minecraft.core.Direction WEST;
    public static final net.minecraft.core.Direction EAST;
    public static final net.minecraft.util.StringRepresentable$EnumCodec<net.minecraft.core.Direction> CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Direction> VERTICAL_CODEC;
    public static final java.util.function.IntFunction<net.minecraft.core.Direction> BY_ID;
    public static final net.minecraft.network.codec.StreamCodec<io.netty.buffer.ByteBuf, net.minecraft.core.Direction> STREAM_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Direction> LEGACY_ID_CODEC;
    public static final com.mojang.serialization.Codec<net.minecraft.core.Direction> LEGACY_ID_CODEC_2D;
    private static final com.google.common.collect.ImmutableList<net.minecraft.core.Direction$Axis> YXZ_AXIS_ORDER;
    private static final com.google.common.collect.ImmutableList<net.minecraft.core.Direction$Axis> YZX_AXIS_ORDER;
    private final int data3d;
    private final int oppositeIndex;
    private final int data2d;
    private final java.lang.String name;
    private final net.minecraft.core.Direction$Axis axis;
    private final net.minecraft.core.Direction$AxisDirection axisDirection;
    private final net.minecraft.core.Vec3i normal;
    private final net.minecraft.world.phys.Vec3 normalVec3;
    private final org.joml.Vector3fc normalVec3f;
    private static final net.minecraft.core.Direction[] VALUES;
    private static final net.minecraft.core.Direction[] BY_3D_DATA;
    private static final net.minecraft.core.Direction[] BY_2D_DATA;
    private static final net.minecraft.core.Direction[] $VALUES;
    public static net.minecraft.core.Direction[] values();
    public static net.minecraft.core.Direction valueOf(java.lang.String);
    private net.minecraft.core.Direction(int, int, int, java.lang.String, net.minecraft.core.Direction$AxisDirection, net.minecraft.core.Direction$Axis, net.minecraft.core.Vec3i);
    public static net.minecraft.core.Direction[] orderedByNearest(net.minecraft.world.entity.Entity);
    private static net.minecraft.core.Direction[] makeDirectionArray(net.minecraft.core.Direction, net.minecraft.core.Direction, net.minecraft.core.Direction);
    public static net.minecraft.core.Direction rotate(org.joml.Matrix4fc, net.minecraft.core.Direction);
    public static java.util.Collection<net.minecraft.core.Direction> allShuffled(net.minecraft.util.RandomSource);
    public static java.util.stream.Stream<net.minecraft.core.Direction> stream();
    public static float getYRot(net.minecraft.core.Direction);
    public org.joml.Quaternionf getRotation();
    public int get3DDataValue();
    public int get2DDataValue();
    public net.minecraft.core.Direction$AxisDirection getAxisDirection();
    public static net.minecraft.core.Direction getFacingAxis(net.minecraft.world.entity.Entity, net.minecraft.core.Direction$Axis);
    public net.minecraft.core.Direction getOpposite();
    public net.minecraft.core.Direction getClockWise(net.minecraft.core.Direction$Axis);
    public net.minecraft.core.Direction getCounterClockWise(net.minecraft.core.Direction$Axis);
    public net.minecraft.core.Direction getClockWise();
    private net.minecraft.core.Direction getClockWiseX();
    private net.minecraft.core.Direction getCounterClockWiseX();
    private net.minecraft.core.Direction getClockWiseZ();
    private net.minecraft.core.Direction getCounterClockWiseZ();
    public net.minecraft.core.Direction getCounterClockWise();
    public int getStepX();
    public int getStepY();
    public int getStepZ();
    public net.minecraft.core.Vec3i getStep();
    public org.joml.Vector3f step();
    public java.lang.String getName();
    public net.minecraft.core.Direction$Axis getAxis();
    public static net.minecraft.core.Direction byName(java.lang.String);
    public static net.minecraft.core.Direction from3DDataValue(int);
    public static net.minecraft.core.Direction from2DDataValue(int);
    public static net.minecraft.core.Direction fromYRot(double);
    public static net.minecraft.core.Direction fromAxisAndDirection(net.minecraft.core.Direction$Axis, net.minecraft.core.Direction$AxisDirection);
    public float toYRot();
    public static net.minecraft.core.Direction getRandom(net.minecraft.util.RandomSource);
    public static net.minecraft.core.Direction getApproximateNearest(double, double, double);
    public static net.minecraft.core.Direction getApproximateNearest(float, float, float);
    public static net.minecraft.core.Direction getApproximateNearest(net.minecraft.world.phys.Vec3);
    public static net.minecraft.core.Direction getNearest(int, int, int, net.minecraft.core.Direction);
    public static net.minecraft.core.Direction getNearest(net.minecraft.core.Vec3i, net.minecraft.core.Direction);
    public java.lang.String toString();
    public java.lang.String getSerializedName();
    private static com.mojang.serialization.DataResult<net.minecraft.core.Direction> verifyVertical(net.minecraft.core.Direction);
    public static net.minecraft.core.Direction get(net.minecraft.core.Direction$AxisDirection, net.minecraft.core.Direction$Axis);
    public static com.google.common.collect.ImmutableList<net.minecraft.core.Direction$Axis> axisStepOrder(net.minecraft.world.phys.Vec3);
    public net.minecraft.core.Vec3i getUnitVec3i();
    public net.minecraft.world.phys.Vec3 getUnitVec3();
    public org.joml.Vector3fc getUnitVec3f();
    public boolean isFacingAngle(float);
    private static net.minecraft.core.Direction[] $values();
    private static java.lang.String lambda$verifyVertical$0();
    private static net.minecraft.core.Direction[] lambda$BY_2D_DATA$0(int);
    private static int lambda$static$4(net.minecraft.core.Direction);
    private static boolean lambda$static$3(net.minecraft.core.Direction);
    private static net.minecraft.core.Direction[] lambda$BY_3D_DATA$0(int);
    private static int lambda$static$2(net.minecraft.core.Direction);
    private static java.lang.Byte lambda$static$1(net.minecraft.core.Direction);
    private static java.lang.Byte lambda$static$0(net.minecraft.core.Direction);
    static {};
}
```
