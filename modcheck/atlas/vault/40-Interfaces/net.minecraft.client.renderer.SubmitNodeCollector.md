---
type: "interface"
fqcn: "net.minecraft.client.renderer.SubmitNodeCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.SubmitNodeCollector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/renderer/OrderedSubmitNodeCollector`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `submitBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Function;ZLj` | inherited_exact | invokeinterface@149 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Function;ZLj` | inherited_exact | invokeinterface@185 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBreakingBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;Lnet/fabricmc/fa` | inherited_exact | invokeinterface@67 in `LevelRendererMixin.submitBreakingBlockModelProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitItem` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDi` | inherited_exact | invokeinterface@38 in `ItemStackRenderStateLayerRenderStateMixin.submitItemProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitItem` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDi` | inherited_exact | invokeinterface@61 in `ItemStackRenderStateLayerRenderStateMixin.submitItemProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract order(I)Lnet/minecraft/client/renderer/OrderedSubmitNodeCollector;
```
