---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureRenderDispatcher"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureRenderDispatcher

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (13, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.feature.FeatureRenderDispatcher implements java.lang.AutoCloseable {
    private final net.minecraft.client.resources.model.ModelManager modelManager;
    private final net.minecraft.client.resources.model.sprite.AtlasManager atlasManager;
    private final net.minecraft.client.gui.Font font;
    private final net.minecraft.client.renderer.state.GameRenderState gameRenderState;
    private final net.minecraft.client.renderer.StagedVertexBuffer stagedVertexBuffer;
    private final net.minecraft.client.renderer.feature.FeatureRendererMap featureRenderers;
    private final net.minecraft.client.renderer.feature.FeatureRenderDispatcher$PreparedFrame preparedFrame;
    public net.minecraft.client.renderer.feature.FeatureRenderDispatcher(net.minecraft.client.renderer.RenderBuffers, net.minecraft.client.resources.model.ModelManager, net.minecraft.client.resources.model.sprite.AtlasManager, net.minecraft.client.gui.Font, net.minecraft.client.renderer.state.GameRenderState);
    public net.minecraft.client.renderer.feature.FeatureRenderDispatcher$PreparedFrame prepareFrame(net.minecraft.client.renderer.SubmitNodeStorage);
    private net.minecraft.client.renderer.feature.FeatureRenderDispatcher$PreparedFrame prepareFrameWithContext(net.minecraft.client.renderer.feature.FeatureFrameContext, net.minecraft.client.renderer.SubmitNodeStorage);
    public static void renderAllFeatures(com.mojang.renderpearl.api.commands.RenderPass, net.minecraft.client.renderer.feature.FeatureRenderDispatcher$PreparedFrame);
    public void close();
    private static void lambda$prepareFrameWithContext$0(net.minecraft.client.renderer.feature.FeatureRenderDispatcher$PreparedFrame, net.minecraft.client.renderer.feature.phase.FeatureRenderPhase);
}
```
