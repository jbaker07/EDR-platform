---
type: "interface"
fqcn: "net.minecraft.world.WorldlyContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.WorldlyContainer

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/Container`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `canPlaceItemThroughFace` | `(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z` | exact | invokeinterface@22 in `WorldlyContainerSlotWrapper.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `canTakeItemThroughFace` | `(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z` | exact | invokeinterface@22 in `WorldlyContainerSlotWrapper.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getSlotsForFace` | `(Lnet/minecraft/core/Direction;)[I` | exact | invokeinterface@10 in `SidedContainerStorageImpl.createWrapperList` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getSlotsForFace(Lnet/minecraft/core/Direction;)[I
public abstract canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
public abstract canTakeItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Direction;)Z
```
