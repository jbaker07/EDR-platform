---
type: "interface"
fqcn: "net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.item.ItemStackRenderState$LayerRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/render/FabricLayerRenderState`, `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `emitter` | `()Lnet/fabricmc/fabric/api/client/renderer/v1/mesh/QuadEmitter;` | inherited_exact | invokevirtual@13 in `CuboidItemModelWrapperMixin.onReturnUpdate` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `()V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `EMPTY_TINTS` | `[I` | exact | getstatic@163 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `submit` | `(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/S` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (14 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final NO_EXTENTS : [Lorg/joml/Vector3fc;
public static final NO_EXTENTS_SUPPLIER : Ljava/util/function/Supplier;
public static final EMPTY_TINTS : [I
private quads : Lnet/minecraft/client/resources/model/geometry/ItemQuads;
private usesBlockLight : Z
private particleMaterial : Lnet/minecraft/client/resources/model/sprite/Material$Baked;
private itemTransform : Lnet/minecraft/client/resources/model/cuboid/ItemTransform;
private final localTransform : Lorg/joml/Matrix4f;
private foilType : Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;
private tintLayers : Lit/unimi/dsi/fastutil/ints/IntList;
private specialRenderer : Lnet/minecraft/client/renderer/special/SpecialModelRenderer;
private argumentForSpecialRendering : Ljava/lang/Object;
private extents : Ljava/util/function/Supplier;
final synthetic this$0 : Lnet/minecraft/client/renderer/item/ItemStackRenderState;
public <init>(Lnet/minecraft/client/renderer/item/ItemStackRenderState;)V
public clear()V
public setQuads(Lnet/minecraft/client/resources/model/geometry/ItemQuads;)V
public setUsesBlockLight(Z)V
public setExtents(Ljava/util/function/Supplier;)V
public setParticleMaterial(Lnet/minecraft/client/resources/model/sprite/Material$Baked;)V
public setItemTransform(Lnet/minecraft/client/resources/model/cuboid/ItemTransform;)V
public setLocalTransform(Lorg/joml/Matrix4fc;)V
public setupSpecialModel(Lnet/minecraft/client/renderer/special/SpecialModelRenderer;Ljava/lang/Object;)V
private static eraseSpecialRenderer(Lnet/minecraft/client/renderer/special/SpecialModelRenderer;)Lnet/minecraft/client/renderer/special/SpecialModelRenderer;
public setFoilType(Lnet/minecraft/client/renderer/item/ItemStackRenderState$FoilType;)V
public tintLayers()Lit/unimi/dsi/fastutil/ints/IntList;
private submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
private applyTransform(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;)V
private static synthetic lambda$static$0()[Lorg/joml/Vector3fc;
static <clinit>()V
```
