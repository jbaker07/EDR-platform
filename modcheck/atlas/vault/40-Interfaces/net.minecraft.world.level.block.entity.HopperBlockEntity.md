---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.HopperBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.HopperBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity`; implements `net/minecraft/world/level/block/entity/Hopper`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `ejectItems` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/mi` | name_only | @Inject at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `suckInItems` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/ent` | exact | @Inject at ['INVOKE_ASSIGN'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `facing` | `Lnet/minecraft/core/Direction;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | declared |

## Declared members (9 fields, 44 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MOVE_ITEM_SPEED : I
public static final HOPPER_CONTAINER_SIZE : I
private static final CACHED_SLOTS : [[I
private static final NO_COOLDOWN_TIME : I
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private items : Lnet/minecraft/core/NonNullList;
private cooldownTime : I
private tickedGameTime : J
private facing : Lnet/minecraft/core/Direction;
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public getContainerSize()I
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public setBlockState(Lnet/minecraft/world/level/block/state/BlockState;)V
protected getDefaultName()Lnet/minecraft/network/chat/Component;
public static pushItemsTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)V
private static tryMoveItems(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;Ljava/util/function/BooleanSupplier;)Z
private inventoryFull()Z
private static ejectItems(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)Z
private static getSlots(Lnet/minecraft/world/Container;Lnet/minecraft/core/Direction;)[I
private static createFlatSlots(I)[I
private static isFullContainer(Lnet/minecraft/world/Container;Lnet/minecraft/core/Direction;)Z
public static suckInItems(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;)Z
private static tryTakeInItemFromSlot(Lnet/minecraft/world/level/block/entity/Hopper;Lnet/minecraft/world/Container;ILnet/minecraft/core/Direction;)Z
public static addItem(Lnet/minecraft/world/Container;Lnet/minecraft/world/entity/item/ItemEntity;)Z
public static addItem(Lnet/minecraft/world/Container;Lnet/minecraft/world/Container;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/item/ItemStack;
private static canPlaceItemInContainer(Lnet/minecraft/world/Container;Lnet/minecraft/world/item/ItemStack;ILnet/minecraft/core/Direction;)Z
private static canTakeItemFromContainer(Lnet/minecraft/world/Container;Lnet/minecraft/world/Container;Lnet/minecraft/world/item/ItemStack;ILnet/minecraft/core/Direction;)Z
private static tryMoveInItem(Lnet/minecraft/world/Container;Lnet/minecraft/world/Container;Lnet/minecraft/world/item/ItemStack;ILnet/minecraft/core/Direction;)Lnet/minecraft/world/item/ItemStack;
private static getAttachedContainer(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)Lnet/minecraft/world/Container;
private static getSourceContainer(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/Container;
public static getItemsAtAndAbove(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/Hopper;)Ljava/util/List;
public static getContainerAt(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/Container;
private static getContainerAt(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;DDD)Lnet/minecraft/world/Container;
private static getBlockContainer(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/Container;
private static getEntityContainer(Lnet/minecraft/world/level/Level;DDD)Lnet/minecraft/world/Container;
private static canMergeItems(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public getLevelX()D
public getLevelY()D
public getLevelZ()D
public isGridAligned()Z
private setCooldown(I)V
private isOnCooldown()Z
private isOnCustomCooldown()Z
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
public static entityInside(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)V
protected createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
private static synthetic lambda$entityInside$0(Lnet/minecraft/world/level/block/entity/HopperBlockEntity;Lnet/minecraft/world/entity/item/ItemEntity;)Z
private static synthetic lambda$pushItemsTick$0(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/HopperBlockEntity;)Z
static <clinit>()V
```
