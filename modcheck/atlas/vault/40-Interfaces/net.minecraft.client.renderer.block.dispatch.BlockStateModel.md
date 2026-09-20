---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModel

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModel`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `collectParts` | `(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` | exact | invokeinterface@6 in `WrapperBlockStateModel.collectParts` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `collectParts` | `(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` | exact | invokeinterface@46 in `CompositeBlockStateModelImpl.collectParts` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `collectParts` | `(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V` | exact | invokeinterface@62 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@9 in `WrapperBlockStateModel.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@42 in `CompositeBlockStateModelImpl.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@103 in `CompositeBlockStateModelImpl.createGeometryKey` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@74 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@129 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `createGeometryKey` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@17 in `WeightedVariantsMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@13 in `WrapperBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@57 in `CompositeBlockStateModelImpl.emitQuads` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@84 in `MultiPartModelMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@21 in `WeightedVariantsMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@57 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@48 in `LevelRendererMixin.submitBreakingBlockModelProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Lnet/min` | inherited_exact | invokeinterface@164 in `AltModelBlockRendererImpl.tesselateBlock` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `hasMaterialFlag` | `(I)Z` | exact | invokeinterface@5 in `WrapperBlockStateModel.hasMaterialFlag` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `hasMaterialFlag` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@11 in `WrapperBlockStateModel.hasMaterialFlag` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `hasMaterialFlag` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@23 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasMaterialFlag` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@28 in `LevelExtractorMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasMaterialFlag` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@35 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `materialFlags` | `()I` | exact | invokeinterface@4 in `WrapperBlockStateModel.materialFlags` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materialFlags` | `()I` | exact | invokeinterface@47 in `CompositeBlockStateModelImpl.<init>` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materialFlags` | `()I` | exact | invokeinterface@4 in `FabricBlockStateModel.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `materialFlags` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@9 in `WrapperBlockStateModel.materialFlags` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materialFlags` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@85 in `MultiPartModelMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `materialFlags` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@17 in `WeightedVariantsMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial` | `()Lnet/minecraft/client/resources/model/sprite/Material$Baked;` | exact | invokeinterface@4 in `WrapperBlockStateModel.particleMaterial` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial` | `()Lnet/minecraft/client/resources/model/sprite/Material$Baked;` | exact | invokeinterface@6 in `CompositeBlockStateModelImpl.particleMaterial` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial` | `()Lnet/minecraft/client/resources/model/sprite/Material$Baked;` | exact | invokeinterface@4 in `FabricBlockStateModel.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@7 in `WrapperBlockStateModel.particleMaterial` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@9 in `CompositeBlockStateModelImpl.particleMaterial` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `particleMaterial` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@11 in `FabricBlockStateModelSet.getParticleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@29 in `MultiPartModelMixin.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `particleMaterial` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | inherited_exact | invokeinterface@24 in `WeightedVariantsMixin.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract collectParts(Lnet/minecraft/util/RandomSource;Ljava/util/List;)V
public abstract particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public abstract materialFlags()I
public hasMaterialFlag(I)Z
```
