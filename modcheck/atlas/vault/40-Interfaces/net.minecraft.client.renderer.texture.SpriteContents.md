---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.SpriteContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.SpriteContents

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isAnimated()Z` | `` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `name()Lnet/minecraft/resources/Identifier;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.texture.SpriteContents implements java.lang.AutoCloseable,net.minecraft.client.renderer.texture.Stitcher$Entry {
    private static final org.slf4j.Logger LOGGER;
    public static final int UBO_SIZE;
    private final net.minecraft.resources.Identifier name;
    private final int width;
    private final int height;
    private final com.mojang.blaze3d.platform.NativeImage originalImage;
    private com.mojang.blaze3d.platform.NativeImage[] byMipLevel;
    private final net.minecraft.client.renderer.texture.SpriteContents$AnimatedTexture animatedTexture;
    private final java.util.List<net.minecraft.server.packs.metadata.MetadataSectionType$WithValue<?>> additionalMetadata;
    private final net.minecraft.client.renderer.texture.MipmapStrategy mipmapStrategy;
    private final float alphaCutoffBias;
    private final com.mojang.blaze3d.platform.Transparency transparency;
    public net.minecraft.client.renderer.texture.SpriteContents(net.minecraft.resources.Identifier, net.minecraft.client.resources.metadata.animation.FrameSize, com.mojang.blaze3d.platform.NativeImage);
    public net.minecraft.client.renderer.texture.SpriteContents(net.minecraft.resources.Identifier, net.minecraft.client.resources.metadata.animation.FrameSize, com.mojang.blaze3d.platform.NativeImage, java.util.Optional<net.minecraft.client.resources.metadata.animation.AnimationMetadataSection>, java.util.List<net.minecraft.server.packs.metadata.MetadataSectionType$WithValue<?>>, java.util.Optional<net.minecraft.client.resources.metadata.texture.TextureMetadataSection>);
    public void increaseMipLevel(int);
    private int getFrameCount();
    public boolean isAnimated();
    public com.mojang.blaze3d.platform.Transparency transparency();
    private net.minecraft.client.renderer.texture.SpriteContents$AnimatedTexture createAnimatedTexture(net.minecraft.client.resources.metadata.animation.FrameSize, int, int, net.minecraft.client.resources.metadata.animation.AnimationMetadataSection);
    public int width();
    public int height();
    public net.minecraft.resources.Identifier name();
    public it.unimi.dsi.fastutil.ints.IntList getUniqueFrames();
    public net.minecraft.client.renderer.texture.SpriteContents$AnimationState createAnimationState(com.mojang.renderpearl.api.buffers.GpuBufferSlice, int);
    public <T> java.util.Optional<T> getAdditionalMetadata(net.minecraft.server.packs.metadata.MetadataSectionType<T>);
    public void close();
    public java.lang.String toString();
    public boolean isTransparent(int, int, int);
    public com.mojang.blaze3d.platform.Transparency computeTransparency(float, float, float, float);
    public void uploadFirstFrame(com.mojang.renderpearl.api.textures.GpuTexture, int);
    private static boolean lambda$createAnimatedTexture$0(it.unimi.dsi.fastutil.ints.IntSet, int);
    private java.lang.String lambda$increaseMipLevel$2() throws java.lang.Exception;
    private java.lang.String lambda$increaseMipLevel$1() throws java.lang.Exception;
    private java.lang.String lambda$increaseMipLevel$0() throws java.lang.Exception;
    private net.minecraft.client.renderer.texture.SpriteContents$AnimatedTexture lambda$new$0(net.minecraft.client.resources.metadata.animation.FrameSize, com.mojang.blaze3d.platform.NativeImage, net.minecraft.client.resources.metadata.animation.AnimationMetadataSection);
    static {};
}
```
