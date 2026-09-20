---
type: "interface"
fqcn: "net.minecraft.core.BlockPos$MutableBlockPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.BlockPos$MutableBlockPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `net/minecraft/core/BlockPos`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@9 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@20 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@9 in `FlatLighter.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@27 in `AltModelBlockRendererImpl.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@125 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@212 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@299 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@386 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@453 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@550 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@647 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@744 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@26 in `FlatLighter.light` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move` | `(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBl` | exact | invokevirtual@72 in `FlatLighter.light` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$MutableBlockP` | exact | invokevirtual@50 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$MutableBlockP` | exact | invokevirtual@5 in `FlatLighter.light` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@39 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@71 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@158 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@245 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@332 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@443 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@540 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@637 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@734 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@801 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset` | `(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecra` | exact | invokevirtual@45 in `AltModelBlockRendererImpl.shouldCullFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (0 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public <init>(III)V
public <init>(DDD)V
public offset(III)Lnet/minecraft/core/BlockPos;
public multiply(I)Lnet/minecraft/core/BlockPos;
public relative(Lnet/minecraft/core/Direction;I)Lnet/minecraft/core/BlockPos;
public relative(Lnet/minecraft/core/Direction$Axis;I)Lnet/minecraft/core/BlockPos;
public rotate(Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/core/BlockPos;
public set(III)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public set(DDD)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public set(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public set(J)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public set(Lnet/minecraft/core/AxisCycle;III)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setWithOffset(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setWithOffset(Lnet/minecraft/core/Vec3i;III)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setWithOffset(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public move(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public move(Lnet/minecraft/core/Direction;I)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public move(III)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public move(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public clamp(Lnet/minecraft/core/Direction$Axis;II)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setX(I)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setY(I)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public setZ(I)Lnet/minecraft/core/BlockPos$MutableBlockPos;
public immutable()Lnet/minecraft/core/BlockPos;
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
public synthetic setZ(I)Lnet/minecraft/core/Vec3i;
public synthetic setY(I)Lnet/minecraft/core/Vec3i;
public synthetic setX(I)Lnet/minecraft/core/Vec3i;
```
