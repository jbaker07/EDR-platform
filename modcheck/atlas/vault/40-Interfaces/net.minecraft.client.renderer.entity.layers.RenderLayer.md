---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.RenderLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.RenderLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/renderer/entity/RenderLayerParent;)V` | exact | invokespecial@2 in `HumanoidArmorLayerMixin.<init>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final renderer : Lnet/minecraft/client/renderer/entity/RenderLayerParent;
public <init>(Lnet/minecraft/client/renderer/entity/RenderLayerParent;)V
protected static coloredCutoutModelCopyLayerRender(Lnet/minecraft/client/model/Model;Lnet/minecraft/resources/Identifier;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;II)V
protected static renderColoredCutoutModel(Lnet/minecraft/client/model/Model;Lnet/minecraft/resources/Identifier;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;II)V
public getParentModel()Lnet/minecraft/client/model/EntityModel;
public abstract submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/EntityRenderState;FF)V
```
