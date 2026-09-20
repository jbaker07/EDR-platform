---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.AbstractFurnaceBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/block/entity/BaseContainerBlockEntity`; implements `net/minecraft/world/WorldlyContainer`, `net/minecraft/world/inventory/StackedContentsCompatible`, `net/minecraft/world/inventory/RecipeCraftingHolder`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getTotalCookTime` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/bl` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| injects_into | `consumeFuel` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `cookingTimer` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| reads | `cookingTotalTime` | `I` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| reads | `items` | `Lnet/minecraft/core/NonNullList;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |
| wraps | `consumeFuel` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (28 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected static final SLOT_INPUT : I
protected static final SLOT_FUEL : I
protected static final SLOT_RESULT : I
public static final DATA_LIT_TIME : I
private static final SLOTS_FOR_UP : [I
private static final SLOTS_FOR_DOWN : [I
private static final SLOTS_FOR_SIDES : [I
public static final DATA_LIT_DURATION : I
public static final DATA_COOKING_PROGRESS : I
public static final DATA_COOKING_TOTAL_TIME : I
public static final NUM_DATA_VALUES : I
public static final BURN_TIME_STANDARD : I
public static final BURN_COOL_SPEED : I
private static final RECIPES_USED_CODEC : Lcom/mojang/serialization/Codec;
private static final DEFAULT_COOKING_TIMER : I
private static final DEFAULT_COOKING_TOTAL_TIME : I
private static final DEFAULT_LIT_TIME_REMAINING : I
private static final DEFAULT_LIT_TOTAL_TIME : I
private static final DEFAULT_SPEED_MULTIPLIER : F
protected items : Lnet/minecraft/core/NonNullList;
private litTimeRemaining : I
private litTotalTime : I
private cookingTimer : I
private cookingTotalTime : I
private speedMultiplier : F
protected final dataAccess : Lnet/minecraft/world/inventory/ContainerData;
private final recipesUsed : Lit/unimi/dsi/fastutil/objects/Reference2IntOpenHashMap;
private final quickCheck : Lnet/minecraft/world/item/crafting/RecipeManager$CachedCheck;
protected <init>(Lnet/minecraft/world/level/block/entity/BlockEntityType;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/item/crafting/RecipeType;)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public static serverTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity;)V
private static consumeFuel(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/NonNullList;Lnet/minecraft/world/item/ItemStack;)V
private static canBurn(Lnet/minecraft/core/NonNullList;ILnet/minecraft/world/item/ItemStack;)Z
private static burn(Lnet/minecraft/core/NonNullList;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)V
protected getBurnDuration(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)I
protected getSpeedMultiplier(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)F
private static getTotalCookTime(Lnet/minecraft/world/item/crafting/RecipeHolder;Lnet/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity;)I
private static getTotalCookTime(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity;)I
public getSlotsForFace(Lnet/minecraft/core/Direction;)[I
public canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public canTakeItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public getContainerSize()I
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z
public setRecipeUsed(Lnet/minecraft/world/item/crafting/RecipeHolder;)V
public getRecipeUsed()Lnet/minecraft/world/item/crafting/RecipeHolder;
public awardUsedRecipes(Lnet/minecraft/world/entity/player/Player;Ljava/util/List;)V
public awardUsedRecipesAndPopExperience(Lnet/minecraft/server/level/ServerPlayer;)V
public getRecipesToAwardAndPopExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/phys/Vec3;)Ljava/util/List;
private static createExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/phys/Vec3;IF)V
public fillStackedContents(Lnet/minecraft/world/entity/player/StackedItemContents;)V
public preRemoveSideEffects(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
private static synthetic lambda$getRecipesToAwardAndPopExperience$0(Ljava/util/List;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/phys/Vec3;Lit/unimi/dsi/fastutil/objects/Reference2IntMap$Entry;Lnet/minecraft/world/item/crafting/RecipeHolder;)V
private static synthetic lambda$getTotalCookTime$0(Lnet/minecraft/world/level/block/entity/AbstractFurnaceBlockEntity;Lnet/minecraft/world/item/crafting/RecipeHolder;)Ljava/lang/Integer;
static <clinit>()V
```
