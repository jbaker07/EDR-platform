---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureRenderDispatcher"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureRenderDispatcher

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<init>` | `(Lnet/minecraft/client/renderer/RenderBuffers;Lnet/minecraft/client/re` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `featureRenderers` | `Lnet/minecraft/client/renderer/feature/FeatureRendererMap;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |

## Declared members (7 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final modelManager : Lnet/minecraft/client/resources/model/ModelManager;
private final atlasManager : Lnet/minecraft/client/resources/model/sprite/AtlasManager;
private final font : Lnet/minecraft/client/gui/Font;
private final gameRenderState : Lnet/minecraft/client/renderer/state/GameRenderState;
private final stagedVertexBuffer : Lnet/minecraft/client/renderer/StagedVertexBuffer;
private final featureRenderers : Lnet/minecraft/client/renderer/feature/FeatureRendererMap;
private final preparedFrame : Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;
public <init>(Lnet/minecraft/client/renderer/RenderBuffers;Lnet/minecraft/client/resources/model/ModelManager;Lnet/minecraft/client/resources/model/sprite/AtlasManager;Lnet/minecraft/client/gui/Font;Lnet/minecraft/client/renderer/state/GameRenderState;)V
public prepareFrame(Lnet/minecraft/client/renderer/SubmitNodeStorage;)Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;
private prepareFrameWithContext(Lnet/minecraft/client/renderer/feature/FeatureFrameContext;Lnet/minecraft/client/renderer/SubmitNodeStorage;)Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;
public static renderAllFeatures(Lcom/mojang/renderpearl/api/commands/RenderPass;Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;)V
public close()V
private static synthetic lambda$prepareFrameWithContext$0(Lnet/minecraft/client/renderer/feature/FeatureRenderDispatcher$PreparedFrame;Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase;)V
```
