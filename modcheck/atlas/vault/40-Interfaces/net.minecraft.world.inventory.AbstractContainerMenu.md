---
type: "interface"
fqcn: "net.minecraft.world.inventory.AbstractContainerMenu"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.inventory.AbstractContainerMenu

System: [[20-Systems/net.minecraft.world.inventory|net.minecraft.world.inventory]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createCarriedSlotAccess` | `()Lnet/minecraft/world/entity/SlotAccess;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | declared |
| calls | `getCarried` | `()Lnet/minecraft/world/item/ItemStack;` | exact | invokevirtual@4 in `CursorSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@25 in `Networking.sendOpenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@40 in `ServerPlayerMixin.fabric_storeOpenedMenu` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@53 in `ServerPlayerMixin.fabric_storeOpenedMenu` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@65 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@97 in `ServerPlayerMixin.fabric_replaceVanillaScreenPacket` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/inventory/MenuType;` | exact | invokevirtual@14 in `CursorSlotWrapper.toString` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setCarried` | `(Lnet/minecraft/world/item/ItemStack;)V` | exact | invokevirtual@5 in `CursorSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `tryItemClickBehaviourOverride` | `(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/invent` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (29 fields, 72 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final SLOT_CLICKED_OUTSIDE : I
public static final CONTAINER_CLICK_PRIMARY : I
public static final CONTAINER_CLICK_SECONDARY : I
public static final QUICKCRAFT_TYPE_CHARITABLE : I
public static final QUICKCRAFT_TYPE_GREEDY : I
public static final QUICKCRAFT_TYPE_CLONE : I
public static final QUICKCRAFT_HEADER_START : I
public static final QUICKCRAFT_HEADER_CONTINUE : I
public static final QUICKCRAFT_HEADER_END : I
public static final CARRIED_SLOT_SIZE : I
public static final SLOTS_PER_ROW : I
public static final SLOT_SIZE : I
private final lastSlots : Lnet/minecraft/core/NonNullList;
public final slots : Lnet/minecraft/core/NonNullList;
private final dataSlots : Ljava/util/List;
private carried : Lnet/minecraft/world/item/ItemStack;
private final remoteSlots : Lnet/minecraft/core/NonNullList;
private final remoteDataSlots : Lit/unimi/dsi/fastutil/ints/IntList;
private remoteCarried : Lnet/minecraft/world/inventory/RemoteSlot;
private stateId : I
private final menuType : Lnet/minecraft/world/inventory/MenuType;
public final containerId : I
private quickcraftType : I
private quickcraftStatus : I
private final quickcraftSlots : Ljava/util/Set;
private final containerListeners : Ljava/util/List;
private synchronizer : Lnet/minecraft/world/inventory/ContainerSynchronizer;
private suppressRemoteUpdates : Z
protected <init>(Lnet/minecraft/world/inventory/MenuType;I)V
protected addInventoryHotbarSlots(Lnet/minecraft/world/Container;II)V
protected addInventoryExtendedSlots(Lnet/minecraft/world/Container;II)V
protected addStandardInventorySlots(Lnet/minecraft/world/Container;II)V
protected static stillValid(Lnet/minecraft/world/inventory/ContainerLevelAccess;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/block/Block;)Z
public getType()Lnet/minecraft/world/inventory/MenuType;
protected static checkContainerSize(Lnet/minecraft/world/Container;I)V
protected static checkContainerDataCount(Lnet/minecraft/world/inventory/ContainerData;I)V
public isValidSlotIndex(I)Z
protected addSlot(Lnet/minecraft/world/inventory/Slot;)Lnet/minecraft/world/inventory/Slot;
protected addDataSlot(Lnet/minecraft/world/inventory/DataSlot;)Lnet/minecraft/world/inventory/DataSlot;
protected addDataSlots(Lnet/minecraft/world/inventory/ContainerData;)V
public addSlotListener(Lnet/minecraft/world/inventory/ContainerListener;)V
public setSynchronizer(Lnet/minecraft/world/inventory/ContainerSynchronizer;)V
public sendAllDataToRemote()V
public removeSlotListener(Lnet/minecraft/world/inventory/ContainerListener;)V
public getItems()Lnet/minecraft/core/NonNullList;
public broadcastChanges()V
public broadcastFullState()V
private updateDataSlotListeners(II)V
private triggerSlotListeners(ILnet/minecraft/world/item/ItemStack;Ljava/util/function/Supplier;)V
private synchronizeSlotToRemote(ILnet/minecraft/world/item/ItemStack;Ljava/util/function/Supplier;)V
private synchronizeDataSlotToRemote(II)V
private synchronizeCarriedToRemote()V
public setRemoteSlot(ILnet/minecraft/world/item/ItemStack;)V
public setRemoteSlotUnsafe(ILnet/minecraft/network/HashedStack;)V
public setRemoteCarried(Lnet/minecraft/network/HashedStack;)V
public clickMenuButton(Lnet/minecraft/world/entity/player/Player;I)Z
public getSlot(I)Lnet/minecraft/world/inventory/Slot;
public abstract quickMoveStack(Lnet/minecraft/world/entity/player/Player;I)Lnet/minecraft/world/item/ItemStack;
public setSelectedBundleItemIndex(II)V
public clicked(IILnet/minecraft/world/inventory/ContainerInput;Lnet/minecraft/world/entity/player/Player;)V
private doClick(IILnet/minecraft/world/inventory/ContainerInput;Lnet/minecraft/world/entity/player/Player;)V
private static isContainerClickButton(I)Z
private static getClickAction(I)Lnet/minecraft/world/inventory/ClickAction;
private tryItemClickBehaviourOverride(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/inventory/ClickAction;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)Z
private createCarriedSlotAccess()Lnet/minecraft/world/entity/SlotAccess;
public canTakeItemForPickAll(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/Slot;)Z
public removed(Lnet/minecraft/world/entity/player/Player;)V
private static dropOrPlaceInInventory(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;)V
protected clearContainer(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/Container;)V
public slotsChanged(Lnet/minecraft/world/Container;)V
public setItem(IILnet/minecraft/world/item/ItemStack;)V
public initializeContents(ILjava/util/List;Lnet/minecraft/world/item/ItemStack;)V
public setData(II)V
public abstract stillValid(Lnet/minecraft/world/entity/player/Player;)Z
protected moveItemStackTo(Lnet/minecraft/world/item/ItemStack;IIZ)Z
public static getQuickcraftType(I)I
public static getQuickcraftHeader(I)I
public static getQuickcraftMask(II)I
public static isValidQuickcraftType(ILnet/minecraft/world/entity/player/Player;)Z
protected resetQuickCraft()V
public static canItemQuickReplace(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/item/ItemStack;Z)Z
public static getQuickCraftPlaceCount(IILnet/minecraft/world/item/ItemStack;)I
public canDragTo(Lnet/minecraft/world/inventory/Slot;)Z
public static getRedstoneSignalFromBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;)I
public static getRedstoneSignalFromContainer(Lnet/minecraft/world/Container;)I
public setCarried(Lnet/minecraft/world/item/ItemStack;)V
public getCarried()Lnet/minecraft/world/item/ItemStack;
public suppressRemoteUpdates()V
public resumeRemoteUpdates()V
public transferState(Lnet/minecraft/world/inventory/AbstractContainerMenu;)V
public findSlot(Lnet/minecraft/world/Container;I)Ljava/util/OptionalInt;
public getStateId()I
public incrementStateId()I
private static synthetic lambda$doClick$1(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;)V
private synthetic lambda$doClick$0(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;)V
private synthetic lambda$clicked$1()Ljava/lang/String;
private synthetic lambda$clicked$0()Ljava/lang/String;
private static synthetic lambda$setSynchronizer$0(Lnet/minecraft/world/inventory/ContainerSynchronizer;Lnet/minecraft/world/inventory/RemoteSlot;)Lnet/minecraft/world/inventory/RemoteSlot;
private static synthetic lambda$stillValid$0(Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Ljava/lang/Boolean;
static <clinit>()V
```
