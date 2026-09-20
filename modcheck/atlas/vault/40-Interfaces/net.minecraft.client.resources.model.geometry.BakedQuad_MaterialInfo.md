---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.BakedQuad$MaterialInfo

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecr` | exact | invokespecial@234 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecr` | exact | invokespecial@156 in `AoCalculator.<init>` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `itemRenderType` | `()Lnet/minecraft/client/renderer/rendertype/RenderType;` | exact | invokevirtual@236 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `layer` | `()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | invokevirtual@227 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `layer` | `()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | invokevirtual@71 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `layer` | `()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;` | exact | invokevirtual@152 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `lightEmission` | `()I` | exact | invokevirtual@140 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `shadeDirectionOverride` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@254 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@132 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@182 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `sprite` | `()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;` | exact | invokevirtual@212 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `tintIndex` | `()I` | exact | invokevirtual@245 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `tintIndex` | `()I` | exact | invokevirtual@4 in `ExtendedBlockModelFeatureRenderer.putQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (8 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final sprite : Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
private final layer : Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
private final itemRenderType : Lnet/minecraft/client/renderer/rendertype/RenderType;
private final itemGlintRenderType : Lnet/minecraft/client/renderer/rendertype/RenderType;
private final itemGlintSpecialRenderType : Lnet/minecraft/client/renderer/rendertype/RenderType;
private final tintIndex : I
private final shadeDirectionOverride : Lnet/minecraft/core/Direction;
private final lightEmission : I
public <init>(Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/client/renderer/rendertype/RenderType;Lnet/minecraft/client/renderer/rendertype/RenderType;ILnet/minecraft/core/Direction;I)V
public static of(Lnet/minecraft/client/resources/model/sprite/Material$Baked;Lcom/mojang/blaze3d/platform/Transparency;ILnet/minecraft/core/Direction;I)Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;
public isTinted()Z
public flags()I
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public sprite()Lnet/minecraft/client/renderer/texture/TextureAtlasSprite;
public layer()Lnet/minecraft/client/renderer/chunk/ChunkSectionLayer;
public itemRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType;
public itemGlintRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType;
public itemGlintSpecialRenderType()Lnet/minecraft/client/renderer/rendertype/RenderType;
public tintIndex()I
public shadeDirectionOverride()Lnet/minecraft/core/Direction;
public lightEmission()I
```
