---
type: "interface"
fqcn: "net.minecraft.core.Vec3i"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Vec3i

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements `java/lang/Comparable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getX` | `()I` | exact | invokevirtual@2 in `PlayerLookup.lambda$around$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@26 in `NormalHelper.computeFaceNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@7 in `PlayerLookup.lambda$around$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@31 in `NormalHelper.computeFaceNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getZ` | `()I` | exact | invokevirtual@12 in `PlayerLookup.lambda$around$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getZ` | `()I` | exact | invokevirtual@36 in `NormalHelper.computeFaceNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (6 fields, 52 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final ZERO : Lnet/minecraft/core/Vec3i;
private x : I
private y : I
private z : I
public static offsetCodec(I)Lcom/mojang/serialization/Codec;
public <init>(III)V
public equals(Ljava/lang/Object;)Z
public hashCode()I
public compareTo(Lnet/minecraft/core/Vec3i;)I
public getX()I
public getY()I
public getZ()I
protected setX(I)Lnet/minecraft/core/Vec3i;
protected setY(I)Lnet/minecraft/core/Vec3i;
protected setZ(I)Lnet/minecraft/core/Vec3i;
public offset(III)Lnet/minecraft/core/Vec3i;
public offset(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public subtract(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public multiply(I)Lnet/minecraft/core/Vec3i;
public multiply(III)Lnet/minecraft/core/Vec3i;
public above()Lnet/minecraft/core/Vec3i;
public above(I)Lnet/minecraft/core/Vec3i;
public below()Lnet/minecraft/core/Vec3i;
public below(I)Lnet/minecraft/core/Vec3i;
public north()Lnet/minecraft/core/Vec3i;
public north(I)Lnet/minecraft/core/Vec3i;
public south()Lnet/minecraft/core/Vec3i;
public south(I)Lnet/minecraft/core/Vec3i;
public west()Lnet/minecraft/core/Vec3i;
public west(I)Lnet/minecraft/core/Vec3i;
public east()Lnet/minecraft/core/Vec3i;
public east(I)Lnet/minecraft/core/Vec3i;
public relative(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Vec3i;
public relative(Lnet/minecraft/core/Direction;I)Lnet/minecraft/core/Vec3i;
public relative(Lnet/minecraft/core/Direction$Axis;I)Lnet/minecraft/core/Vec3i;
public cross(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/Vec3i;
public closerThan(Lnet/minecraft/core/Vec3i;D)Z
public closerToCenterThan(Lnet/minecraft/core/Position;D)Z
public distSqr(Lnet/minecraft/core/Vec3i;)D
public distToCenterSqr(Lnet/minecraft/core/Position;)D
public distToCenterSqr(DDD)D
public distToLowCornerSqr(DDD)D
public distManhattan(Lnet/minecraft/core/Vec3i;)I
public distChessboard(Lnet/minecraft/core/Vec3i;)I
public differsHorizontally(Lnet/minecraft/core/Vec3i;)Z
public get(Lnet/minecraft/core/Direction$Axis;)I
public toMutable()Lorg/joml/Vector3i;
public toString()Ljava/lang/String;
public toShortString()Ljava/lang/String;
public synthetic compareTo(Ljava/lang/Object;)I
private static synthetic lambda$offsetCodec$0(ILnet/minecraft/core/Vec3i;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$offsetCodec$1(ILnet/minecraft/core/Vec3i;)Ljava/lang/String;
private static synthetic lambda$static$2(Lnet/minecraft/core/Vec3i;)Ljava/util/stream/IntStream;
private static synthetic lambda$static$0(Ljava/util/stream/IntStream;)Lcom/mojang/serialization/DataResult;
private static synthetic lambda$static$1([I)Lnet/minecraft/core/Vec3i;
static <clinit>()V
```
