---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (9, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer<Submit extends net.minecraft.client.renderer.feature.submit.SubmitNode> implements net.minecraft.client.renderer.feature.FeatureRenderer<Submit> {
    private net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer$Group currentGroup;
    private final java.util.List<net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer$Group> groups;
    public net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer();
    protected abstract void buildGroup(net.minecraft.client.renderer.feature.FeatureFrameContext, java.util.List<Submit>);
    protected final com.mojang.blaze3d.vertex.VertexConsumer getVertexBuilder(net.minecraft.client.renderer.rendertype.RenderType);
    private net.minecraft.client.renderer.feature.RenderTypeFeatureRenderer$Group currentGroup();
    public final void prepareGroup(net.minecraft.client.renderer.feature.FeatureFrameContext, java.util.List<Submit>, boolean);
    public void executeGroup(net.minecraft.client.renderer.feature.FeatureFrameContext, net.minecraft.client.renderer.oit.OitStage, com.mojang.renderpearl.api.commands.RenderPass, int, java.util.List<Submit>, boolean);
    public void finishExecute(net.minecraft.client.renderer.feature.FeatureFrameContext);
}
```
