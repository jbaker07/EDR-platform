---
type: "interface"
fqcn: "net.fabricmc.fabric.api.util.Item2ObjectMap"
module: "fabric-content-registries-v0"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.util.Item2ObjectMap

Module: [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] -- kind: interface

```java
public abstract V get(net.minecraft.world.level.ItemLike)
public abstract void add(net.minecraft.world.level.ItemLike, V)
public abstract void add(net.minecraft.tags.TagKey<net.minecraft.world.item.Item>, V)
public abstract void remove(net.minecraft.world.level.ItemLike)
public abstract void remove(net.minecraft.tags.TagKey<net.minecraft.world.item.Item>)
public abstract void clear(net.minecraft.world.level.ItemLike)
public abstract void clear(net.minecraft.tags.TagKey<net.minecraft.world.item.Item>)
```
