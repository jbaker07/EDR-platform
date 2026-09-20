---
type: "interface"
fqcn: "net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder"
module: "fabric-object-builder-api-v1"
sha256: "3a5f0ccef440552828d9469420547dc3cdbe3e206af73fa98d0bfcfb5b75c1ba"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder

Module: [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- kind: class

```java
public static <T extends net.minecraft.world.level.block.entity.BlockEntity> net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder<T> create(net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder$Factory<? extends T>, net.minecraft.world.level.block.Block...)
public net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder<T> addBlock(net.minecraft.world.level.block.Block)
public net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder<T> addBlocks(net.minecraft.world.level.block.Block...)
public net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder<T> addBlocks(java.util.Collection<? extends net.minecraft.world.level.block.Block>)
public net.fabricmc.fabric.api.object.builder.v1.block.entity.FabricBlockEntityTypeBuilder<T> canPotentiallyExecuteCommands(boolean)
public net.minecraft.world.level.block.entity.BlockEntityType<T> build()
public net.minecraft.world.level.block.entity.BlockEntityType<T> build(com.mojang.datafixers.types.Type<?>)
```
