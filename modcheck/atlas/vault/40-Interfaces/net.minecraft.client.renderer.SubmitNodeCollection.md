---
type: "interface"
fqcn: "net.minecraft.client.renderer.SubmitNodeCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.SubmitNodeCollection

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `submitMovingBlock` | `@Redirect at INVOKE Lnet/minecraft/client/renderer/block/dispatch/BlockStateMode` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (40, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.SubmitNodeCollection implements net.minecraft.client.renderer.OrderedSubmitNodeCollector {
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase solid;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase waterMask;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase outline;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase alwaysOnTopGizmos;
    public final net.minecraft.client.renderer.feature.phase.TranslucentFeatureRenderPhase seeThrough;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase oitTranslucent;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase shadows;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase nameTags;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase texts;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase shapeOutlines;
    public final net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<? super net.minecraft.client.renderer.feature.submit.TranslucentSubmit> translucentBlocksAndItems;
    public final net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<? super net.minecraft.client.renderer.feature.submit.TranslucentSubmit> translucentModels;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase translucentCustomGeometry;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase breakingOverlay;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase afterTerrain;
    public final net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase translucentGizmos;
    private final java.util.List<net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<?>> allPhases;
    public net.minecraft.client.renderer.SubmitNodeCollection(boolean, net.minecraft.client.renderer.feature.phase.TranslucentFeatureRenderPhase);
    public void submitShadow(com.mojang.blaze3d.vertex.PoseStack, float, java.util.List<net.minecraft.client.renderer.entity.state.EntityRenderState$ShadowPiece>);
    public void submitNameTag(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.phys.Vec3, int, net.minecraft.network.chat.Component, boolean, int, net.minecraft.client.renderer.state.level.CameraRenderState);
    private static net.minecraft.client.renderer.feature.TextFeatureRenderer$Submit nameTag(org.joml.Matrix4f, float, float, net.minecraft.util.FormattedCharSequence, int, int, int, net.minecraft.client.gui.Font$DisplayMode);
    private void submitNameTagPart(net.minecraft.client.renderer.feature.TextFeatureRenderer$Submit);
    private static boolean canRenderAsSolid(net.minecraft.client.renderer.feature.TextFeatureRenderer$Content);
    public void submitText(com.mojang.blaze3d.vertex.PoseStack, float, float, net.minecraft.util.FormattedCharSequence, boolean, net.minecraft.client.gui.Font$DisplayMode, int, int, int, int);
    public void submitTextBackground(com.mojang.blaze3d.vertex.PoseStack, float, float, float, float, int, net.minecraft.client.gui.Font$DisplayMode, int);
    private void submitTextPart(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.gui.Font$DisplayMode, int, net.minecraft.client.renderer.feature.TextFeatureRenderer$Content);
    public void submitFlame(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.entity.state.EntityRenderState, org.joml.Quaternionf);
    public void submitLeash(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.entity.state.EntityRenderState$LeashState);
    public <S> void submitModel(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.texture.UvMapping, int);
    public <S> void submitCrumblingOverlay(net.minecraft.client.model.Model<? super S>, S, com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, int, int, int, net.minecraft.client.renderer.feature.ModelFeatureRenderer$CrumblingOverlay);
    public void submitMovingBlock(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.block.MovingBlockRenderState, int);
    public void submitBlockModel(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, int[], int, int, int);
    private static net.minecraft.client.renderer.rendertype.RenderType getOutlineRenderType(net.minecraft.client.renderer.rendertype.RenderType);
    public void submitBreakingBlockModel(com.mojang.blaze3d.vertex.PoseStack, java.util.List<net.minecraft.client.renderer.block.dispatch.BlockStateModelPart>, int, boolean);
    public void submitShapeOutline(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.phys.shapes.VoxelShape, net.minecraft.client.renderer.rendertype.RenderType, int, float, boolean);
    public void submitItem(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.world.item.ItemDisplayContext, int, int, int, int[], net.minecraft.client.resources.model.geometry.ItemQuads, net.minecraft.client.renderer.item.ItemStackRenderState$FoilType);
    public void submitCustomGeometry(com.mojang.blaze3d.vertex.PoseStack, net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.client.renderer.SubmitNodeCollector$CustomGeometryRenderer);
    public void submitQuadParticleGroup(net.minecraft.client.renderer.state.level.QuadParticleRenderState);
    public void submitGizmoPrimitives(net.minecraft.client.renderer.gizmos.DrawableGizmoPrimitives$Group, net.minecraft.client.renderer.state.level.CameraRenderState, boolean);
    public java.util.List<net.minecraft.client.renderer.feature.phase.FeatureRenderPhase<?>> allPhases();
}
```
