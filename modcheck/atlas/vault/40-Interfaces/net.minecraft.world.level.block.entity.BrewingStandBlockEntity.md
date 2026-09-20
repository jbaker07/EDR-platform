---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BrewingStandBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BrewingStandBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `doBrew` | `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/mi` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (45, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.block.entity.BrewingStandBlockEntity extends net.minecraft.world.level.block.entity.BaseContainerBlockEntity implements net.minecraft.world.WorldlyContainer {
    private static final int INGREDIENT_SLOT;
    private static final int FUEL_SLOT;
    private static final int[] SLOTS_FOR_UP;
    private static final int[] SLOTS_FOR_DOWN;
    private static final int[] SLOTS_FOR_SIDES;
    public static final int DATA_BREW_TIME;
    public static final int DATA_FUEL_USES;
    public static final int DATA_TOTAL_BREW_TIME;
    public static final int DATA_TOTAL_FUEL_USES;
    public static final int NUM_DATA_VALUES;
    private static final int DEFAULT_BREW_TIME;
    public static final int BREWING_TIME_SECONDS;
    private static final int DEFAULT_FUEL;
    private static final float DEFAULT_SPEED_MULTIPLIER;
    private static final int DEFAULT_FUEL_USES;
    private static final net.minecraft.network.chat.Component DEFAULT_NAME;
    private net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    private int brewTime;
    private int totalBrewTime;
    private boolean[] lastPotionCount;
    private net.minecraft.world.item.Item ingredient;
    private int fuel;
    private int totalFuel;
    private float speedMultiplier;
    protected final net.minecraft.world.inventory.ContainerData dataAccess;
    private final net.minecraft.world.item.crafting.RecipeManager$CachedCheck<net.minecraft.world.item.crafting.BrewingInput, net.minecraft.world.item.crafting.BrewingRecipe> quickCheck;
    public net.minecraft.world.level.block.entity.BrewingStandBlockEntity(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    protected net.minecraft.network.chat.Component getDefaultName();
    public int getContainerSize();
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    protected int getUses(net.minecraft.server.level.ServerLevel, net.minecraft.world.item.component.BrewingFuel);
    protected float getSpeedMultiplier(net.minecraft.server.level.ServerLevel, net.minecraft.world.item.component.BrewingFuel);
    public static void serverTick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BrewingStandBlockEntity);
    private boolean[] getPotionBits();
    private static boolean isBrewable(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.block.entity.BrewingStandBlockEntity);
    private static void doBrew(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.entity.BrewingStandBlockEntity);
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public boolean canPlaceItem(int, net.minecraft.world.item.ItemStack);
    public int[] getSlotsForFace(net.minecraft.core.Direction);
    public boolean canPlaceItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public boolean canTakeItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    protected net.minecraft.world.inventory.AbstractContainerMenu createMenu(int, net.minecraft.world.entity.player.Inventory);
    static {};
}
```
