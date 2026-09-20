---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.TextureAtlas"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.TextureAtlas

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `net/minecraft/client/renderer/texture/AbstractTexture`; implements `net/minecraft/client/renderer/texture/TickableTexture`, `net/minecraft/client/renderer/texture/Dumpable`, `net/fabricmc/fabric/api/client/renderer/v1/sprite/FabricTextureAtlas`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `upload` | `(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOCATION_BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@142 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| reads | `LOCATION_BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@79 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOCATION_BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@1 in `QuadAtlas.ofLocation` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOCATION_BLOCKS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@7 in `QuadAtlas.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOCATION_ITEMS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@15 in `QuadAtlas.ofLocation` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `LOCATION_ITEMS` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@26 in `QuadAtlas.<clinit>` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `missingSprite` | `Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |
| reads | `texturesByName` | `Ljava/util/Map;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | declared |

## Declared members (16 fields, 26 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final LOCATION_BLOCKS : Lnet/minecraft/resources/Identifier;
public static final LOCATION_ITEMS : Lnet/minecraft/resources/Identifier;
public static final LOCATION_PARTICLES : Lnet/minecraft/resources/Identifier;
private sprites : Ljava/util/List;
private animatedTexturesStates : Ljava/util/List;
private texturesByName : Ljava/util/Map;
private missingSprite : Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
private final location : Lnet/minecraft/resources/Identifier;
private final maxSupportedTextureSize : I
private width : I
private height : I
private maxMipLevel : I
private mipLevelCount : I
private mipViews : [Lcom/mojang/renderpearl/api/textures/GpuTextureView;
private spriteUbos : Lcom/mojang/renderpearl/api/buffers/GpuBuffer;
public <init>(Lnet/minecraft/resources/Identifier;)V
private createTexture(III)V
public upload(Lnet/minecraft/client/renderer/texture/SpriteLoader$Preparations;)V
private uploadInitialContents()V
public dumpContents(Lnet/minecraft/resources/Identifier;Ljava/nio/file/Path;)V
private static dumpSpriteNames(Ljava/nio/file/Path;Ljava/lang/String;Ljava/util/Map;)V
public cycleAnimationFrames()V
private uploadAnimationFrames()V
public tick()V
public getSprite(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public missingSprite()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public clearTextureData()V
protected releaseTextures()V
public close()V
public location()Lnet/minecraft/resources/Identifier;
public maxSupportedTextureSize()I
 getWidth()I
 getHeight()I
private synthetic lambda$uploadAnimationFrames$0()Ljava/lang/String;
private static synthetic lambda$dumpContents$0(I)I
private synthetic lambda$uploadInitialContents$3()Ljava/lang/String;
private static synthetic lambda$uploadInitialContents$2()Ljava/lang/String;
private static synthetic lambda$uploadInitialContents$1(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)Ljava/lang/String;
private static synthetic lambda$uploadInitialContents$0(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;)Z
private synthetic lambda$upload$0()Ljava/lang/String;
static <clinit>()V
```
