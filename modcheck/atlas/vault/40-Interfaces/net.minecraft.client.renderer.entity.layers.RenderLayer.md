---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.layers.RenderLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.layers.RenderLayer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/renderer/entity/RenderLayerParent;)V` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.renderer.entity.layers.RenderLayer<S extends net.minecraft.client.renderer.entity.state.EntityRenderState, M extends net.minecraft.client.model.EntityModel<? super S>> {
    private final net.minecraft.client.renderer.entity.RenderLayerParent<S, M> renderer;
    public net.minecraft.client.renderer.entity.layers.RenderLayer(net.minecraft.client.renderer.entity.RenderLayerParent<S, M>);
    protected static <S extends net.minecraft.client.renderer.entity.state.LivingEntityRenderState> void coloredCutoutModelCopyLayerRender(net.minecraft.client.model.Model<? super S>, net.minecraft.resources.Identifier, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, S, int, int);
    protected static <S extends net.minecraft.client.renderer.entity.state.LivingEntityRenderState> void renderColoredCutoutModel(net.minecraft.client.model.Model<? super S>, net.minecraft.resources.Identifier, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, S, int, int);
    public M getParentModel();
    public abstract void submit(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.SubmitNodeCollector, int, S, float, float);
}
```
