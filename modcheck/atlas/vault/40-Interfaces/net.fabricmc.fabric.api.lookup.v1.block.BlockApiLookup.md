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
public static <A, C> net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup<A, C> get(net.minecraft.resources.Identifier, java.lang.Class<A>, java.lang.Class<C>)
public default A find(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, C)
public abstract A find(net.minecraft.world.level.Level, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.block.entity.BlockEntity, C)
public abstract void registerSelf(net.minecraft.world.level.block.entity.BlockEntityType<?>...)
public abstract void registerForBlocks(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider<A, C>, net.minecraft.world.level.block.Block...)
public default <T extends net.minecraft.world.level.block.entity.BlockEntity> void registerForBlockEntity(java.util.function.BiFunction<? super T, C, A>, net.minecraft.world.level.block.entity.BlockEntityType<T>)
public abstract void registerForBlockEntities(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockEntityApiProvider<A, C>, net.minecraft.world.level.block.entity.BlockEntityType<?>...)
public abstract void registerFallback(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider<A, C>)
public abstract net.minecraft.resources.Identifier getId()
public abstract java.lang.Class<A> apiClass()
public abstract java.lang.Class<C> contextClass()
public abstract net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup$BlockApiProvider<A, C> getProvider(net.minecraft.world.level.block.Block)
```
