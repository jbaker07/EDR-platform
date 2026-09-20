---
type: "interface"
fqcn: "net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup"
module: "fabric-api-lookup-api-v1"
sha256: "4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup

Module: [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] -- kind: interface

```java
public static <A, C> net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup<A, C> get(net.minecraft.resources.Identifier, java.lang.Class<A>, java.lang.Class<C>)
public abstract A find(net.minecraft.world.item.ItemStack, C)
public abstract void registerSelf(net.minecraft.world.level.ItemLike...)
public abstract void registerForItems(net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup$ItemApiProvider<A, C>, net.minecraft.world.level.ItemLike...)
public abstract void registerFallback(net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup$ItemApiProvider<A, C>)
public abstract net.minecraft.resources.Identifier getId()
public abstract java.lang.Class<A> apiClass()
public abstract java.lang.Class<C> contextClass()
public abstract net.fabricmc.fabric.api.lookup.v1.item.ItemApiLookup$ItemApiProvider<A, C> getProvider(net.minecraft.world.item.Item)
```
