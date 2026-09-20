---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/renderer/v1/render/FabricBlockModelRenderState`, `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `identityToNull` | `(Lorg/joml/Matrix4fc;)Lorg/joml/Matrix4fc;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| calls | `scratchRandomSource` | `(J)Lnet/minecraft/util/RandomSource;` | exact | invokevirtual@19 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `scratchRandomSource` | `(J)Lnet/minecraft/util/RandomSource;` | exact | invokevirtual@49 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `setupMesh` | `(Lorg/joml/Matrix4fc;Z)Lnet/fabricmc/fabric/api/client/renderer/v1/mes` | inherited_exact | invokevirtual@28 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `()V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `clear` | `()V` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `isEmpty` | `()Z` | exact | @ModifyReturnValue at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `setupModel` | `(Lorg/joml/Matrix4fc;Z)Ljava/util/List;` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| injects_into | `submitModel` | `(Lnet/minecraft/client/renderer/rendertype/RenderType;Lcom/mojang/blaz` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS` | `[I` | exact | getstatic@79 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS` | `[I` | exact | getstatic@90 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS` | `[I` | exact | getstatic@98 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY_TINTS` | `[I` | exact | getstatic@89 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `blockLightCoords` | `I` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `modelParts` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `renderType` | `Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `tintLayers` | `Lit/unimi/dsi/fastutil/ints/IntList;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `transformation` | `Lorg/joml/Matrix4fc;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (9 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY_TINTS : [I
private modelParts : Ljava/util/List;
private transformation : Lorg/joml/Matrix4fc;
private renderType : Lnet/minecraft/client/renderer/rendertype/RenderType;
private specialRenderer : Lnet/minecraft/client/renderer/special/SpecialModelRenderer;
private specialRendererTransformation : Lorg/joml/Matrix4fc;
private tintLayers : Lit/unimi/dsi/fastutil/ints/IntList;
public blockLightCoords : I
private randomSource : Lnet/minecraft/util/RandomSource;
public <init>()V
public clear()V
public tintLayers()Lit/unimi/dsi/fastutil/ints/IntList;
public setupSpecialModel(Lnet/minecraft/client/renderer/special/SpecialModelRenderer;Lorg/joml/Matrix4fc;)V
public setupModel(Lorg/joml/Matrix4fc;Z)Ljava/util/List;
public submit(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
private static identityToNull(Lorg/joml/Matrix4fc;)Lorg/joml/Matrix4fc;
private submitModel(Lnet/minecraft/client/renderer/rendertype/RenderType;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
private static submitSpecialRenderer(Lnet/minecraft/client/renderer/special/SpecialModelRenderer;Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
public submitOnlyOutline(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
public submitWithZOffset(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/client/renderer/SubmitNodeCollector;III)V
public isEmpty()Z
public scratchRandomSource(J)Lnet/minecraft/util/RandomSource;
static <clinit>()V
```
