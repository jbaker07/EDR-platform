---
type: "interface"
fqcn: "net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup"
module: "fabric-api-lookup-api-v1"
sha256: "4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup

Module: [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup get(net.minecraft.resources.Identifier, java.lang.Class, java.lang.Class)
public java.lang.Object find(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, java.lang.Object)
public abstract java.lang.Object find(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BlockEntity, java.lang.Object)
public abstract void registerSelf(net.minecraft.world.level.block.entity.BlockEntityType[])
public abstract void registerForBlocks(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider, net.minecraft.world.level.block.Block[])
public void registerForBlockEntity(java.util.function.BiFunction, net.minecraft.world.level.block.entity.BlockEntityType)
public abstract void registerForBlockEntities(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockEntityApiProvider, net.minecraft.world.level.block.entity.BlockEntityType[])
public abstract void registerFallback(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider)
public abstract net.minecraft.resources.Identifier getId()
public abstract java.lang.Class apiClass()
public abstract java.lang.Class contextClass()
public abstract net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider getProvider(net.minecraft.world.level.block.Block)
```
