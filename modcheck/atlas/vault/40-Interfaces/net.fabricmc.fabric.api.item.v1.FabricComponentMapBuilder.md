---
type: "interface"
fqcn: "net.fabricmc.fabric.api.item.v1.FabricComponentMapBuilder"
module: "fabric-item-api-v1"
sha256: "aff8cffc3d6da5475f21060e45cd28307676974674c4e10504c0fcc89c7746c4"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.item.v1.FabricComponentMapBuilder

Module: [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] -- kind: interface

```java
public default <T> T get(net.minecraft.core.component.DataComponentType<T>)
public default <T> T getOrCreate(net.minecraft.core.component.DataComponentType<T>, java.util.function.Supplier<T>)
public default <T> T getOrDefault(net.minecraft.core.component.DataComponentType<T>, T)
public default <T> java.util.List<T> getOrEmpty(net.minecraft.core.component.DataComponentType<java.util.List<T>>)
public default boolean contains(net.minecraft.core.component.DataComponentType<?>)
```
