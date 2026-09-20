---
type: "interface"
fqcn: "net.fabricmc.fabric.api.item.v1.FabricItem$Properties"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.item.v1.FabricItem$Properties

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public default <T> net.minecraft.world.item.Item$Properties modifyComponent(net.minecraft.core.component.DataComponentType<T>, org.apache.commons.lang3.function.TriFunction<T, net.minecraft.core.HolderLookup$Provider, net.minecraft.resources.ResourceKey<net.minecraft.world.item.Item>, T>)
public default net.minecraft.world.item.Item$Properties modifyComponents(net.minecraft.core.component.DataComponentInitializers$Initializer<net.minecraft.world.item.Item>)
public default net.minecraft.world.item.Item$Properties equipmentSlot(net.fabricmc.fabric.api.item.v1.EquipmentSlotProvider)
public default net.minecraft.world.item.Item$Properties customDamage(net.fabricmc.fabric.api.item.v1.CustomDamageHandler)
public default net.minecraft.world.item.Item$Properties modelId(net.minecraft.resources.Identifier)
public default net.minecraft.resources.ResourceKey<net.minecraft.world.item.Item> itemId()
```
