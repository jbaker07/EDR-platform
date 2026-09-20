---
type: "interface"
fqcn: "net.minecraft.world.level.pathfinder.PathfindingContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.pathfinder.PathfindingContext

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `level` | `()Lnet/minecraft/world/level/CollisionGetter;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| injects_into | `getPathTypeFromState` | `(III)Lnet/minecraft/world/level/pathfinder/PathType;` | name_only | @Inject at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final level : Lnet/minecraft/world/level/CollisionGetter;
private final cache : Lnet/minecraft/world/level/pathfinder/PathTypeCache;
private final mobPosition : Lnet/minecraft/core/BlockPos;
private final mutablePos : Lnet/minecraft/core/BlockPos$MutableBlockPos;
public <init>(Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/world/entity/Mob;)V
public getPathTypeFromState(III)Lnet/minecraft/world/level/pathfinder/PathType;
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public level()Lnet/minecraft/world/level/CollisionGetter;
public mobPosition()Lnet/minecraft/core/BlockPos;
```
