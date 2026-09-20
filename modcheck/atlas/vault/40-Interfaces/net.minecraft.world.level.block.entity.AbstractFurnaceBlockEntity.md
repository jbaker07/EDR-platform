---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `consumeFuel` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `setItem` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `consumeFuel` | `@Redirect at INVOKE Lnet/minecraft/world/item/Item;getCraftingRemainder()Lnet/mi` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (58, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity extends net.minecraft.world.level.block.entity.BaseContainerBlockEntity implements net.minecraft.world.WorldlyContainer,net.minecraft.world.inventory.StackedContentsCompatible,net.minecraft.world.inventory.RecipeCraftingHolder {
    protected static final int SLOT_INPUT;
    protected static final int SLOT_FUEL;
    protected static final int SLOT_RESULT;
    public static final int DATA_LIT_TIME;
    private static final int[] SLOTS_FOR_UP;
    private static final int[] SLOTS_FOR_DOWN;
    private static final int[] SLOTS_FOR_SIDES;
    public static final int DATA_LIT_DURATION;
    public static final int DATA_COOKING_PROGRESS;
    public static final int DATA_COOKING_TOTAL_TIME;
    public static final int NUM_DATA_VALUES;
    public static final int BURN_TIME_STANDARD;
    public static final int BURN_COOL_SPEED;
    private static final com.mojang.serialization.Codec<java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>, java.lang.Integer>> RECIPES_USED_CODEC;
    private static final int DEFAULT_COOKING_TIMER;
    private static final int DEFAULT_COOKING_TOTAL_TIME;
    private static final int DEFAULT_LIT_TIME_REMAINING;
    private static final int DEFAULT_LIT_TOTAL_TIME;
    private static final float DEFAULT_SPEED_MULTIPLIER;
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> items;
    private int litTimeRemaining;
    private int litTotalTime;
    private int cookingTimer;
    private int cookingTotalTime;
    private float speedMultiplier;
    protected final net.minecraft.world.inventory.ContainerData dataAccess;
    private final it.unimi.dsi.fastutil.objects.Reference2IntOpenHashMap<net.minecraft.resources.ResourceKey<net.minecraft.world.item.crafting.Recipe<?>>> recipesUsed;
    private final net.minecraft.world.item.crafting.RecipeManager$CachedCheck<net.minecraft.world.item.crafting.SingleRecipeInput, ? extends net.minecraft.world.item.crafting.AbstractCookingRecipe> quickCheck;
    protected net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity(net.minecraft.world.level.block.entity.BlockEntityType<?>, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.item.crafting.RecipeType<? extends net.minecraft.world.item.crafting.AbstractCookingRecipe>);
    protected void loadAdditional(net.minecraft.world.level.storage.ValueInput);
    protected void saveAdditional(net.minecraft.world.level.storage.ValueOutput);
    public static void serverTick(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity);
    private static void consumeFuel(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>, net.minecraft.world.item.ItemStack);
    private static boolean canBurn(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>, int, net.minecraft.world.item.ItemStack);
    private static void burn(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>, net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack);
    protected int getBurnDuration(net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack);
    protected float getSpeedMultiplier(net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack);
    private static int getTotalCookTime(net.minecraft.world.item.crafting.RecipeHolder<? extends net.minecraft.world.item.crafting.AbstractCookingRecipe>, net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity);
    private static int getTotalCookTime(net.minecraft.server.level.ServerLevel, net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity);
    public int[] getSlotsForFace(net.minecraft.core.Direction);
    public boolean canPlaceItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public boolean canTakeItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public int getContainerSize();
    protected net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack> getItems();
    protected void setItems(net.minecraft.core.NonNullList<net.minecraft.world.item.ItemStack>);
    public void setItem(int, net.minecraft.world.item.ItemStack);
    public boolean canPlaceItem(int, net.minecraft.world.item.ItemStack);
    public void setRecipeUsed(net.minecraft.world.item.crafting.RecipeHolder<?>);
    public net.minecraft.world.item.crafting.RecipeHolder<?> getRecipeUsed();
    public void awardUsedRecipes(net.minecraft.world.entity.player.Player, java.util.List<net.minecraft.world.item.ItemStack>);
    public void awardUsedRecipesAndPopExperience(net.minecraft.server.level.ServerPlayer);
    public java.util.List<net.minecraft.world.item.crafting.RecipeHolder<?>> getRecipesToAwardAndPopExperience(net.minecraft.server.level.ServerLevel, net.minecraft.world.phys.Vec3);
    private static void createExperience(net.minecraft.server.level.ServerLevel, net.minecraft.world.phys.Vec3, int, float);
    public void fillStackedContents(net.minecraft.world.entity.player.StackedItemContents);
    public void preRemoveSideEffects(net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState);
    private static void lambda$getRecipesToAwardAndPopExperience$0(java.util.List, net.minecraft.server.level.ServerLevel, net.minecraft.world.phys.Vec3, it.unimi.dsi.fastutil.objects.Reference2IntMap$Entry, net.minecraft.world.item.crafting.RecipeHolder);
    private static java.lang.Integer lambda$getTotalCookTime$0(net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity, net.minecraft.world.item.crafting.RecipeHolder);
    static {};
}
```
