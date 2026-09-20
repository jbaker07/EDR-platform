---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;L` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `itemRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `layer()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `layer()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `lightEmission()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `shadeDirectionOverride()Lnet/minecraft/core/Direction;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `sprite()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `tintIndex()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `tintIndex()I` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo extends java.lang.Record {
    private final net.minecraft.client.renderer.texture.TextureAtlasSprite sprite;
    private final net.minecraft.client.renderer.chunk.ChunkSectionLayer layer;
    private final net.minecraft.client.renderer.rendertype.RenderType itemRenderType;
    private final net.minecraft.client.renderer.rendertype.RenderType itemGlintRenderType;
    private final net.minecraft.client.renderer.rendertype.RenderType itemGlintSpecialRenderType;
    private final int tintIndex;
    private final net.minecraft.core.Direction shadeDirectionOverride;
    private final int lightEmission;
    public net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo(net.minecraft.client.renderer.texture.TextureAtlasSprite, net.minecraft.client.renderer.chunk.ChunkSectionLayer, net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.client.renderer.rendertype.RenderType, net.minecraft.client.renderer.rendertype.RenderType, int, net.minecraft.core.Direction, int);
    public static net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo of(net.minecraft.client.resources.model.sprite.Material$Baked, com.mojang.blaze3d.platform.Transparency, int, net.minecraft.core.Direction, int);
    public boolean isTinted();
    public int flags();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.client.renderer.texture.TextureAtlasSprite sprite();
    public net.minecraft.client.renderer.chunk.ChunkSectionLayer layer();
    public net.minecraft.client.renderer.rendertype.RenderType itemRenderType();
    public net.minecraft.client.renderer.rendertype.RenderType itemGlintRenderType();
    public net.minecraft.client.renderer.rendertype.RenderType itemGlintSpecialRenderType();
    public int tintIndex();
    public net.minecraft.core.Direction shadeDirectionOverride();
    public int lightEmission();
}
```
