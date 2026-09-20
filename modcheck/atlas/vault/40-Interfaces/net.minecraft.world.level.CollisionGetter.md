---
type: "interface"
fqcn: "net.minecraft.world.level.CollisionGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.CollisionGetter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/BlockGetter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@7 in `LivingEntityMixin.modifyWakeUpPosition` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (0 fields, 31 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getWorldBorder()Lnet/minecraft/world/level/border/WorldBorder;
public abstract getChunkForCollisions(II)Lnet/minecraft/world/level/BlockGetter;
public isUnobstructed(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/shapes/VoxelShape;)Z
public isUnobstructed(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Z
public isUnobstructed(Lnet/minecraft/world/entity/Entity;)Z
public noCollision(Lnet/minecraft/world/phys/AABB;)Z
public noCollision(Lnet/minecraft/world/entity/Entity;)Z
public noCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z
public noCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Z)Z
public noBlockCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z
public noBlockCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Z)Z
public noEntityCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z
public noBorderCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z
public abstract getEntityCollisions(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
public getCollisions(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/lang/Iterable;
public getPreMoveCollisions(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/phys/Vec3;)Ljava/lang/Iterable;
public getBlockCollisions(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/lang/Iterable;
public getBlockAndLiquidCollisions(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/lang/Iterable;
public getBlockCollisionsFromContext(Lnet/minecraft/world/phys/shapes/CollisionContext;Lnet/minecraft/world/phys/AABB;)Ljava/lang/Iterable;
private borderCollision(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public clipIncludingBorder(Lnet/minecraft/world/level/ClipContext;)Lnet/minecraft/world/phys/BlockHitResult;
public collidesWithSuffocatingBlock(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Z
public findSupportingBlock(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/Optional;
public findFreePosition(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/world/phys/Vec3;DDD)Ljava/util/Optional;
private static synthetic lambda$findFreePosition$2(DDDLnet/minecraft/world/phys/AABB;)Lnet/minecraft/world/phys/AABB;
private static synthetic lambda$findFreePosition$1(Lnet/minecraft/world/phys/shapes/VoxelShape;)Ljava/util/stream/Stream;
private synthetic lambda$findFreePosition$0(Lnet/minecraft/world/phys/shapes/VoxelShape;)Z
private static synthetic lambda$findSupportingBlock$0(Lnet/minecraft/core/BlockPos$MutableBlockPos;Lnet/minecraft/world/phys/shapes/VoxelShape;)Lnet/minecraft/core/BlockPos;
private static synthetic lambda$collidesWithSuffocatingBlock$0(Lnet/minecraft/core/BlockPos$MutableBlockPos;Lnet/minecraft/world/phys/shapes/VoxelShape;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private synthetic lambda$getBlockCollisionsFromContext$0(Lnet/minecraft/world/phys/shapes/CollisionContext;Lnet/minecraft/world/phys/AABB;)Ljava/util/Iterator;
private static synthetic lambda$getBlockCollisionsFromContext$1(Lnet/minecraft/core/BlockPos$MutableBlockPos;Lnet/minecraft/world/phys/shapes/VoxelShape;)Lnet/minecraft/world/phys/shapes/VoxelShape;
```
