---
type: "interface"
fqcn: "net.minecraft.world.level.pathfinder.PathfindingContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.pathfinder.PathfindingContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `getPathTypeFromState` | `@Inject at INVOKE_ASSIGN Lnet/minecraft/core/BlockPos$MutableBlockPos;set(III)Ln` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.pathfinder.PathfindingContext {
    private final net.minecraft.world.level.CollisionGetter level;
    private final net.minecraft.world.level.pathfinder.PathTypeCache cache;
    private final net.minecraft.core.BlockPos mobPosition;
    private final net.minecraft.core.BlockPos$MutableBlockPos mutablePos;
    public net.minecraft.world.level.pathfinder.PathfindingContext(net.minecraft.world.level.CollisionGetter, net.minecraft.world.entity.Mob);
    public net.minecraft.world.level.pathfinder.PathType getPathTypeFromState(int, int, int);
    public net.minecraft.world.level.block.state.BlockState getBlockState(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.CollisionGetter level();
    public net.minecraft.core.BlockPos mobPosition();
}
```
