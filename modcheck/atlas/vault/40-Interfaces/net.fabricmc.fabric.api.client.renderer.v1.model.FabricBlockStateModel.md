---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModel"
module: "fabric-renderer-api-v1"
sha256: "2e4aaeb20f8615e8176f30bc82616cee25f8727323bfef7a70d5a6555ae8e9ee"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.renderer.v1.model.FabricBlockStateModel

Module: [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] -- kind: interface

```java
public void emitQuads(net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource, java.util.function.Predicate)
public java.lang.Object createGeometryKey(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource)
public net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState)
public int materialFlags(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource)
public boolean hasMaterialFlag(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource, int)
```
