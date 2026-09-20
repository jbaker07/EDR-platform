---
type: "interface"
fqcn: "net.minecraft.world.level.block.LiquidBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.LiquidBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `shouldSpreadLiquid` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (30, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.LiquidBlock extends net.minecraft.world.level.block.Block implements net.minecraft.world.level.block.BucketPickup {
    public static final net.minecraft.world.level.block.state.properties.IntegerProperty LEVEL;
    protected final net.minecraft.world.level.material.FlowingFluid fluid;
    private final java.util.List<net.minecraft.world.level.material.FluidState> stateCache;
    public static final com.google.common.collect.ImmutableList<net.minecraft.core.Direction> POSSIBLE_FLOW_DIRECTIONS;
    private static final int BUBBLE_COLUMN_CHECK_DELAY;
    protected net.minecraft.world.level.block.LiquidBlock(net.minecraft.world.level.material.FlowingFluid, net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.world.phys.shapes.VoxelShape getCollisionShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    private java.util.Optional<net.minecraft.world.entity.LivingEntity> ifMobIsColliding(net.minecraft.world.phys.shapes.CollisionContext);
    protected boolean isRandomlyTicking(net.minecraft.world.level.block.state.BlockState);
    protected void randomTick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    protected boolean propagatesSkylightDown(net.minecraft.world.level.block.state.BlockState);
    protected boolean isPathfindable(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.pathfinder.PathComputationType);
    protected net.minecraft.world.level.material.FluidState getFluidState(net.minecraft.world.level.block.state.BlockState);
    protected boolean skipRendering(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.Direction);
    protected net.minecraft.world.level.block.RenderShape getRenderShape(net.minecraft.world.level.block.state.BlockState);
    protected java.util.List<net.minecraft.world.item.ItemStack> getDrops(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.storage.loot.LootParams$Builder);
    protected net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    protected void onPlace(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, boolean);
    protected void tick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    protected net.minecraft.world.level.block.state.BlockState updateShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource);
    private static boolean shouldBubbleColumnOccupy(net.minecraft.world.level.block.state.BlockState);
    protected void neighborChanged(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.Block, net.minecraft.world.level.redstone.Orientation, boolean);
    private void tryScheduleBubbleBlockColumn(net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private boolean shouldSpreadLiquid(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private void fizz(net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    public net.minecraft.world.item.ItemStack pickupBlock(net.minecraft.world.entity.LivingEntity, net.minecraft.world.level.LevelAccessor, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public java.util.Optional<net.minecraft.sounds.SoundEvent> getPickupSound();
    private static boolean lambda$getCollisionShape$0(net.minecraft.world.phys.shapes.CollisionContext, net.minecraft.core.BlockPos, net.minecraft.world.level.BlockGetter, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.phys.shapes.VoxelShape);
    static {};
}
```
