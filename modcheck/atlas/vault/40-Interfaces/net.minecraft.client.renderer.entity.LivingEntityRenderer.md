---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.LivingEntityRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.LivingEntityRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`abstract_class` public abstract; extends `net/minecraft/client/renderer/entity/EntityRenderer`; implements `net/minecraft/client/renderer/entity/RenderLayerParent`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `extractRenderState` | `(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/client/render` | exact | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (4 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final EYE_BED_OFFSET : F
protected model : Lnet/minecraft/client/model/EntityModel;
protected final itemModelResolver : Lnet/minecraft/client/renderer/item/ItemModelResolver;
protected final layers : Ljava/util/List;
public <init>(Lnet/minecraft/client/renderer/entity/EntityRendererProvider$Context;Lnet/minecraft/client/model/EntityModel;F)V
protected final addLayer(Lnet/minecraft/client/renderer/entity/layers/RenderLayer;)Z
public getModel()Lnet/minecraft/client/model/EntityModel;
protected getBoundingBoxForCulling(Lnet/minecraft/world/entity/LivingEntity;F)Lnet/minecraft/world/phys/AABB;
public submit(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V
protected shouldRenderLayers(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Z
protected getModelTint(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)I
public abstract getTextureLocation(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Lnet/minecraft/resources/Identifier;
protected getRenderType(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;ZZZ)Lnet/minecraft/client/renderer/rendertype/RenderType;
public static getOverlayCoords(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;F)I
protected isBodyVisible(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Z
private static sleepDirectionToRotation(Lnet/minecraft/core/Direction;)F
protected isShaking(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)Z
protected setupRotations(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;FF)V
protected getFlipDegrees()F
protected getWhiteOverlayProgress(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)F
protected scale(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;)V
protected shouldShowName(Lnet/minecraft/world/entity/LivingEntity;D)Z
public isEntityUpsideDown(Lnet/minecraft/world/entity/LivingEntity;)Z
protected static isUpsideDownName(Ljava/lang/String;)Z
protected getShadowRadius(Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;)F
public extractRenderState(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;F)V
protected extractNameTags(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/client/renderer/entity/state/LivingEntityRenderState;F)V
private static solveBodyRot(Lnet/minecraft/world/entity/LivingEntity;FF)F
protected synthetic extractNameTags(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/client/renderer/entity/state/EntityRenderState;F)V
public synthetic extractRenderState(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/client/renderer/entity/state/EntityRenderState;F)V
protected synthetic getShadowRadius(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;)F
protected synthetic shouldShowName(Lnet/minecraft/world/entity/Entity;D)Z
public synthetic submit(Lnet/minecraft/client/renderer/entity/state/EntityRenderState;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;Lnet/minecraft/client/renderer/state/level/CameraRenderState;)V
protected synthetic getBoundingBoxForCulling(Lnet/minecraft/world/entity/Entity;F)Lnet/minecraft/world/phys/AABB;
```
