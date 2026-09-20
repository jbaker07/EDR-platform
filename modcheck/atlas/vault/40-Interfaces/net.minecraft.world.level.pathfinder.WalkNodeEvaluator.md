---
type: "interface"
fqcn: "net.minecraft.world.level.pathfinder.WalkNodeEvaluator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.pathfinder.WalkNodeEvaluator

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/pathfinder/NodeEvaluator`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `getPathTypeFromState` | `(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (5 fields, 35 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SPACE_BETWEEN_WALL_POSTS : D
private static final DEFAULT_MOB_JUMP_HEIGHT : D
private final pathTypesByPosCacheByMob : Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
private final collisionCache : Lit/unimi/dsi/fastutil/objects/Object2BooleanMap;
private final reusableNeighbors : [Lnet/minecraft/world/level/pathfinder/Node;
public <init>()V
public prepare(Lnet/minecraft/world/level/PathNavigationRegion;Lnet/minecraft/world/entity/Mob;)V
public done()V
public getStart()Lnet/minecraft/world/level/pathfinder/Node;
protected getStartNode(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/pathfinder/Node;
protected canStartAt(Lnet/minecraft/core/BlockPos;)Z
public getTarget(DDD)Lnet/minecraft/world/level/pathfinder/Target;
public getNeighbors([Lnet/minecraft/world/level/pathfinder/Node;Lnet/minecraft/world/level/pathfinder/Node;)I
protected isNeighborValid(Lnet/minecraft/world/level/pathfinder/Node;Lnet/minecraft/world/level/pathfinder/Node;)Z
protected isDiagonalValid(Lnet/minecraft/world/level/pathfinder/Node;Lnet/minecraft/world/level/pathfinder/Node;Lnet/minecraft/world/level/pathfinder/Node;)Z
protected isDiagonalValid(Lnet/minecraft/world/level/pathfinder/Node;)Z
private static doesBlockHavePartialCollision(Lnet/minecraft/world/level/pathfinder/PathType;)Z
private canReachWithoutCollision(Lnet/minecraft/world/level/pathfinder/Node;)Z
protected getFloorLevel(Lnet/minecraft/core/BlockPos;)D
public static getFloorLevel(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)D
protected isAmphibious()Z
protected findAcceptedNode(IIIIDLnet/minecraft/core/Direction;Lnet/minecraft/world/level/pathfinder/PathType;)Lnet/minecraft/world/level/pathfinder/Node;
private getMobJumpHeight()D
private getNodeAndUpdateCostToMax(IIILnet/minecraft/world/level/pathfinder/PathType;F)Lnet/minecraft/world/level/pathfinder/Node;
private getBlockedNode(III)Lnet/minecraft/world/level/pathfinder/Node;
private getClosedNode(IIILnet/minecraft/world/level/pathfinder/PathType;)Lnet/minecraft/world/level/pathfinder/Node;
private tryJumpOn(IIIIDLnet/minecraft/core/Direction;Lnet/minecraft/world/level/pathfinder/PathType;Lnet/minecraft/core/BlockPos$MutableBlockPos;)Lnet/minecraft/world/level/pathfinder/Node;
private tryFindFirstNonWaterBelow(IIILnet/minecraft/world/level/pathfinder/Node;)Lnet/minecraft/world/level/pathfinder/Node;
private tryFindFirstGroundNodeBelow(III)Lnet/minecraft/world/level/pathfinder/Node;
private hasCollisions(Lnet/minecraft/world/phys/AABB;)Z
protected getCachedPathType(III)Lnet/minecraft/world/level/pathfinder/PathType;
public getPathTypeOfMob(Lnet/minecraft/world/level/pathfinder/PathfindingContext;IIILnet/minecraft/world/entity/Mob;)Lnet/minecraft/world/level/pathfinder/PathType;
public getPathTypeWithinMobBB(Lnet/minecraft/world/level/pathfinder/PathfindingContext;III)Ljava/util/Set;
public getPathType(Lnet/minecraft/world/level/pathfinder/PathfindingContext;III)Lnet/minecraft/world/level/pathfinder/PathType;
public static getPathTypeStatic(Lnet/minecraft/world/entity/Mob;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/pathfinder/PathType;
public static getPathTypeStatic(Lnet/minecraft/world/level/pathfinder/PathfindingContext;Lnet/minecraft/core/BlockPos$MutableBlockPos;)Lnet/minecraft/world/level/pathfinder/PathType;
public static checkNeighbourBlocks(Lnet/minecraft/world/level/pathfinder/PathfindingContext;IIILnet/minecraft/world/level/pathfinder/PathType;)Lnet/minecraft/world/level/pathfinder/PathType;
protected static getPathTypeFromState(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/pathfinder/PathType;
private synthetic lambda$getCachedPathType$0(IIIJ)Lnet/minecraft/world/level/pathfinder/PathType;
private synthetic lambda$hasCollisions$0(Lnet/minecraft/world/phys/AABB;Ljava/lang/Object;)Z
```
