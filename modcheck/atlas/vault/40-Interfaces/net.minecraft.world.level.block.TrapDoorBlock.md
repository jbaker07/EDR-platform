---
type: "interface"
fqcn: "net.minecraft.world.level.block.TrapDoorBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.TrapDoorBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `FACINGLnet/minecraft/world/level/block/state/properties/EnumProper` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.TrapDoorBlock extends net.minecraft.world.level.block.HorizontalDirectionalBlock implements net.minecraft.world.level.block.SimpleWaterloggedBlock {
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty OPEN;
    public static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.world.level.block.state.properties.Half> HALF;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty POWERED;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty WATERLOGGED;
    private static final java.util.Map<net.minecraft.core.Direction, net.minecraft.world.phys.shapes.VoxelShape> SHAPES;
    private final net.minecraft.world.level.block.state.properties.BlockSetType type;
    protected net.minecraft.world.level.block.TrapDoorBlock(net.minecraft.world.level.block.state.properties.BlockSetType, net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    protected boolean isPathfindable(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.pathfinder.PathComputationType);
    protected net.minecraft.world.InteractionResult useWithoutItem(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player, net.minecraft.world.phys.BlockHitResult);
    protected void onExplosionHit(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.Explosion, java.util.function.BiConsumer<net.minecraft.world.item.ItemStack, net.minecraft.core.BlockPos>);
    private void toggle(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player);
    protected void playSound(net.minecraft.world.entity.player.Player, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, boolean);
    protected void neighborChanged(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.Block, net.minecraft.world.level.redstone.Orientation, boolean);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    protected net.minecraft.world.level.material.FluidState getFluidState(net.minecraft.world.level.block.state.BlockState);
    protected net.minecraft.world.level.block.state.BlockState updateShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource);
    protected net.minecraft.world.level.block.state.properties.BlockSetType getType();
    static {};
}
```
