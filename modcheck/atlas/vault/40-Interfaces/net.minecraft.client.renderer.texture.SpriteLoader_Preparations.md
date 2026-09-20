---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.SpriteLoader$Preparations"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.SpriteLoader$Preparations

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `missing()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `spriteFinder()Lnet/fabricmc/fabric/api/client/renderer/v1/sprite/SpriteF` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.renderer.texture.SpriteLoader$Preparations extends java.lang.Record {
    private final int width;
    private final int height;
    private final int mipLevel;
    private final net.minecraft.client.renderer.texture.TextureAtlasSprite missing;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlasSprite> regions;
    private final java.util.concurrent.CompletableFuture<java.lang.Void> readyForUpload;
    public net.minecraft.client.renderer.texture.SpriteLoader$Preparations(int, int, int, net.minecraft.client.renderer.texture.TextureAtlasSprite, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlasSprite>, java.util.concurrent.CompletableFuture<java.lang.Void>);
    public net.minecraft.client.renderer.texture.TextureAtlasSprite getSprite(net.minecraft.resources.Identifier);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public int width();
    public int height();
    public int mipLevel();
    public net.minecraft.client.renderer.texture.TextureAtlasSprite missing();
    public java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlasSprite> regions();
    public java.util.concurrent.CompletableFuture<java.lang.Void> readyForUpload();
}
```
