---
type: "interface"
fqcn: "net.minecraft.world.level.block.CrafterBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.CrafterBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `dispenseItem` | `@Inject at INVOKE Lnet/minecraft/world/item/ItemStack;isEmpty()Z` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.CrafterBlock extends net.minecraft.world.level.block.BaseEntityBlock {
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty CRAFTING;
    public static final net.minecraft.world.level.block.state.properties.BooleanProperty TRIGGERED;
    private static final net.minecraft.world.level.block.state.properties.EnumProperty<net.minecraft.core.FrontAndTop> ORIENTATION;
    private static final int MAX_CRAFTING_TICKS;
    private static final int CRAFTING_TICK_DELAY;
    private static final net.minecraft.world.item.crafting.RecipeCache RECIPE_CACHE;
    private static final int CRAFTER_ADVANCEMENT_DIAMETER;
    public net.minecraft.world.level.block.CrafterBlock(net.minecraft.world.level.block.state.BlockBehaviour$Properties);
    protected boolean hasAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState);
    protected int getAnalogOutputSignal(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.core.Direction);
    protected void neighborChanged(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.Block, net.minecraft.world.level.redstone.Orientation, boolean);
    protected void tick(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public <T extends net.minecraft.world.level.block.entity.BlockEntity> net.minecraft.world.level.block.entity.BlockEntityTicker<T> getTicker(net.minecraft.world.level.Level, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BlockEntityType<T>);
    private void setBlockEntityTriggered(net.minecraft.world.level.block.entity.BlockEntity, boolean);
    public net.minecraft.world.level.block.entity.BlockEntity newBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public net.minecraft.world.level.block.state.BlockState getStateForPlacement(net.minecraft.world.item.context.BlockPlaceContext);
    public void setPlacedBy(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.LivingEntity, net.minecraft.world.item.ItemStack);
    protected void affectNeighborsAfterRemoval(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, boolean);
    protected net.minecraft.world.InteractionResult useWithoutItem(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player, net.minecraft.world.phys.BlockHitResult);
    protected void dispenseFrom(net.minecraft.world.level.block.state.BlockState, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos);
    public static java.util.Optional<net.minecraft.world.item.crafting.RecipeHolder<net.minecraft.world.item.crafting.CraftingRecipe>> getPotentialResults(net.minecraft.server.level.ServerLevel, net.minecraft.world.item.crafting.CraftingInput);
    private void dispenseItem(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.CrafterBlockEntity, net.minecraft.world.item.ItemStack, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.item.crafting.RecipeHolder<?>);
    protected net.minecraft.world.level.block.state.BlockState rotate(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Rotation);
    protected net.minecraft.world.level.block.state.BlockState mirror(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.Mirror);
    protected void createBlockStateDefinition(net.minecraft.world.level.block.state.StateDefinition$Builder<net.minecraft.world.level.block.Block, net.minecraft.world.level.block.state.BlockState>);
    private static void lambda$dispenseFrom$0(net.minecraft.world.item.ItemStack);
    static {};
}
```
