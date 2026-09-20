---
type: "interface"
fqcn: "net.minecraft.world.SimpleContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.SimpleContainer

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/Container`, `net/minecraft/world/inventory/StackedContentsCompatible`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `setChanged` | `()V` | exact | invokevirtual@8 in `SimpleContainerMixin.fabric_redirectChanged` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| wraps | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (2 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final size : I
public final items : Lnet/minecraft/core/NonNullList;
public <init>(I)V
public <init>([Lnet/minecraft/world/item/ItemStack;)V
public getItem(I)Lnet/minecraft/world/item/ItemStack;
public removeAllItems()Ljava/util/List;
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public removeItemType(Lnet/minecraft/world/item/Item;I)Lnet/minecraft/world/item/ItemStack;
public addItem(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
public canAddItem(Lnet/minecraft/world/item/ItemStack;)Z
public removeItemNoUpdate(I)Lnet/minecraft/world/item/ItemStack;
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public setChanged()V
public getContainerSize()I
public isEmpty()Z
public stillValid(Lnet/minecraft/world/entity/player/Player;)Z
public clearContent()V
public fillStackedContents(Lnet/minecraft/world/entity/player/StackedItemContents;)V
public toString()Ljava/lang/String;
private moveItemToEmptySlots(Lnet/minecraft/world/item/ItemStack;)V
private moveItemToOccupiedSlotsWithSameType(Lnet/minecraft/world/item/ItemStack;)V
private moveItemsBetweenStacks(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/item/ItemStack;)V
public fromItemList(Lnet/minecraft/world/level/storage/ValueInput$TypedInputList;)V
public storeAsItemList(Lnet/minecraft/world/level/storage/ValueOutput$TypedOutputList;)V
public getItems()Lnet/minecraft/core/NonNullList;
private static synthetic lambda$toString$0(Lnet/minecraft/world/item/ItemStack;)Z
private static synthetic lambda$removeAllItems$0(Lnet/minecraft/world/item/ItemStack;)Z
```
