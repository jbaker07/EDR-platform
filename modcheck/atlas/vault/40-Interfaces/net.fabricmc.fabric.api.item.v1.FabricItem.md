---
type: "interface"
fqcn: "net.fabricmc.fabric.api.item.v1.FabricItem"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.item.v1.FabricItem

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public boolean allowComponentsUpdateAnimation(net.minecraft.world.entity.player.Player, net.minecraft.world.InteractionHand, net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack)
public boolean allowContinuingBlockBreaking(net.minecraft.world.entity.player.Player, net.minecraft.world.item.ItemStack, net.minecraft.world.item.ItemStack)
public net.minecraft.world.item.ItemStackTemplate getCraftingRemainder(net.minecraft.world.item.ItemStack)
public boolean canBeEnchantedWith(net.minecraft.world.item.ItemStack, net.minecraft.core.Holder, net.fabricmc.fabric.api.item.v1.EnchantingContext)
public java.lang.String getCreatorNamespace(net.minecraft.world.item.ItemStack)
```
