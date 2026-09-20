---
type: "interface"
fqcn: "net.minecraft.world.level.block.DispenserBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.DispenserBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `FACINGLnet/minecraft/world/level/block/state/properties/EnumProper` | `` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (26, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.DispenserBlock extends net.minecraft.world.level.block.BaseEntityBlock {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.core.Direction> FACING;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty TRIGGERED;
    private static final net.minecraft.core.dispenser.DefaultDispenseItemBehavior DEFAULT_BEHAVIOR;
    public static final java.util.Map<net.minecraft.world.item.Item, net.minecraft.core.dispenser.DispenseItemBehavior> DISPENSER_REGISTRY;
    private static final int TRIGGER_DURATION;
    public static void registerBehavior(net.minecraft.world.level.ItemLike, net.minecraft.core.dispenser.DispenseItemBehavior);
    public static void registerProjectileBehavior(net.minecraft.world.level.ItemLike);
    protected net.minecraft.world.level.block.DispenserBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected net.minecraft.world.InteractionResult useWithoutItem(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player, net.minecraft.world.phys.BlockHitResult);
    protected void dispenseFrom(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos);
    protected net.minecraft.core.dispenser.DispenseItemBehavior getDispenseMethod(net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack);
    private static net.minecraft.core.dispenser.DispenseItemBehavior getDefaultDispenseMethod(net.minecraft.world.item.ItemStack);
    protected void neighborChanged(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.Block, net.minecraft.world.level.redstone.Orientation, boolean);
    protected void tick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public net.minecraft.world.level.block.entity.BlockEntity newBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    protected void affectNeighborsAfterRemoval(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, boolean);
    public static net.minecraft.core.Position getDispensePosition(net.minecraft.core.dispenser.BlockSource);
    public static net.minecraft.core.Position getDispensePosition(net.minecraft.core.dispenser.BlockSource, double, net.minecraft.world.phys.Vec3);
    protected boolean hasAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState);
    protected int getAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    protected net.minecraft.world.level.block.state.BlockState rotate(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Rotation);
    protected net.minecraft.world.level.block.state.BlockState mirror(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Mirror);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    static {};
}
```
