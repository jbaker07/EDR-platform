---
type: "interface"
fqcn: "net.fabricmc.fabric.api.util.Block2ObjectMap"
module: "fabric-content-registries-v0"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.util.Block2ObjectMap

Module: [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] -- kind: interface

```java
public abstract java.lang.Object get(net.minecraft.world.level.block.Block)
public abstract void add(net.minecraft.world.level.block.Block, java.lang.Object)
public abstract void add(net.minecraft.tags.TagKey, java.lang.Object)
public abstract void remove(net.minecraft.world.level.block.Block)
public abstract void remove(net.minecraft.tags.TagKey)
public abstract void clear(net.minecraft.world.level.block.Block)
public abstract void clear(net.minecraft.tags.TagKey)
```
