---
type: "interface"
fqcn: "net.minecraft.client.renderer.SubmitNodeCollection"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.SubmitNodeCollection

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/OrderedSubmitNodeCollector`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getOutlineRenderType` | `(Lnet/minecraft/client/renderer/rendertype/RenderType;)Lnet/minecraft/` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `afterTerrain` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$12` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `alwaysOnTopGizmos` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$13` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `breakingOverlay` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `breakingOverlay` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$10` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `nameTags` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$2` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `outline` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `outline` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$14` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `seeThrough` | `Lnet/minecraft/client/renderer/feature/phase/TranslucentFeatureRenderP` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$3` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `shadows` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `shapeOutlines` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$5` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `solid` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `solid` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$0` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `texts` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$4` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `translucentBlocksAndItems` | `Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `translucentBlocksAndItems` | `Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$6` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `translucentCustomGeometry` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$8` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `translucentGizmos` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$9` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `translucentModels` | `Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$7` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `waterMask` | `Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;` | exact | getfield@1 in `SubmitRenderPhases.lambda$static$11` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `submitMovingBlock` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/b` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (17 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public final solid : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final waterMask : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final outline : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final alwaysOnTopGizmos : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final seeThrough : Lnet/minecraft/client/renderer/feature/phase/TranslucentFeatureRenderPhase;
public final oitTranslucent : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final shadows : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final nameTags : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final texts : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final shapeOutlines : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final translucentBlocksAndItems : Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;
public final translucentModels : Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;
public final translucentCustomGeometry : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final breakingOverlay : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final afterTerrain : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
public final translucentGizmos : Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase;
private final allPhases : Ljava/util/List;
public <init>(ZLnet/minecraft/client/renderer/feature/phase/TranslucentFeatureRenderPhase;)V
public submitShadow(Lcom/mojang/blaze3d/vertex/PoseStack;FLjava/util/List;)V
public submitNameTag(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/Vec3;ILnet/minecraft/network/chat/Component;ZILnet/minecraft/client/renderer/state/level/CameraRenderState;)V
private static nameTag(Lorg/joml/Matrix4f;FFLnet/minecraft/util/FormattedCharSequence;IIILnet/minecraft/client/gui/Font$DisplayMode;)Lnet/minecraft/client/renderer/feature/TextFeatureRenderer$Submit;
private submitNameTagPart(Lnet/minecraft/client/renderer/feature/TextFeatureRenderer$Submit;)V
private static canRenderAsSolid(Lnet/minecraft/client/renderer/feature/TextFeatureRenderer$Content;)Z
public submitText(Lcom/mojang/blaze3d/vertex/PoseStack;FFLnet/minecraft/util/FormattedCharSequence;ZLnet/minecraft/client/gui/Font$DisplayMode;IIII)V
public submitTextBackground(Lcom/mojang/blaze3d/vertex/PoseStack;FFFFILnet/minecraft/client/gui/Font$DisplayMode;I)V
private submitTextPart(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/gui/Font$DisplayMode;ILnet/minecraft/client/renderer/feature/TextFeatureRenderer$Content;)V
public submitFlame(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lorg/joml/Quaternionf;)V
public submitLeash(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/entity/state/EntityRenderState$LeashState;)V
public submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/texture/UvMapping;I)V
public submitCrumblingOverlay(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V
public submitMovingBlock(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/block/MovingBlockRenderState;I)V
public submitBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;Ljava/util/List;[IIII)V
private static getOutlineRenderType(Lnet/minecraft/client/renderer/rendertype/RenderType;)Lnet/minecraft/client/renderer/rendertype/RenderType;
public submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;IZ)V
public submitShapeOutline(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/client/renderer/rendertype/RenderType;IFZ)V
public submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDisplayContext;III[ILnet/minecraft/client/resources/model/geometry/ItemQuads;Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;)V
public submitCustomGeometry(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/client/renderer/SubmitNodeCollector$CustomGeometryRenderer;)V
public submitQuadParticleGroup(Lnet/minecraft/client/renderer/state/level/QuadParticleRenderState;)V
public submitGizmoPrimitives(Lnet/minecraft/client/renderer/gizmos/DrawableGizmoPrimitives$Group;Lnet/minecraft/client/renderer/state/level/CameraRenderState;Z)V
public allPhases()Ljava/util/List;
```
