---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Inventory"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Inventory

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/Container`, `net/minecraft/world/Nameable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getSelectedSlot` | `()I` | exact | invokevirtual@1 in `MouseHandlerMixin.wrapSelectedSlot` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getSelectedSlot` | `()I` | exact | invokevirtual@14 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getSelectedSlot` | `()I` | exact | invokevirtual@28 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getSelectedSlot` | `()I` | exact | invokevirtual@43 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `isHotbarSlot` | `(I)Z` | exact | invokestatic@17 in `PlayerInventoryStorageImpl.getHandSlot` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/world/entity/player/Player;` | exact | getfield@20 in `DebugMessages.forInventory` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/world/entity/player/Player;` | exact | getfield@62 in `PlayerInventoryStorageImpl$DroppedStacks.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/world/entity/player/Player;` | exact | getfield@101 in `PlayerInventoryStorageImpl$DroppedStacks.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `player` | `Lnet/minecraft/world/entity/player/Player;` | exact | getfield@15 in `PlayerInventoryStorageImpl.drop` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (14 fields, 49 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final POP_TIME_DURATION : I
public static final INVENTORY_SIZE : I
public static final SELECTION_SIZE : I
public static final SLOT_OFFHAND : I
public static final SLOT_BODY_ARMOR : I
public static final SLOT_SADDLE : I
public static final NOT_FOUND_INDEX : I
public static final EQUIPMENT_SLOT_MAPPING : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private static final DEFAULT_NAME : Lnet/minecraft/network/chat/Component;
private final items : Lnet/minecraft/core/NonNullList;
private selected : I
public final player : Lnet/minecraft/world/entity/player/Player;
private final equipment : Lnet/minecraft/world/entity/EntityEquipment;
private timesChanged : I
public <init>(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/EntityEquipment;)V
public getSelectedSlot()I
public setSelectedSlot(I)V
public getSelectedItem()Lnet/minecraft/world/item/ItemStack;
public setSelectedItem(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
public static getSelectionSize()I
public getNonEquipmentItems()Lnet/minecraft/core/NonNullList;
private hasRemainingSpaceForItem(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
public getFreeSlot()I
public addAndPickItem(Lnet/minecraft/world/item/ItemStack;)V
public pickSlot(I)V
public static isHotbarSlot(I)Z
public findSlotMatchingItem(Lnet/minecraft/world/item/ItemStack;)I
public static isUsableForCrafting(Lnet/minecraft/world/item/ItemStack;)Z
public findSlotMatchingCraftingIngredient(Lnet/minecraft/core/Holder;Lnet/minecraft/world/item/ItemStack;)I
public getSuitableHotbarSlot()I
public clearOrCountMatchingItems(Ljava/util/function/Predicate;ZILnet/minecraft/world/Container;)I
private addResource(Lnet/minecraft/world/item/ItemStack;)I
private addResource(ILnet/minecraft/world/item/ItemStack;)I
public getSlotWithRemainingSpace(Lnet/minecraft/world/item/ItemStack;)I
public tick()V
public add(Lnet/minecraft/world/item/ItemStack;)Z
public add(ILnet/minecraft/world/item/ItemStack;)Z
public placeItemBackInInventory(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/util/Prediction;)V
public placeItemBackInInventory(Lnet/minecraft/world/item/ItemStack;ZLnet/minecraft/util/Prediction;)V
public createInventoryUpdatePacket(I)Lnet/minecraft/network/protocol/game/ClientboundSetPlayerInventoryPacket;
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public removeItem(Lnet/minecraft/world/item/ItemStack;)V
public removeItemNoUpdate(I)Lnet/minecraft/world/item/ItemStack;
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public save(Lnet/minecraft/world/level/storage/ValueOutput$TypedOutputList;)V
public load(Lnet/minecraft/world/level/storage/ValueInput$TypedInputList;)V
public getContainerSize()I
public isEmpty()Z
public getItem(I)Lnet/minecraft/world/item/ItemStack;
public getName()Lnet/minecraft/network/chat/Component;
public dropAll()V
public setChanged()V
public getTimesChanged()I
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public contains(Lnet/minecraft/world/item/ItemStack;)Z
public contains(Lnet/minecraft/tags/TagKey;)Z
public contains(Ljava/util/function/Predicate;)Z
public replaceWith(Lnet/minecraft/world/entity/player/Inventory;)V
public clearContent()V
public fillStackedContents(Lnet/minecraft/world/entity/player/StackedItemContents;)V
public removeFromSelected(Z)Lnet/minecraft/world/item/ItemStack;
private static synthetic lambda$add$0(Lnet/minecraft/world/item/ItemStack;)Ljava/lang/String;
static <clinit>()V
```
