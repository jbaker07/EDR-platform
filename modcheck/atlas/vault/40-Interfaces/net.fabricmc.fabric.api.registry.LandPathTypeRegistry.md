---
type: "interface"
fqcn: "net.fabricmc.fabric.api.registry.LandPathTypeRegistry"
module: "fabric-content-registries-v0"
sha256: "e83273ce3a8d06e08c00f31bdc38497d653f678af3693a2fc3ba094537fa8879"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.registry.LandPathTypeRegistry

Module: [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] -- kind: class

```java
public static void register(net.minecraft.world.level.block.Block, net.minecraft.world.level.pathfinder.PathType, net.minecraft.world.level.pathfinder.PathType)
public static void register(net.minecraft.world.level.block.Block, net.fabricmc.fabric.api.registry.LandPathTypeRegistry$StaticPathTypeProvider)
public static void registerDynamic(net.minecraft.world.level.block.Block, net.fabricmc.fabric.api.registry.LandPathTypeRegistry$DynamicPathTypeProvider)
public static net.minecraft.world.level.pathfinder.PathType getPathType(net.minecraft.world.level.block.state.BlockState, net.minecraft.world.level.BlockGetter, net.minecraft.core.BlockPos, boolean)
public static net.fabricmc.fabric.api.registry.LandPathTypeRegistry$PathTypeProvider getPathTypeProvider(net.minecraft.world.level.block.Block)
static {}
```
