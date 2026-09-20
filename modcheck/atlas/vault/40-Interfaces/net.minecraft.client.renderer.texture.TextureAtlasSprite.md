---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.TextureAtlasSprite"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.TextureAtlasSprite

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `atlasLocation()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `atlasLocation()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `contents()Lnet/minecraft/client/renderer/texture/SpriteContents;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `contents()Lnet/minecraft/client/renderer/texture/SpriteContents;` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getU0()F` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1()F` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0()F` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1()F` | `` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.texture.TextureAtlasSprite implements net.minecraft.client.renderer.texture.UvMapping,java.lang.AutoCloseable {
    private final net.minecraft.resources.Identifier atlasLocation;
    private final net.minecraft.client.renderer.texture.SpriteContents contents;
    private final int x;
    private final int y;
    private final float u0;
    private final float u1;
    private final float v0;
    private final float v1;
    private final int padding;
    protected net.minecraft.client.renderer.texture.TextureAtlasSprite(net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.SpriteContents, int, int, int, int, int);
    public int getX();
    public int getY();
    public float getU0();
    public float getU1();
    public net.minecraft.client.renderer.texture.SpriteContents contents();
    public net.minecraft.client.renderer.texture.SpriteContents$AnimationState createAnimationState(com.mojang.renderpearl.api.buffers.GpuBufferSlice, int);
    public com.mojang.blaze3d.platform.Transparency transparency();
    public float getU(float);
    public float getV0();
    public float getV1();
    public float getV(float);
    public net.minecraft.resources.Identifier atlasLocation();
    public java.lang.String toString();
    public void uploadFirstFrame(com.mojang.renderpearl.api.textures.GpuTexture, int);
    public boolean isAnimated();
    public void uploadSpriteUbo(java.nio.ByteBuffer, int, int, int, int, int);
    public void close();
}
```
