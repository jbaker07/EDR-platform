---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.item.v1.ItemTooltipCallback> EVENT
public abstract void getTooltip(net.minecraft.world.item.ItemStack, net.minecraft.world.item.Item$TooltipContext, net.minecraft.world.item.TooltipFlag, java.util.List<net.minecraft.network.chat.Component>)
static {}
```
