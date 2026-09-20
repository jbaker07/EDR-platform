---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `renderArmorPiece` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/HumanoidRenderState;FF)V` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer<S extends net.minecraft.client.renderer.entity.state.HumanoidRenderState, M extends net.minecraft.client.model.HumanoidModel<S>, A extends net.minecraft.client.model.HumanoidModel<S>> extends net.minecraft.client.renderer.entity.layers.RenderLayer<S, M> {
    private final net.minecraft.client.renderer.entity.ArmorModelSet<A> modelSet;
    private final net.minecraft.client.renderer.entity.ArmorModelSet<A> babyModelSet;
    private final net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer equipmentRenderer;
    public net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer(net.minecraft.client.renderer.entity.RenderLayerParent<S, M>, net.minecraft.client.renderer.entity.ArmorModelSet<A>, net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer);
    public net.minecraft.client.renderer.entity.layers.HumanoidArmorLayer(net.minecraft.client.renderer.entity.RenderLayerParent<S, M>, net.minecraft.client.renderer.entity.ArmorModelSet<A>, net.minecraft.client.renderer.entity.ArmorModelSet<A>, net.minecraft.client.renderer.entity.layers.EquipmentLayerRenderer);
    public static boolean shouldRender(net.minecraft.world.item.ItemStack, net.minecraft.world.entity.EquipmentSlot);
    private static boolean shouldRender(net.minecraft.world.item.equipment.Equippable, net.minecraft.world.entity.EquipmentSlot);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, S, float, float);
    private void renderArmorPiece(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, net.minecraft.world.item.ItemStack, net.minecraft.world.entity.EquipmentSlot, int, S);
    private A getArmorModel(S, net.minecraft.world.entity.EquipmentSlot);
    private boolean usesInnerModel(net.minecraft.world.entity.EquipmentSlot);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, net.minecraft.client.renderer.entity.state.EntityRenderState, float, float);
}
```
