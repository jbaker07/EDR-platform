---
type: "interface"
fqcn: "net.minecraft.world.level.block.AbstractBedBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.AbstractBedBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `findStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.block.AbstractBedBlock extends net.minecraft.world.level.block.HorizontalDirectionalBlock {
    public static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.world.level.block.state.properties.BedPart> PART;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty OCCUPIED;
    public net.minecraft.world.level.block.AbstractBedBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    public static net.minecraft.core.Direction getBedOrientation(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    protected abstract net.minecraft.world.attribute.EnvironmentAttribute<net.minecraft.world.attribute.BedRule> getBedEnvironmentAttribute();
    protected abstract net.minecraft.world.InteractionResult destroyOnUse(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player);
    protected abstract void destroyOnLeave(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    public net.minecraft.world.attribute.BedRule getBedRule(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    public net.minecraft.resources.Identifier getSleptInBedStatType();
    public java.util.OptionalDouble getSleepHeight(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    protected net.minecraft.world.InteractionResult useWithoutItem(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player, net.minecraft.world.phys.BlockHitResult);
    public void onStopSleeping(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private boolean kickVillagerOutOfBed(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    protected net.minecraft.world.level.block.state.BlockState updateShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource);
    private static net.minecraft.core.Direction getNeighbourDirection(net.minecraft.world.level.block.state.properties.BedPart, net.minecraft.core.Direction);
    public net.minecraft.world.level.block.state.BlockState playerWillDestroy(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.player.Player);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    public static net.minecraft.core.Direction getConnectedDirection(net.minecraft.world.level.block.state.BlockState);
    public static net.minecraft.world.level.block.DoubleBlockCombiner$BlockType getBlockType(net.minecraft.world.level.block.state.BlockState);
    private static boolean isBunkBed(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos);
    public static java.util.Optional<net.minecraft.world.phys.Vec3> findStandUpPosition(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.CollisionGetter, net.minecraft.core.BlockPos, net.minecraft.core.Direction, float);
    private static java.util.Optional<net.minecraft.world.phys.Vec3> findBunkBedStandUpPosition(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.CollisionGetter, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.Direction);
    private static java.util.Optional<net.minecraft.world.phys.Vec3> findStandUpPositionAtOffset(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.level.CollisionGetter, net.minecraft.core.BlockPos, int[][], boolean);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    public void setPlacedBy(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.LivingEntity, net.minecraft.world.item.ItemStack);
    protected long getSeed(net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    protected boolean isPathfindable(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.pathfinder.PathComputationType);
    private static int[][] bedStandUpOffsets(net.minecraft.core.Direction, net.minecraft.core.Direction);
    private static int[][] bedSurroundStandUpOffsets(net.minecraft.core.Direction, net.minecraft.core.Direction);
    private static int[][] bedAboveStandUpOffsets(net.minecraft.core.Direction);
    private static void lambda$useWithoutItem$0(net.minecraft.world.entity.player.Player, net.minecraft.world.entity.player.Player$BedSleepingProblem);
    static {};
}
```
