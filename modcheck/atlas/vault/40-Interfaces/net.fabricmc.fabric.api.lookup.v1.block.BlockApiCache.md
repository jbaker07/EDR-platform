---
type: "interface"
fqcn: "net.fabricmc.fabric.api.lookup.v1.block.BlockApiCache"
module: "fabric-api-lookup-api-v1"
sha256: "4ff3be674760c602b4ed59c10d74d2d52597e8a562489ecd4b68ebf7f71d466c"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.lookup.v1.block.BlockApiCache

Module: [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] -- kind: interface

```java
public default A find(C)
public abstract A find(net.minecraft.world.level.block.state.BlockState, C)
public abstract net.minecraft.world.level.block.entity.BlockEntity getBlockEntity()
public abstract net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup<A, C> getLookup()
public abstract net.minecraft.server.level.ServerLevel getLevel()
public abstract net.minecraft.core.BlockPos getPos()
public static <A, C> net.fabricmc.fabric.api.lookup.v1.block.BlockApiCache<A, C> create(net.fabricmc.fabric.api.lookup.v1.block.BlockApiLookup<A, C>, net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos)
```
