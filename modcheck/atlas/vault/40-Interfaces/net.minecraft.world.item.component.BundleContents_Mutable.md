---
type: "interface"
fqcn: "net.minecraft.world.item.component.BundleContents$Mutable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.component.BundleContents$Mutable

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`class` public; extends `net/minecraft/world/item/component/GrowableMutableContainer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `toImmutable` | `()Lnet/minecraft/world/item/component/BundleContents;` | exact | invokevirtual@79 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `tryInsert` | `(Lnet/minecraft/world/item/ItemStack;)I` | exact | invokevirtual@59 in `BundleContentsStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private weight : Lorg/apache/commons/lang3/math/Fraction;
private selectedItem : I
private needsFlattening : Z
private <init>(Ljava/util/List;Lorg/apache/commons/lang3/math/Fraction;I)V
public <init>()V
public clearItems()Lnet/minecraft/world/item/component/BundleContents$Mutable;
private findStackIndexWithinRange(Lnet/minecraft/world/item/ItemStack;II)I
private findStackIndex(Lnet/minecraft/world/item/ItemStack;)I
private getMaxAmountToAdd(Lorg/apache/commons/lang3/math/Fraction;)I
public tryInsert(Lnet/minecraft/world/item/ItemStack;)I
public tryTransfer(Lnet/minecraft/world/inventory/Slot;Lnet/minecraft/world/entity/player/Player;)I
public toggleSelectedItem(I)V
private indexIsOutsideAllowedBounds(I)Z
public removeOne()Lnet/minecraft/world/item/ItemStack;
private static getStackedWeight(Lorg/apache/commons/lang3/math/Fraction;I)Lorg/apache/commons/lang3/math/Fraction;
private static getStackedWeight(Lnet/minecraft/world/item/ItemStack;)Lorg/apache/commons/lang3/math/Fraction;
public weight()Lorg/apache/commons/lang3/math/Fraction;
public replaceSlotItems(Lnet/minecraft/world/item/ItemProvider;Lnet/minecraft/world/item/slot/SlotSelector;)I
public modifySlots(Ljava/util/function/Consumer;Lnet/minecraft/world/item/slot/SlotSelector;)V
protected setItem(ILnet/minecraft/world/item/ItemStack;)Z
protected addSlotWithItem(Lnet/minecraft/world/item/ItemProvider;)Z
private static getWeightWithAddedItems(Lorg/apache/commons/lang3/math/Fraction;Lnet/minecraft/world/item/ItemStack;)Lorg/apache/commons/lang3/math/Fraction;
public canInsertNewSlots()Z
private mergeIdenticalStacks()V
public toImmutable()Lnet/minecraft/world/item/component/BundleContents;
public synthetic toImmutable()Ljava/lang/Object;
```
