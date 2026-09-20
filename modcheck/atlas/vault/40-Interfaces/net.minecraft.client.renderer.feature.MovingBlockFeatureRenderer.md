---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `buildGroup` | `@Inject at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.<init>(` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| wraps | `buildGroup` | `@Redirect at INVOKE net/minecraft/client/renderer/block/ModelBlockRenderer.tesse` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer extends net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer<net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit> {
    public static final net.minecraft.client.renderer.feature.FeatureRendererType<net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit> TYPE;
    private final com.mojang.blaze3d.vertex.PoseStack poseStack;
    public net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer();
    protected void buildGroup(net.minecraft.client.renderer.feature.FeatureFrameContext, java.util.List<net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit>);
    private void putBakedQuad(com.mojang.blaze3d.vertex.PoseStack, float, float, float, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance, net.minecraft.client.renderer.chunk.ChunkSectionLayer, int);
    private void lambda$buildGroup$1(net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit, float, float, float, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
    private void lambda$buildGroup$0(net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit, float, float, float, net.minecraft.client.resources.model.geometry.BakedQuad, com.mojang.blaze3d.vertex.QuadInstance);
    static {};
}
```
