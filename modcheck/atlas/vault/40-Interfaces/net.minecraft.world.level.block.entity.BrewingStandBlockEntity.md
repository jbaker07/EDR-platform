---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.BrewingStandBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.BrewingStandBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/BaseContainerBlockEntity`; implements `net/minecraft/world/WorldlyContainer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `doBrew` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (26 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final INGREDIENT_SLOT : I
private static final FUEL_SLOT : I
private static final SLOTS_FOR_UP : [I
private static final SLOTS_FOR_DOWN : [I
private static final SLOTS_FOR_SIDES : [I
public static final DATA_BREW_TIME : I
public static final DATA_FUEL_USES : I
public static final DATA_TOTAL_BREW_TIME : I
public static final DATA_TOTAL_FUEL_USES : I
public static final NUM_DATA_VALUES : I
private static final DEFAULT_BREW_TIME : I
public static final BREWING_TIME_SECONDS : I
private static final DEFAULT_FUEL : I
private static final DEFAULT_SPEED_MULTIPLIER : F
private static final DEFAULT_FUEL_USES : I
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private items : Lnet/minecraft/core/NonNullList;
private brewTime : I
private totalBrewTime : I
private lastPotionCount : [Z
private ingredient : Lnet/minecraft/world/item/Item;
private fuel : I
private totalFuel : I
private speedMultiplier : F
protected final dataAccess : Lnet/minecraft/world/inventory/ContainerData;
private final quickCheck : Lnet/minecraft/world/item/crafting/RecipeManager$CachedCheck;
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected getDefaultName()Lnet/minecraft/network/chat/Component;
public getContainerSize()I
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
protected getUses(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/component/BrewingFuel;)I
protected getSpeedMultiplier(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/component/BrewingFuel;)F
public static serverTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/BrewingStandBlockEntity;)V
private getPotionBits()[Z
private static isBrewable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/block/entity/BrewingStandBlockEntity;)Z
private static doBrew(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BrewingStandBlockEntity;)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z
public getSlotsForFace(Lnet/minecraft/core/Direction;)[I
public canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public canTakeItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
protected createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
static <clinit>()V
```
