---
type: "interface"
fqcn: "net.minecraft.world.level.pathfinder.WalkNodeEvaluator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.pathfinder.WalkNodeEvaluator

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `getPathTypeFromState` | `@Inject at INVOKE Lnet/minecraft/world/level/block/state/BlockState;getBlock()Ln` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (40, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.pathfinder.WalkNodeEvaluator extends net.minecraft.world.level.pathfinder.NodeEvaluator {
    public static final double SPACE_BETWEEN_WALL_POSTS;
    private static final double DEFAULT_MOB_JUMP_HEIGHT;
    private final it.unimi.dsi.fastutil.longs.Long2ObjectMap<net.minecraft.world.level.pathfinder.PathType> pathTypesByPosCacheByMob;
    private final it.unimi.dsi.fastutil.objects.Object2BooleanMap<net.minecraft.world.phys.AABB> collisionCache;
    private final net.minecraft.world.level.pathfinder.Node[] reusableNeighbors;
    public net.minecraft.world.level.pathfinder.WalkNodeEvaluator();
    public void prepare(net.minecraft.world.level.PathNavigationRegion, net.minecraft.world.entity.Mob);
    public void done();
    public net.minecraft.world.level.pathfinder.Node getStart();
    protected net.minecraft.world.level.pathfinder.Node getStartNode(net.minecraft.core.BlockPos);
    protected boolean canStartAt(net.minecraft.core.BlockPos);
    public net.minecraft.world.level.pathfinder.Target getTarget(double, double, double);
    public int getNeighbors(net.minecraft.world.level.pathfinder.Node[], net.minecraft.world.level.pathfinder.Node);
    protected boolean isNeighborValid(net.minecraft.world.level.pathfinder.Node, net.minecraft.world.level.pathfinder.Node);
    protected boolean isDiagonalValid(net.minecraft.world.level.pathfinder.Node, net.minecraft.world.level.pathfinder.Node, net.minecraft.world.level.pathfinder.Node);
    protected boolean isDiagonalValid(net.minecraft.world.level.pathfinder.Node);
    private static boolean doesBlockHavePartialCollision(net.minecraft.world.level.pathfinder.PathType);
    private boolean canReachWithoutCollision(net.minecraft.world.level.pathfinder.Node);
    protected double getFloorLevel(net.minecraft.core.BlockPos);
    public static double getFloorLevel(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    protected boolean isAmphibious();
    protected net.minecraft.world.level.pathfinder.Node findAcceptedNode(int, int, int, int, double, net.minecraft.core.Direction, net.minecraft.world.level.pathfinder.PathType);
    private double getMobJumpHeight();
    private net.minecraft.world.level.pathfinder.Node getNodeAndUpdateCostToMax(int, int, int, net.minecraft.world.level.pathfinder.PathType, float);
    private net.minecraft.world.level.pathfinder.Node getBlockedNode(int, int, int);
    private net.minecraft.world.level.pathfinder.Node getClosedNode(int, int, int, net.minecraft.world.level.pathfinder.PathType);
    private net.minecraft.world.level.pathfinder.Node tryJumpOn(int, int, int, int, double, net.minecraft.core.Direction, net.minecraft.world.level.pathfinder.PathType, net.minecraft.core.BlockPos$MutableBlockPos);
    private net.minecraft.world.level.pathfinder.Node tryFindFirstNonWaterBelow(int, int, int, net.minecraft.world.level.pathfinder.Node);
    private net.minecraft.world.level.pathfinder.Node tryFindFirstGroundNodeBelow(int, int, int);
    private boolean hasCollisions(net.minecraft.world.phys.AABB);
    protected net.minecraft.world.level.pathfinder.PathType getCachedPathType(int, int, int);
    public net.minecraft.world.level.pathfinder.PathType getPathTypeOfMob(net.minecraft.world.level.pathfinder.PathfindingContext, int, int, int, net.minecraft.world.entity.Mob);
    public java.util.Set<net.minecraft.world.level.pathfinder.PathType> getPathTypeWithinMobBB(net.minecraft.world.level.pathfinder.PathfindingContext, int, int, int);
    public net.minecraft.world.level.pathfinder.PathType getPathType(net.minecraft.world.level.pathfinder.PathfindingContext, int, int, int);
    public static net.minecraft.world.level.pathfinder.PathType getPathTypeStatic(net.minecraft.world.entity.Mob, net.minecraft.core.BlockPos);
    public static net.minecraft.world.level.pathfinder.PathType getPathTypeStatic(net.minecraft.world.level.pathfinder.PathfindingContext, net.minecraft.core.BlockPos$MutableBlockPos);
    public static net.minecraft.world.level.pathfinder.PathType checkNeighbourBlocks(net.minecraft.world.level.pathfinder.PathfindingContext, int, int, int, net.minecraft.world.level.pathfinder.PathType);
    protected static net.minecraft.world.level.pathfinder.PathType getPathTypeFromState(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    private net.minecraft.world.level.pathfinder.PathType lambda$getCachedPathType$0(int, int, int, long);
    private boolean lambda$hasCollisions$0(net.minecraft.world.phys.AABB, java.lang.Object);
}
```
