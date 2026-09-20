---
type: "interface"
fqcn: "net.minecraft.client.renderer.OrderedSubmitNodeCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.OrderedSubmitNodeCollector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `submitBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Fu` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;Lnet/f` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitCustom(Lnet/fabricmc/fabric/api/client/rendering/v1/SubmitRenderPh` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/i` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.OrderedSubmitNodeCollector {
    public abstract void submitShadow(com.mojang.blaze3d.vertex.PoseStack, float, java.util.List<net.minecraft.client.renderer.entity.state.EntityRenderState$ShadowPiece>);
    public abstract void submitNameTag(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.phys.Vec3, int, net.minecraft.network.chat.Component, boolean, int, net.minecraft.client.renderer.state.level.CameraRenderState);
    public abstract void submitText(com.mojang.blaze3d.vertex.PoseStack, float, float, net.minecraft.util.FormattedCharSequence, boolean, net.minecraft.client.gui.Font$DisplayMode, int, int, int, int);
    public abstract void submitTextBackground(com.mojang.blaze3d.vertex.PoseStack, float, float, float, float, int, net.minecraft.client.gui.Font$DisplayMode, int);
    public abstract void submitFlame(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.entity.state.EntityRenderState, org.joml.Quaternionf);
    public abstract void submitLeash(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.entity.state.EntityRenderState$LeashState);
    public abstract <S> void submitModel(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.texture.UvMapping, int);
    public default <S> void submitModel(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int);
    public default <S> void submitModel(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.resources.Identifier, int, int, int);
    public default <S> void submitModel(net.minecraft.client.model.Model<S>, S, com.mojang.blaze3d.vertex.PoseStack, int, int, int, net.minecraft.client.resources.model.sprite.SpriteId, net.minecraft.client.resources.model.sprite.SpriteGetter, int);
    public abstract <S> void submitCrumblingOverlay(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.feature.ModelFeatureRenderer$CrumblingOverlay);
    public default void submitCrumblingOverlay(net.minecraft.client.model.geom.ModelPart, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.feature.ModelFeatureRenderer$CrumblingOverlay);
    public default void submitModelPart(net.minecraft.client.model.geom.ModelPart, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, net.minecraft.client.renderer.texture.UvMapping);
    public default void submitModelPart(net.minecraft.client.model.geom.ModelPart, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, net.minecraft.client.renderer.texture.UvMapping, int);
    public default void submitModelPart(net.minecraft.client.model.geom.ModelPart, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, net.minecraft.client.renderer.texture.UvMapping, int, int);
    public abstract void submitMovingBlock(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.block.MovingBlockRenderState, int);
    public abstract void submitBlockModel(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, int[], int, int, int);
    public abstract void submitBreakingBlockModel(com.mojang.blaze3d.vertex.PoseStack, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, int, boolean);
    public abstract void submitShapeOutline(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.phys.shapes.VoxelShape, net.minecraft.client.renderer.rendertype.RenderType, int, float, boolean);
    public abstract void submitItem(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.item.ItemDisplayContext, int, int, int, int[], net.minecraft.client.resources.model.geometry.ItemQuads, net.minecraft.client.renderer.item.ItemStackRenderState$FoilType);
    public abstract void submitCustomGeometry(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.client.renderer.SubmitNodeCollector$CustomGeometryRenderer);
    public abstract void submitQuadParticleGroup(net.minecraft.client.renderer.state.level.QuadParticleRenderState);
    public abstract void submitGizmoPrimitives(net.minecraft.client.renderer.gizmos.DrawableGizmoPrimitives$Group, net.minecraft.client.renderer.state.level.CameraRenderState, boolean);
    private static net.minecraft.client.renderer.rendertype.RenderType lambda$submitModelPart$0(net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.resources.Identifier);
    private static net.minecraft.client.renderer.rendertype.RenderType lambda$submitCrumblingOverlay$0(net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.resources.Identifier);
}
```
