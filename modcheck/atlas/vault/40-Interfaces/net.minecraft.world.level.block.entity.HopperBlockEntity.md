---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.HopperBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.HopperBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `ejectItems` | `@Inject at INVOKE_ASSIGN Lnet/minecraft/world/level/block/entity/HopperBlockEnti` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `suckInItems(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;)Z` | `@Inject at INVOKE_ASSIGN Lnet/minecraft/world/level/block/entity/HopperBlockEnti` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (53, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.HopperBlockEntity extends net.minecraft.world.level.block.entity.RandomizableContainerBlockEntity implements net.minecraft.world.level.block.entity.Hopper {
    public static final int MOVE_ITEM_SPEED;
    public static final int HOPPER_CONTAINER_SIZE;
    private static final int[][] CACHED_SLOTS;
    private static final int NO_COOLDOWN_TIME;
    private static final net.minecraft.network.chat.Component DEFAULT_NAME;
    private net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    private int cooldownTime;
    private long tickedGameTime;
    private net.minecraft.core.Direction facing;
    public net.minecraft.world.level.block.entity.HopperBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public int getContainerSize();
    public net.minecraft.world.item.ItemStack removeItem(int, int);
    public void setItem(int, net.minecraft.world.item.ItemStack);
    public void setBlockState(net.minecraft.world.level.block.state.BlockState);
    protected net.minecraft.network.chat.Component getDefaultName();
    public static void pushItemsTick(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.HopperBlockEntity);
    private static boolean tryMoveItems(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.HopperBlockEntity, java.util.function.BooleanSupplier);
    private boolean inventoryFull();
    private static boolean ejectItems(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.HopperBlockEntity);
    private static int[] getSlots(net.minecraft.world.Container, net.minecraft.core.Direction);
    private static int[] createFlatSlots(int);
    private static boolean isFullContainer(net.minecraft.world.Container, net.minecraft.core.Direction);
    public static boolean suckInItems(net.minecraft.world.level.Level, net.minecraft.world.level.block.entity.Hopper);
    private static boolean tryTakeInItemFromSlot(net.minecraft.world.level.block.entity.Hopper, net.minecraft.world.Container, int, net.minecraft.core.Direction);
    public static boolean addItem(net.minecraft.world.Container, net.minecraft.world.entity.item.ItemEntity);
    public static net.minecraft.world.item.ItemStack addItem(net.minecraft.world.Container, net.minecraft.world.Container, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    private static boolean canPlaceItemInContainer(net.minecraft.world.Container, net.minecraft.world.item.ItemStack, int, net.minecraft.core.Direction);
    private static boolean canTakeItemFromContainer(net.minecraft.world.Container, net.minecraft.world.Container, net.minecraft.world.item.ItemStack, int, net.minecraft.core.Direction);
    private static net.minecraft.world.item.ItemStack tryMoveInItem(net.minecraft.world.Container, net.minecraft.world.Container, net.minecraft.world.item.ItemStack, int, net.minecraft.core.Direction);
    private static net.minecraft.world.Container getAttachedContainer(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.HopperBlockEntity);
    private static net.minecraft.world.Container getSourceContainer(net.minecraft.world.level.Level, net.minecraft.world.level.block.entity.Hopper, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    public static java.util.List<net.minecraft.world.entity.item.ItemEntity> getItemsAtAndAbove(net.minecraft.world.level.Level, net.minecraft.world.level.block.entity.Hopper);
    public static net.minecraft.world.Container getContainerAt(net.minecraft.world.level.Level, net.minecraft.core.BlockPos);
    private static net.minecraft.world.Container getContainerAt(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, double, double, double);
    private static net.minecraft.world.Container getBlockContainer(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private static net.minecraft.world.Container getEntityContainer(net.minecraft.world.level.Level, double, double, double);
    private static boolean canMergeItems(net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack);
    public double getLevelX();
    public double getLevelY();
    public double getLevelZ();
    public boolean isGridAligned();
    private void setCooldown(int);
    private boolean isOnCooldown();
    private boolean isOnCustomCooldown();
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    public static void entityInside(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.entity.Entity, net.minecraft.world.level.block.entity.HopperBlockEntity);
    protected net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    private static boolean lambda$entityInside$0(net.minecraft.world.level.block.entity.HopperBlockEntity, net.minecraft.world.entity.item.ItemEntity);
    private static boolean lambda$pushItemsTick$0(net.minecraft.world.level.Level, net.minecraft.world.level.block.entity.HopperBlockEntity);
    static {};
}
```
