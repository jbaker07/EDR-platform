---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelBaker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelBaker

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getModel` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/resources/` | exact | invokeinterface@9 in `SimpleUnbakedExtraModel.bake` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materials` | `()Lnet/minecraft/client/resources/model/sprite/MaterialBaker;` | exact | invokeinterface@34 in `SimpleUnbakedExtraModel.lambda$bakeResolved$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materials` | `()Lnet/minecraft/client/resources/model/sprite/MaterialBaker;` | exact | invokeinterface@38 in `SimpleModelWrapperMixin.lambda$analyzeMesh$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `missingBlockModelPart` | `()Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;` | exact | invokeinterface@209 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getModel(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/resources/model/ResolvedModel;
public abstract missingBlockModelPart()Lnet/minecraft/client/renderer/block/dispatch/BlockStateModelPart;
public abstract materials()Lnet/minecraft/client/resources/model/sprite/MaterialBaker;
public abstract interner()Lnet/minecraft/client/resources/model/ModelBaker$Interner;
public abstract compute(Lnet/minecraft/client/resources/model/ModelBaker$SharedOperationKey;)Ljava/lang/Object;
```
