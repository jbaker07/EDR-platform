---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureFrameContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureFrameContext

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `blockColors()Lnet/minecraft/client/color/block/BlockColors;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `options()Lnet/minecraft/client/renderer/state/OptionsRenderState;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.feature.FeatureFrameContext extends java.lang.Record {
    private final net.minecraft.client.renderer.state.OptionsRenderState options;
    private final net.minecraft.client.gui.Font font;
    private final net.minecraft.client.renderer.block.BlockStateModelSet blockStateModelSet;
    private final net.minecraft.client.color.block.BlockColors blockColors;
    private final net.minecraft.client.renderer.texture.TextureManager textureManager;
    private final net.minecraft.client.resources.model.sprite.AtlasManager atlasManager;
    private final com.mojang.renderpearl.api.textures.GpuTextureView lightmap;
    private final net.minecraft.client.renderer.StagedVertexBuffer stagedVertexBuffer;
    public net.minecraft.client.renderer.feature.FeatureFrameContext(net.minecraft.client.renderer.state.OptionsRenderState, net.minecraft.client.gui.Font, net.minecraft.client.renderer.block.BlockStateModelSet, net.minecraft.client.color.block.BlockColors, net.minecraft.client.renderer.texture.TextureManager, net.minecraft.client.resources.model.sprite.AtlasManager, com.mojang.renderpearl.api.textures.GpuTextureView, net.minecraft.client.renderer.StagedVertexBuffer);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.client.renderer.state.OptionsRenderState options();
    public net.minecraft.client.gui.Font font();
    public net.minecraft.client.renderer.block.BlockStateModelSet blockStateModelSet();
    public net.minecraft.client.color.block.BlockColors blockColors();
    public net.minecraft.client.renderer.texture.TextureManager textureManager();
    public net.minecraft.client.resources.model.sprite.AtlasManager atlasManager();
    public com.mojang.renderpearl.api.textures.GpuTextureView lightmap();
    public net.minecraft.client.renderer.StagedVertexBuffer stagedVertexBuffer();
}
```
