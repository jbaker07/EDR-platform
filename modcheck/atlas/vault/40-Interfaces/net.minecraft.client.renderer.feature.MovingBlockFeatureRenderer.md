---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `net/minecraft/client/renderer/feature/RenderTypeFeatureRenderer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `buildGroup` | `(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `poseStack` | `Lcom/mojang/blaze3d/vertex/PoseStack;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| wraps | `buildGroup` | `(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final TYPE : Lnet/minecraft/client/renderer/feature/FeatureRendererType;
private final poseStack : Lcom/mojang/blaze3d/vertex/PoseStack;
public <init>()V
protected buildGroup(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Ljava/util/List;)V
private putBakedQuad(Lcom/mojang/blaze3d/vertex/PoseStack;FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;I)V
private synthetic lambda$buildGroup$1(Lnet/minecraft/client/renderer/feature/MovingBlockFeatureRenderer$Submit;FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
private synthetic lambda$buildGroup$0(Lnet/minecraft/client/renderer/feature/MovingBlockFeatureRenderer$Submit;FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
static <clinit>()V
```
