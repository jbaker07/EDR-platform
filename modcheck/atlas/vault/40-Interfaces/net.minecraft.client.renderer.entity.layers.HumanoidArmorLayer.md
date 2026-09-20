---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `net/minecraft/client/renderer/entity/layers/RenderLayer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `renderArmorPiece` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submit` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final modelSet : Lnet/minecraft/client/renderer/entity/ArmorModelSet;
private final babyModelSet : Lnet/minecraft/client/renderer/entity/ArmorModelSet;
private final equipmentRenderer : Lnet/minecraft/client/renderer/entity/layers/EquipmentLayerRenderer;
public <init>(Lnet/minecraft/client/renderer/entity/RenderLayerParent;Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/renderer/entity/layers/EquipmentLayerRenderer;)V
public <init>(Lnet/minecraft/client/renderer/entity/RenderLayerParent;Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/renderer/entity/layers/EquipmentLayerRenderer;)V
public static shouldRender(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;)Z
private static shouldRender(Lnet/minecraft/world/item/equipment/Equippable;Lnet/minecraft/world/entity/EquipmentSlot;)Z
public submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V
private renderArmorPiece(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/EquipmentSlot;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;)V
private getArmorModel(Lnet/minecraft/client/renderer/entity/state/HumanoidRenderState;Lnet/minecraft/world/entity/EquipmentSlot;)Lnet/minecraft/client/model/HumanoidModel;
private usesInnerModel(Lnet/minecraft/world/entity/EquipmentSlot;)Z
public synthetic submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/EntityRenderState;FF)V
```
