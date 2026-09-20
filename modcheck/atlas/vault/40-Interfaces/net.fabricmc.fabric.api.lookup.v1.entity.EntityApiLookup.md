---
type: "interface"
fqcn: "net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup"
module: "fabric-api-lookup-api-v1"
sha256: "4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup

Module: [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup get(net.minecraft.resources.Identifier, java.lang.Class, java.lang.Class)
public abstract java.lang.Object find(net.minecraft.world.entity.Entity, java.lang.Object)
public abstract void registerSelf(net.minecraft.world.entity.EntityType[])
public void registerForType(java.util.function.BiFunction, net.minecraft.world.entity.EntityType)
public abstract void registerForTypes(net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup$EntityApiProvider, net.minecraft.world.entity.EntityType[])
public abstract void registerFallback(net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup$EntityApiProvider)
public abstract net.minecraft.resources.Identifier getId()
public abstract java.lang.Class apiClass()
public abstract java.lang.Class contextClass()
public abstract net.fabricmc.fabric.api.lookup.v1.entity.EntityApiLookup$EntityApiProvider getProvider(net.minecraft.world.entity.EntityType)
```
