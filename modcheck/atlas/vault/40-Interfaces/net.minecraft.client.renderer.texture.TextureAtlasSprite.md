---
type: "interface"
fqcn: "net.minecraft.client.renderer.texture.TextureAtlasSprite"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.texture.TextureAtlasSprite

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/texture/UvMapping`, `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@139 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@165 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@59 in `SimpleUnbakedExtraModel.lambda$bakeResolved$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@76 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@63 in `SimpleModelWrapperMixin.lambda$analyzeMesh$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `atlasLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@185 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@170 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@64 in `SimpleUnbakedExtraModel.lambda$bakeResolved$0` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@32 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@162 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@72 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@68 in `SimpleModelWrapperMixin.lambda$analyzeMesh$0` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `contents` | `()Lnet/minecraft/client/renderer/texture/SpriteContents;` | exact | invokevirtual@215 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getU` | `(F)F` | exact | invokevirtual@145 in `ModelStateHelper.lambda$asQuadTransform$1` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@97 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@119 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@140 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@5 in `ModelStateHelper.getFrameFromU` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@12 in `ModelStateHelper.getFrameFromU` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@1 in `QuadSpriteBaker.interpolate` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@1 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@82 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU0` | `()F` | exact | invokevirtual@126 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@93 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@1 in `ModelStateHelper.getFrameFromU` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@6 in `QuadSpriteBaker.interpolate` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@11 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@92 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getU1` | `()F` | exact | invokevirtual@147 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV` | `(F)F` | exact | invokevirtual@152 in `ModelStateHelper.lambda$asQuadTransform$1` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@110 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@129 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@152 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@5 in `ModelStateHelper.getFrameFromV` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@12 in `ModelStateHelper.getFrameFromV` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@13 in `QuadSpriteBaker.interpolate` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@21 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@102 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV0` | `()F` | exact | invokevirtual@168 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@106 in `ModelHelper.computeTransparency` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@1 in `ModelStateHelper.getFrameFromV` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@19 in `QuadSpriteBaker.interpolate` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@31 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@112 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getV1` | `()F` | exact | invokevirtual@190 in `SpriteFinderImpl$Node.add` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (9 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final atlasLocation : Lnet/minecraft/resources/Identifier;
private final contents : Lnet/minecraft/client/renderer/texture/SpriteContents;
private final x : I
private final y : I
private final u0 : F
private final u1 : F
private final v0 : F
private final v1 : F
private final padding : I
protected <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/renderer/texture/SpriteContents;IIIII)V
public getX()I
public getY()I
public getU0()F
public getU1()F
public contents()Lnet/minecraft/client/renderer/texture/SpriteContents;
public createAnimationState(Lcom/mojang/renderpearl/api/buffers/GpuBufferSlice;I)Lnet/minecraft/client/renderer/texture/SpriteContents$AnimationState;
public transparency()Lcom/mojang/blaze3d/platform/Transparency;
public getU(F)F
public getV0()F
public getV1()F
public getV(F)F
public atlasLocation()Lnet/minecraft/resources/Identifier;
public toString()Ljava/lang/String;
public uploadFirstFrame(Lcom/mojang/renderpearl/api/textures/GpuTexture;I)V
public isAnimated()Z
public uploadSpriteUbo(Ljava/nio/ByteBuffer;IIIII)V
public close()V
```
