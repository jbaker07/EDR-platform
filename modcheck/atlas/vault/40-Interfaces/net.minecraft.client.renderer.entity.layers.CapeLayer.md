---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.CapeLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.CapeLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;ILnet/minecraft/client/renderer/entity/state/AvatarRenderState;FF)V` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.entity.layers.CapeLayer extends net.minecraft.client.renderer.entity.layers.RenderLayer<net.minecraft.client.renderer.entity.state.AvatarRenderState, net.minecraft.client.model.player.PlayerModel> {
    private final net.minecraft.client.model.HumanoidModel<net.minecraft.client.renderer.entity.state.AvatarRenderState> model;
    private final net.minecraft.client.resources.model.EquipmentAssetManager equipmentAssets;
    public net.minecraft.client.renderer.entity.layers.CapeLayer(net.minecraft.client.renderer.entity.RenderLayerParent<net.minecraft.client.renderer.entity.state.AvatarRenderState, net.minecraft.client.model.player.PlayerModel>, net.minecraft.client.model.geom.EntityModelSet, net.minecraft.client.resources.model.EquipmentAssetManager);
    private boolean hasLayer(net.minecraft.world.item.ItemStack, net.minecraft.client.resources.model.EquipmentClientInfo$LayerType);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, net.minecraft.client.renderer.entity.state.AvatarRenderState, float, float);
    public void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, net.minecraft.client.renderer.entity.state.EntityRenderState, float, float);
}
```
