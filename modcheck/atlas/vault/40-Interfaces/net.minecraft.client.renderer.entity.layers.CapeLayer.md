---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.CapeLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.CapeLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `net/minecraft/client/renderer/entity/layers/RenderLayer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `submit` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (2 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final model : Lnet/minecraft/client/model/HumanoidModel;
private final equipmentAssets : Lnet/minecraft/client/resources/model/EquipmentAssetManager;
public <init>(Lnet/minecraft/client/renderer/entity/RenderLayerParent;Lnet/minecraft/client/model/geom/EntityModelSet;Lnet/minecraft/client/resources/model/EquipmentAssetManager;)V
private hasLayer(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/client/resources/model/EquipmentClientInfo$LayerType;)Z
public submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V
public synthetic submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/EntityRenderState;FF)V
```
