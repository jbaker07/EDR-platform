---
type: "interface"
fqcn: "net.minecraft.core.Direction$Axis"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Direction$Axis

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`enum` public final; extends `java/lang/Enum`; implements `java/util/function/Predicate`, `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isVertical` | `()Z` | exact | invokevirtual@8 in `ComposterWrapper.get` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@15 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@30 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@45 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@4 in `GeometryHelper.isQuadParallelToFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@6 in `GeometryHelper.isParallelQuadOnFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `ordinal` | `()I` | inherited_exact | invokevirtual@14 in `GeometryHelper.lightFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `values` | `()[Lnet/minecraft/core/Direction$Axis;` | exact | invokestatic@0 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `X` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@12 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `X` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@24 in `GeometryHelper.longestAxis` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `Y` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@27 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `Y` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@0 in `GeometryHelper.longestAxis` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `Z` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@42 in `GeometryHelper$1.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `Z` | `Lnet/minecraft/core/Direction$Axis;` | exact | getstatic@42 in `GeometryHelper.longestAxis` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (7 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final X : Lnet/minecraft/core/Direction$Axis;
public static final Y : Lnet/minecraft/core/Direction$Axis;
public static final Z : Lnet/minecraft/core/Direction$Axis;
public static final VALUES : [Lnet/minecraft/core/Direction$Axis;
public static final CODEC : Lnet/minecraft/util/StringRepresentable$EnumCodec;
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/core/Direction$Axis;
public static values()[Lnet/minecraft/core/Direction$Axis;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/core/Direction$Axis;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public static byName(Ljava/lang/String;)Lnet/minecraft/core/Direction$Axis;
public getName()Ljava/lang/String;
public isVertical()Z
public isHorizontal()Z
public getPositive()Lnet/minecraft/core/Direction;
public getNegative()Lnet/minecraft/core/Direction;
public getDirections()[Lnet/minecraft/core/Direction;
public toString()Ljava/lang/String;
public static getRandom(Lnet/minecraft/util/RandomSource;)Lnet/minecraft/core/Direction$Axis;
public test(Lnet/minecraft/core/Direction;)Z
public getPlane()Lnet/minecraft/core/Direction$Plane;
public getSerializedName()Ljava/lang/String;
public choose(III)I
public choose(DDD)D
public choose(ZZZ)Z
public synthetic test(Ljava/lang/Object;)Z
private static synthetic $values()[Lnet/minecraft/core/Direction$Axis;
static <clinit>()V
```
