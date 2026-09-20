---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.AtlasManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.AtlasManager

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getAtlasOrThrow(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (24, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.resources.model.sprite.AtlasManager implements java.lang.AutoCloseable,net.minecraft.server.packs.resources.PreparableReloadListener,net.minecraft.client.resources.model.sprite.SpriteGetter {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.List<net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig> KNOWN_ATLASES;
    public static final net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<net.minecraft.client.resources.model.sprite.AtlasManager$PendingStitchResults> PENDING_STITCH;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.AtlasManager$AtlasEntry> atlasByTexture;
    private final java.util.Map<net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.AtlasManager$AtlasEntry> atlasById;
    private java.util.Map<net.minecraft.client.resources.model.sprite.SpriteId, net.minecraft.client.renderer.texture.TextureAtlasSprite> spriteLookup;
    private int maxMipmapLevels;
    public net.minecraft.client.resources.model.sprite.AtlasManager(net.minecraft.client.renderer.texture.TextureManager, int);
    public net.minecraft.client.renderer.texture.TextureAtlas getAtlasOrThrow(net.minecraft.resources.Identifier);
    public void forEach(java.util.function.BiConsumer<net.minecraft.resources.Identifier, net.minecraft.client.renderer.texture.TextureAtlas>);
    public void updateMaxMipLevel(int);
    public void close();
    public net.minecraft.client.renderer.texture.TextureAtlasSprite get(net.minecraft.client.resources.model.sprite.SpriteId);
    public void prepareSharedState(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState);
    public java.util.concurrent.CompletableFuture<java.lang.Void> reload(net.minecraft.server.packs.resources.PreparableReloadListener$SharedState, java.util.concurrent.Executor, net.minecraft.server.packs.resources.PreparableReloadListener$PreparationBarrier, java.util.concurrent.Executor);
    private void updateSpriteMaps(net.minecraft.client.resources.model.sprite.AtlasManager$PendingStitchResults);
    private static void lambda$updateSpriteMaps$0(java.util.Map, net.minecraft.client.resources.model.sprite.SpriteId, net.minecraft.client.renderer.texture.TextureAtlasSprite);
    private void lambda$reload$2(net.minecraft.client.resources.model.sprite.AtlasManager$PendingStitchResults, java.lang.Object);
    private void lambda$reload$0(net.minecraft.server.packs.resources.ResourceManager, java.util.concurrent.Executor, net.minecraft.client.resources.model.sprite.AtlasManager$PendingStitch);
    private static void lambda$reload$1(net.minecraft.client.resources.model.sprite.AtlasManager$PendingStitch, net.minecraft.client.renderer.texture.SpriteLoader$Preparations, java.lang.Throwable);
    private static java.util.concurrent.CompletableFuture[] lambda$prepareSharedState$1(int);
    private static void lambda$prepareSharedState$0(java.util.Map, java.util.List, java.util.List, net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.AtlasManager$AtlasEntry);
    private static void lambda$forEach$0(java.util.function.BiConsumer, net.minecraft.resources.Identifier, net.minecraft.client.resources.model.sprite.AtlasManager$AtlasEntry);
    static {};
}
```
