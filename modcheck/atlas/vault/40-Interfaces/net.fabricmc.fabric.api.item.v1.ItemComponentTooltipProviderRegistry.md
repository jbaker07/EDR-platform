---
type: "interface"
fqcn: "net.fabricmc.fabric.api.item.v1.ItemComponentTooltipProviderRegistry"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.item.v1.ItemComponentTooltipProviderRegistry

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public static void addFirst(net.minecraft.core.component.DataComponentType)
public static void addFirst(net.minecraft.core.component.DataComponentType, net.minecraft.world.item.component.TooltipProvider$Getter)
public static void addLast(net.minecraft.core.component.DataComponentType)
public static void addLast(net.minecraft.core.component.DataComponentType, net.minecraft.world.item.component.TooltipProvider$Getter)
public static void addBefore(net.minecraft.core.component.DataComponentType, net.minecraft.core.component.DataComponentType)
public static void addBefore(net.minecraft.core.component.DataComponentType, net.minecraft.core.component.DataComponentType, net.minecraft.world.item.component.TooltipProvider$Getter)
public static void addAfter(net.minecraft.core.component.DataComponentType, net.minecraft.core.component.DataComponentType)
public static void addAfter(net.minecraft.core.component.DataComponentType, net.minecraft.core.component.DataComponentType, net.minecraft.world.item.component.TooltipProvider$Getter)
```
