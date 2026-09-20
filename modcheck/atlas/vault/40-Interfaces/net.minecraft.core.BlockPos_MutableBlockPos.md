---
type: "interface"
fqcn: "net.minecraft.core.BlockPos$MutableBlockPos"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.BlockPos$MutableBlockPos

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `move(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `set(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$Mut` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `set(Lnet/minecraft/core/Vec3i;)Lnet/minecraft/core/BlockPos$Mut` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Ln` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setWithOffset(Lnet/minecraft/core/Vec3i;Lnet/minecraft/core/Direction;)Ln` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (48, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.BlockPos$MutableBlockPos extends net.minecraft.core.BlockPos {
    public net.minecraft.core.BlockPos$MutableBlockPos();
    public net.minecraft.core.BlockPos$MutableBlockPos(int, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos(double, double, double);
    public net.minecraft.core.BlockPos offset(int, int, int);
    public net.minecraft.core.BlockPos multiply(int);
    public net.minecraft.core.BlockPos relative(net.minecraft.core.Direction, int);
    public net.minecraft.core.BlockPos relative(net.minecraft.core.Direction$Axis, int);
    public net.minecraft.core.BlockPos rotate(net.minecraft.world.level.block.Rotation);
    public net.minecraft.core.BlockPos$MutableBlockPos set(int, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos set(double, double, double);
    public net.minecraft.core.BlockPos$MutableBlockPos set(net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos$MutableBlockPos set(long);
    public net.minecraft.core.BlockPos$MutableBlockPos set(net.minecraft.core.AxisCycle, int, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos setWithOffset(net.minecraft.core.Vec3i, net.minecraft.core.Direction);
    public net.minecraft.core.BlockPos$MutableBlockPos setWithOffset(net.minecraft.core.Vec3i, int, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos setWithOffset(net.minecraft.core.Vec3i, net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos$MutableBlockPos move(net.minecraft.core.Direction);
    public net.minecraft.core.BlockPos$MutableBlockPos move(net.minecraft.core.Direction, int);
    public net.minecraft.core.BlockPos$MutableBlockPos move(int, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos move(net.minecraft.core.Vec3i);
    public net.minecraft.core.BlockPos$MutableBlockPos clamp(net.minecraft.core.Direction$Axis, int, int);
    public net.minecraft.core.BlockPos$MutableBlockPos setX(int);
    public net.minecraft.core.BlockPos$MutableBlockPos setY(int);
    public net.minecraft.core.BlockPos$MutableBlockPos setZ(int);
    public net.minecraft.core.BlockPos immutable();
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
    public net.minecraft.core.Vec3i setZ(int);
    public net.minecraft.core.Vec3i setY(int);
    public net.minecraft.core.Vec3i setX(int);
}
```
