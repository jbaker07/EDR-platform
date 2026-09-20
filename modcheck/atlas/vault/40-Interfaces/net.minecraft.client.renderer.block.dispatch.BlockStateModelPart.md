---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.BlockStateModelPart"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.BlockStateModelPart

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/model/FabricBlockStateModelPart`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Ljava/ut` | inherited_exact | invokeinterface@101 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Ljava/ut` | inherited_exact | invokespecial@73 in `SimpleModelWrapperMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `emitQuads` | `(Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;Ljava/ut` | inherited_exact | invokeinterface@52 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getQuads` | `(Lnet/minecraft/core/Direction;)Ljava/util/List;` | exact | invokeinterface@53 in `VanillaBlockModelPartEncoder.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getQuads` | `(Lnet/minecraft/core/Direction;)Ljava/util/List;` | exact | invokeinterface@30 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getQuads` | `(Lnet/minecraft/core/Direction;)Ljava/util/List;` | exact | invokeinterface@111 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `useAmbientOcclusion` | `()Z` | exact | invokeinterface@1 in `VanillaBlockModelPartEncoder.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getQuads(Lnet/minecraft/core/Direction;)Ljava/util/List;
public abstract useAmbientOcclusion()Z
public abstract particleMaterial()Lnet/minecraft/client/resources/model/sprite/Material$Baked;
public abstract materialFlags()I
```
