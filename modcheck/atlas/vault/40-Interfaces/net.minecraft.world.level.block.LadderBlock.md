---
type: "interface"
fqcn: "net.minecraft.world.level.block.LadderBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.LadderBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `FACINGLnet/minecraft/world/level/block/state/properties/EnumProper` | `` | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.LadderBlock extends net.minecraft.world.level.block.Block implements net.minecraft.world.level.block.SimpleWaterloggedBlock {
    public static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.core.Direction> FACING;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty WATERLOGGED;
    public static final java.util.Map<net.minecraft.core.Direction, net.minecraft.world.phys.shapes.VoxelShape> SHAPES;
    protected net.minecraft.world.level.block.LadderBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.world.phys.shapes.VoxelShape getShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.world.phys.shapes.CollisionContext);
    private boolean canAttachTo(net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    protected boolean canSurvive(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.core.BlockPos);
    protected net.minecraft.world.level.block.state.BlockState updateShape(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.LevelReader, net.minecraft.world.level.ScheduledTickAccess, net.minecraft.core.BlockPos, net.minecraft.core.Direction, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    protected net.minecraft.world.level.block.state.BlockState rotate(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Rotation);
    protected net.minecraft.world.level.block.state.BlockState mirror(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Mirror);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    protected net.minecraft.world.level.material.FluidState getFluidState(net.minecraft.world.level.block.state.BlockState);
    static {};
}
```
