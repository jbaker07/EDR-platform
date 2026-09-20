---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemModel$Unbaked"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemModel$Unbaked

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/resources/model/ResolvableModel`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bake` | `(Lnet/minecraft/client/renderer/item/ItemModel$BakingContext;Lorg/joml` | exact | invokeinterface@6 in `WrapperUnbakedItemModel.bake` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `resolveDependencies` | `(Lnet/minecraft/client/resources/model/ResolvableModel$Resolver;)V` | inherited_exact | invokeinterface@5 in `WrapperUnbakedItemModel.resolveDependencies` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `type` | `()Lcom/mojang/serialization/MapCodec;` | exact | invokeinterface@4 in `WrapperUnbakedItemModel.type` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract type()Lcom/mojang/serialization/MapCodec;
public abstract bake(Lnet/minecraft/client/renderer/item/ItemModel$BakingContext;Lorg/joml/Matrix4fc;)Lnet/minecraft/client/renderer/item/ItemModel;
```
