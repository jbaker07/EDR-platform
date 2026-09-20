---
type: "interface"
fqcn: "net.minecraft.world.level.block.entity.ListBackedContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.entity.ListBackedContainer

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/Container`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `setItem` | `(ILnet/minecraft/world/item/ItemStack;)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getItems()Lnet/minecraft/core/NonNullList;
public count()I
public getContainerSize()I
public clearContent()V
public isEmpty()Z
public getItem(I)Lnet/minecraft/world/item/ItemStack;
public removeItem(II)Lnet/minecraft/world/item/ItemStack;
public removeItemNoUpdate(I)Lnet/minecraft/world/item/ItemStack;
public canPlaceItem(ILnet/minecraft/world/item/ItemStack;)Z
public acceptsItemType(Lnet/minecraft/world/item/ItemStack;)Z
public setItem(ILnet/minecraft/world/item/ItemStack;)V
public setItemNoUpdate(ILnet/minecraft/world/item/ItemStack;)V
```
