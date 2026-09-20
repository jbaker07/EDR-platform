---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.AtlasManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.AtlasManager

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`, `net/minecraft/server/packs/resources/PreparableReloadListener`, `net/minecraft/client/resources/model/sprite/SpriteGetter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAtlasOrThrow` | `(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/t` | exact | invokevirtual@9 in `FabricSpriteSetImpl.getAtlas` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/renderer/texture/TextureManager;I)V` | name_only | @ModifyExpressionValue at ['FIELD'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (7 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final KNOWN_ATLASES : Ljava/util/List;
public static final PENDING_STITCH : Lnet/minecraft/server/packs/resources/PreparableReloadListener$StateKey;
private final atlasByTexture : Ljava/util/Map;
private final atlasById : Ljava/util/Map;
private spriteLookup : Ljava/util/Map;
private maxMipmapLevels : I
public <init>(Lnet/minecraft/client/renderer/texture/TextureManager;I)V
public getAtlasOrThrow(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/texture/TextureAtlas;
public forEach(Ljava/util/function/BiConsumer;)V
public updateMaxMipLevel(I)V
public close()V
public get(Lnet/minecraft/client/resources/model/sprite/SpriteId;)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public prepareSharedState(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;)V
public reload(Lnet/minecraft/server/packs/resources/PreparableReloadListener$SharedState;Ljava/util/concurrent/Executor;Lnet/minecraft/server/packs/resources/PreparableReloadListener$PreparationBarrier;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private updateSpriteMaps(Lnet/minecraft/client/resources/model/sprite/AtlasManager$PendingStitchResults;)V
private static synthetic lambda$updateSpriteMaps$0(Ljava/util/Map;Lnet/minecraft/client/resources/model/sprite/SpriteId;Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)V
private synthetic lambda$reload$2(Lnet/minecraft/client/resources/model/sprite/AtlasManager$PendingStitchResults;Ljava/lang/Object;)V
private synthetic lambda$reload$0(Lnet/minecraft/server/packs/resources/ResourceManager;Ljava/util/concurrent/Executor;Lnet/minecraft/client/resources/model/sprite/AtlasManager$PendingStitch;)V
private static synthetic lambda$reload$1(Lnet/minecraft/client/resources/model/sprite/AtlasManager$PendingStitch;Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;Ljava/lang/Throwable;)V
private static synthetic lambda$prepareSharedState$1(I)[Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$prepareSharedState$0(Ljava/util/Map;Ljava/util/List;Ljava/util/List;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/model/sprite/AtlasManager$AtlasEntry;)V
private static synthetic lambda$forEach$0(Ljava/util/function/BiConsumer;Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/model/sprite/AtlasManager$AtlasEntry;)V
static <clinit>()V
```
