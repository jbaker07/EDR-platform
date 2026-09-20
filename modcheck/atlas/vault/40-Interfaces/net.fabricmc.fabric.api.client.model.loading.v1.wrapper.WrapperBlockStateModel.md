---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel"
module: "fabric-model-loading-api-v1"
sha256: "4889e5947bb2f9d899f72c17676b27aaef7d2594ecaaf75e89626e1eea169711"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel

Module: [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] -- kind: abstract_class

```java
protected net.minecraft.client.renderer.block.dispatch.BlockStateModel wrapped
protected net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel()
protected net.fabricmc.fabric.api.client.model.loading.v1.wrapper.WrapperBlockStateModel(net.minecraft.client.renderer.block.dispatch.BlockStateModel)
public void collectParts(net.minecraft.util.RandomSource, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>)
public net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial()
public int materialFlags()
public boolean hasMaterialFlag(int)
public void emitQuads(net.fabricmc.fabric.api.client.renderer.v1.mesh.QuadEmitter, net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource, java.util.function.Predicate<net.minecraft.core.Direction>)
public java.lang.Object createGeometryKey(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource)
public net.minecraft.client.resources.model.sprite.Material$Baked particleMaterial(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState)
public int materialFlags(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource)
public boolean hasMaterialFlag(net.minecraft.client.renderer.block.BlockAndTintGetter, net.minecraft.core.BlockPos, net.minecraft.world.level.block.state.BlockState, net.minecraft.util.RandomSource, int)
```
