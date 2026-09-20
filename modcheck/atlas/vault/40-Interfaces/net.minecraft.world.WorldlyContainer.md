---
type: "interface"
fqcn: "net.minecraft.world.WorldlyContainer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.WorldlyContainer

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `canPlaceItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Di` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `canTakeItemThroughFace(ILnet/minecraft/world/item/ItemStack;Lnet/minecraft/core/Di` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getSlotsForFace(Lnet/minecraft/core/Direction;)[I` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.WorldlyContainer extends net.minecraft.world.Container {
    public abstract int[] getSlotsForFace(net.minecraft.core.Direction);
    public abstract boolean canPlaceItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
    public abstract boolean canTakeItemThroughFace(int, net.minecraft.world.item.ItemStack, net.minecraft.core.Direction);
}
```
