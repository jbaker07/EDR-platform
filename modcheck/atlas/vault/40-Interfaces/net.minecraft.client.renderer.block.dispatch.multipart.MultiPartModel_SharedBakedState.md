---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$SharedBakedState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` final; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `selectModels` | `(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;` | exact | invokevirtual@16 in `MultiPartModelMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `selectModels` | `(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;` | exact | invokevirtual@16 in `MultiPartModelMixin.createGeometryKey` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `selectModels` | `(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;` | exact | invokevirtual@16 in `MultiPartModelMixin.materialFlags` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final selectors : Ljava/util/List;
private final particleMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private final materialFlags : I
private final subsets : Ljava/util/Map;
private static getFirstModel(Ljava/util/List;)Lnet/minecraft/client/renderer/block/dispatch/BlockStateModel;
private static computeMaterialFlags(Ljava/util/List;)I
public <init>(Ljava/util/List;)V
public selectModels(Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/List;
private synthetic lambda$selectModels$0(Ljava/util/BitSet;)Ljava/util/List;
```
