---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.TextureAtlas"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.TextureAtlas

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `upload` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (42, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.texture.TextureAtlas extends net.minecraft.client.renderer.texture.AbstractTexture implements net.minecraft.client.renderer.texture.TickableTexture,net.minecraft.client.renderer.texture.Dumpable {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.resources.Identifier LOCATION_BLOCKS;
    public static final net.minecraft.resources.Identifier LOCATION_ITEMS;
    public static final net.minecraft.resources.Identifier LOCATION_PARTICLES;
    private java.util.List<net.minecraft.client.renderer.texture.TextureAtlasSprite> sprites;
    private java.util.List<net.minecraft.client.renderer.texture.SpriteContents$AnimationState> animatedTexturesStates;
    private java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlasSprite> texturesByName;
    private net.minecraft.client.renderer.texture.TextureAtlasSprite missingSprite;
    private final net.minecraft.resources.Identifier location;
    private final int maxSupportedTextureSize;
    private int width;
    private int height;
    private int maxMipLevel;
    private int mipLevelCount;
    private com.mojang.renderpearl.api.textures.GpuTextureView[] mipViews;
    private com.mojang.renderpearl.api.buffers.GpuBuffer spriteUbos;
    public net.minecraft.client.renderer.texture.TextureAtlas(net.minecraft.resources.Identifier);
    private void createTexture(int, int, int);
    public void upload(net.minecraft.client.renderer.texture.SpriteLoader$Preparations);
    private void uploadInitialContents();
    public void dumpContents(net.minecraft.resources.Identifier, java.nio.file.Path) throws java.io.IOException;
    private static void dumpSpriteNames(java.nio.file.Path, java.lang.String, java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlasSprite>);
    public void cycleAnimationFrames();
    private void uploadAnimationFrames();
    public void tick();
    public net.minecraft.client.renderer.texture.TextureAtlasSprite getSprite(net.minecraft.resources.Identifier);
    public net.minecraft.client.renderer.texture.TextureAtlasSprite missingSprite();
    public void clearTextureData();
    protected void releaseTextures();
    public void close();
    public net.minecraft.resources.Identifier location();
    public int maxSupportedTextureSize();
    int getWidth();
    int getHeight();
    private java.lang.String lambda$uploadAnimationFrames$0();
    private static int lambda$dumpContents$0(int);
    private java.lang.String lambda$uploadInitialContents$3();
    private static java.lang.String lambda$uploadInitialContents$2();
    private static java.lang.String lambda$uploadInitialContents$1(net.minecraft.client.renderer.texture.TextureAtlasSprite);
    private static boolean lambda$uploadInitialContents$0(net.minecraft.client.renderer.texture.TextureAtlasSprite);
    private java.lang.String lambda$upload$0();
    static {};
}
```
