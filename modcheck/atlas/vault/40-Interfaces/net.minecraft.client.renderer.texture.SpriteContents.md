---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.SpriteContents"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.SpriteContents

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`, `net/minecraft/client/renderer/texture/Stitcher$Entry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `computeTransparency` | `(FFFF)Lcom/mojang/blaze3d/platform/Transparency;` | exact | invokevirtual@171 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isAnimated` | `()Z` | exact | invokevirtual@35 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isAnimated` | `()Z` | exact | invokevirtual@218 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `name` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@173 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `name` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@67 in `SimpleUnbakedExtraModel.lambda$bakeResolved$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `name` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@75 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `name` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@71 in `SimpleModelWrapperMixin.lambda$analyzeMesh$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (12 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final UBO_SIZE : I
private final name : Lnet/minecraft/resources/Identifier;
private final width : I
private final height : I
private final originalImage : Lcom/mojang/blaze3d/platform/NativeImage;
private byMipLevel : [Lcom/mojang/blaze3d/platform/NativeImage;
private final animatedTexture : Lnet/minecraft/client/renderer/texture/SpriteContents$AnimatedTexture;
private final additionalMetadata : Ljava/util/List;
private final mipmapStrategy : Lnet/minecraft/client/renderer/texture/MipmapStrategy;
private final alphaCutoffBias : F
private final transparency : Lcom/mojang/blaze3d/platform/Transparency;
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/metadata/animation/FrameSize;Lcom/mojang/blaze3d/platform/NativeImage;)V
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/resources/metadata/animation/FrameSize;Lcom/mojang/blaze3d/platform/NativeImage;Ljava/util/Optional;Ljava/util/List;Ljava/util/Optional;)V
public increaseMipLevel(I)V
private getFrameCount()I
public isAnimated()Z
public transparency()Lcom/mojang/blaze3d/platform/Transparency;
private createAnimatedTexture(Lnet/minecraft/client/resources/metadata/animation/FrameSize;IILnet/minecraft/client/resources/metadata/animation/AnimationMetadataSection;)Lnet/minecraft/client/renderer/texture/SpriteContents$AnimatedTexture;
public width()I
public height()I
public name()Lnet/minecraft/resources/Identifier;
public getUniqueFrames()Lit/unimi/dsi/fastutil/ints/IntList;
public createAnimationState(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;I)Lnet/minecraft/client/renderer/texture/SpriteContents$AnimationState;
public getAdditionalMetadata(Lnet/minecraft/server/packs/metadata/MetadataSectionType;)Ljava/util/Optional;
public close()V
public toString()Ljava/lang/String;
public isTransparent(III)Z
public computeTransparency(FFFF)Lcom/mojang/blaze3d/platform/Transparency;
public uploadFirstFrame(Lcom/mojang/renderpearl/api/textures/GpuTexture;I)V
private static synthetic lambda$createAnimatedTexture$0(Lit/unimi/dsi/fastutil/ints/IntSet;I)Z
private synthetic lambda$increaseMipLevel$2()Ljava/lang/String;
private synthetic lambda$increaseMipLevel$1()Ljava/lang/String;
private synthetic lambda$increaseMipLevel$0()Ljava/lang/String;
private synthetic lambda$new$0(Lnet/minecraft/client/resources/metadata/animation/FrameSize;Lcom/mojang/blaze3d/platform/NativeImage;Lnet/minecraft/client/resources/metadata/animation/AnimationMetadataSection;)Lnet/minecraft/client/renderer/texture/SpriteContents$AnimatedTexture;
static <clinit>()V
```
