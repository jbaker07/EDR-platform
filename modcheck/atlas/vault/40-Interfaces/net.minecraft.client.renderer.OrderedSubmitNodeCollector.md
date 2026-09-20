---
type: "interface"
fqcn: "net.minecraft.client.renderer.OrderedSubmitNodeCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.OrderedSubmitNodeCollector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/render/FabricOrderedSubmitNodeCollector`, `net/fabricmc/fabric/api/client/rendering/v1/FabricOrderedSubmitNodeCollector`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `submitBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Function;ZLj` | inherited_exact | invokeinterface@20 in `SubmitNodeStorageMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/r` | exact | invokeinterface@19 in `FabricOrderedSubmitNodeCollector.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBreakingBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;IZ)V` | exact | invokeinterface@10 in `FabricOrderedSubmitNodeCollector.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBreakingBlockModel` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;Lnet/fabricmc/fa` | inherited_exact | invokeinterface@12 in `SubmitNodeStorageMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitCustom` | `(Lnet/fabricmc/fabric/api/client/rendering/v1/SubmitRenderPhase;Lnet/m` | inherited_exact | invokeinterface@7 in `SubmitNodeStorageMixin.submitCustom` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `submitItem` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDi` | inherited_exact | invokeinterface@20 in `SubmitNodeStorageMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitItem` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDi` | exact | invokeinterface@17 in `FabricOrderedSubmitNodeCollector.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitModel` | `(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze` | exact | invokeinterface@24 in `ArmorRenderer.submitTransformCopyingModel` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `submitModel` | `(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze` | exact | invokeinterface@28 in `ArmorRenderer.submitTransformCopyingModel` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (0 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract submitShadow(Lcom/mojang/blaze3d/vertex/PoseStack;FLjava/util/List;)V
public abstract submitNameTag(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/Vec3;ILnet/minecraft/network/chat/Component;ZILnet/minecraft/client/renderer/state/level/CameraRenderState;)V
public abstract submitText(Lcom/mojang/blaze3d/vertex/PoseStack;FFLnet/minecraft/util/FormattedCharSequence;ZLnet/minecraft/client/gui/Font$DisplayMode;IIII)V
public abstract submitTextBackground(Lcom/mojang/blaze3d/vertex/PoseStack;FFFFILnet/minecraft/client/gui/Font$DisplayMode;I)V
public abstract submitFlame(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lorg/joml/Quaternionf;)V
public abstract submitLeash(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/entity/state/EntityRenderState$LeashState;)V
public abstract submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/texture/UvMapping;I)V
public submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;III)V
public submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/resources/Identifier;III)V
public submitModel(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;IIILnet/minecraft/client/resources/model/sprite/SpriteId;Lnet/minecraft/client/resources/model/sprite/SpriteGetter;I)V
public abstract submitCrumblingOverlay(Lnet/minecraft/client/model/Model;Ljava/lang/Object;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V
public submitCrumblingOverlay(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IIILnet/minecraft/client/renderer/feature/ModelFeatureRenderer$CrumblingOverlay;)V
public submitModelPart(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IILnet/minecraft/client/renderer/texture/UvMapping;)V
public submitModelPart(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IILnet/minecraft/client/renderer/texture/UvMapping;I)V
public submitModelPart(Lnet/minecraft/client/model/geom/ModelPart;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;IILnet/minecraft/client/renderer/texture/UvMapping;II)V
public abstract submitMovingBlock(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/block/MovingBlockRenderState;I)V
public abstract submitBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;Ljava/util/List;[IIII)V
public abstract submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;IZ)V
public abstract submitShapeOutline(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/client/renderer/rendertype/RenderType;IFZ)V
public abstract submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/item/ItemDisplayContext;III[ILnet/minecraft/client/resources/model/geometry/ItemQuads;Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;)V
public abstract submitCustomGeometry(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/client/renderer/SubmitNodeCollector$CustomGeometryRenderer;)V
public abstract submitQuadParticleGroup(Lnet/minecraft/client/renderer/state/level/QuadParticleRenderState;)V
public abstract submitGizmoPrimitives(Lnet/minecraft/client/renderer/gizmos/DrawableGizmoPrimitives$Group;Lnet/minecraft/client/renderer/state/level/CameraRenderState;Z)V
private static synthetic lambda$submitModelPart$0(Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
private static synthetic lambda$submitCrumblingOverlay$0(Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/rendertype/RenderType;
```
