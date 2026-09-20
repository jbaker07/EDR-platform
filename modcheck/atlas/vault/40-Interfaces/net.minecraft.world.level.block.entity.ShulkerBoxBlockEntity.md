---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ShulkerBoxBlockEntity

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/entity/RandomizableContainerBlockEntity`; implements `net/minecraft/world/WorldlyContainer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `canPlaceItemThroughFace` | `(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z` | exact | invokevirtual@27 in `ContainerSlotWrapper.canInsert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (15 fields, 27 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final COLUMNS : I
public static final ROWS : I
public static final CONTAINER_SIZE : I
public static final EVENT_SET_OPEN_COUNT : I
public static final OPENING_TICK_LENGTH : I
public static final MAX_LID_HEIGHT : F
public static final MAX_LID_ROTATION : F
private static final SLOTS : [I
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private itemStacks : Lnet/minecraft/core/NonNullList;
private openCount : I
private animationStatus : Lnet/minecraft/world/level/block/entity/ShulkerBoxBlockEntity$AnimationStatus;
private progress : F
private progressOld : F
private final color : Lnet/minecraft/world/item/DyeColor;
public <init>(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public <init>(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public static tick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/ShulkerBoxBlockEntity;)V
private updateAnimation(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getAnimationStatus()Lnet/minecraft/world/level/block/entity/ShulkerBoxBlockEntity$AnimationStatus;
public getBoundingBox(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/phys/AABB;
private moveCollidedEntities(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getContainerSize()I
public triggerEvent(II)Z
private static doNeighborUpdates(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public preRemoveSideEffects(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public startOpen(Lnet/minecraft/world/entity/ContainerUser;)V
public stopOpen(Lnet/minecraft/world/entity/ContainerUser;)V
protected getDefaultName()Lnet/minecraft/network/chat/Component;
protected loadAdditional(Lnet/minecraft/world/level/storage/ValueInput;)V
protected saveAdditional(Lnet/minecraft/world/level/storage/ValueOutput;)V
public loadFromTag(Lnet/minecraft/world/level/storage/ValueInput;)V
protected getItems()Lnet/minecraft/core/NonNullList;
protected setItems(Lnet/minecraft/core/NonNullList;)V
public getSlotsForFace(Lnet/minecraft/core/Direction;)[I
public canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public canTakeItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public getProgress(F)F
public getColor()Lnet/minecraft/world/item/DyeColor;
protected createMenu(ILnet/minecraft/world/entity/player/Inventory;)Lnet/minecraft/world/inventory/AbstractContainerMenu;
public isClosed()Z
static <clinit>()V
```
