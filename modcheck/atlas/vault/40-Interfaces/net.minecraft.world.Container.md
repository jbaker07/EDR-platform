---
type: "interface"
fqcn: "net.minecraft.world.Container"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.Container

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/Clearable`, `java/lang/Iterable`, `net/minecraft/world/entity/SlotProvider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `canPlaceItem` | `(ILnet/minecraft/world/item/ItemStack;)Z` | exact | invokeinterface@40 in `ContainerSlotWrapper.canInsert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getContainerSize` | `()I` | exact | invokeinterface@4 in `ContainerStorageImpl.resizeSlotList` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getItem` | `(I)Lnet/minecraft/world/item/ItemStack;` | exact | invokeinterface@11 in `ContainerSlotWrapper.getStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxStackSize` | `()I` | exact | invokeinterface@74 in `ContainerSlotWrapper.getCapacity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getMaxStackSize` | `(Lnet/minecraft/world/item/ItemStack;)I` | exact | invokeinterface@95 in `ContainerSlotWrapper.getCapacity` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setChanged` | `()V` | exact | invokeinterface@7 in `ContainerStorageImpl$SetChangedParticipant.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | exact | invokeinterface@19 in `ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | exact | invokeinterface@49 in `ContainerSlotWrapper.setStack` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `toString` | `()Ljava/lang/String;` | inherited_exact | invokeinterface@28 in `DebugMessages.forInventory` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (1 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DEFAULT_DISTANCE_BUFFER : F
public abstract getContainerSize()I
public abstract isEmpty()Z
public abstract getItem(I)Lnet/minecraft/world/item/ItemStack;
public abstract removeItem(II)Lnet/minecraft/world/item/ItemStack;
public abstract removeItemNoUpdate(I)Lnet/minecraft/world/item/ItemStack;
public abstract setItem(ILnet/minecraft/world/item/ItemStack;)V
public getMaxStackSize()I
public getMaxStackSize(Lnet/minecraft/world/item/ItemStack;)I
public abstract setChanged()V
public abstract stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public startOpen(Lnet/minecraft/world/entity/ContainerUser;)V
public stopOpen(Lnet/minecraft/world/entity/ContainerUser;)V
public getEntitiesWithContainerOpen()Ljava/util/List;
public canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z
public canTakeItem(Lnet/minecraft/world/Container;ILnet/minecraft/world/item/ItemStack;)Z
public countItem(Lnet/minecraft/world/item/Item;)I
public hasAnyOf(Ljava/util/Set;)Z
public hasAnyMatching(Ljava/util/function/Predicate;)Z
public static stillValidBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/player/Player;)Z
public static stillValidBlockEntity(Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/player/Player;F)Z
public getSlot(I)Lnet/minecraft/world/entity/SlotAccess;
public iterator()Ljava/util/Iterator;
private static synthetic lambda$hasAnyOf$0(Ljava/util/Set;Lnet/minecraft/world/item/ItemStack;)Z
```
