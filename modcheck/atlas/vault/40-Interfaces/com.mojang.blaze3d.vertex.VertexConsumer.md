---
type: "interface"
fqcn: "com.mojang.blaze3d.vertex.VertexConsumer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.vertex.VertexConsumer

System: [[20-Systems/com.mojang.blaze3d.vertex|com.mojang.blaze3d.vertex]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addVertex` | `(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@22 in `ItemSheetedDecalTextureGenerator.addVertex` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@83 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@182 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@299 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@134 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@271 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `addVertex` | `(FFFIFFIIFFF)V` | exact | invokeinterface@435 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `putBakedQuad` | `(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lnet/minecraft/client/resou` | exact | invokeinterface@57 in `ExtendedBlockModelFeatureRenderer.putQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setColor` | `(I)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@5 in `ItemSheetedDecalTextureGenerator.setColor` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setColor` | `(IIII)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@9 in `ItemSheetedDecalTextureGenerator.setColor` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setLineWidth` | `(F)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@5 in `ItemSheetedDecalTextureGenerator.setLineWidth` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setNormal` | `(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@7 in `ItemSheetedDecalTextureGenerator.setNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setUv` | `(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@6 in `ItemSheetedDecalTextureGenerator.setUv` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setUv1` | `(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@6 in `ItemSheetedDecalTextureGenerator.setUv1` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setUv2` | `(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@6 in `ItemSheetedDecalTextureGenerator.setUv2` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `setUv3` | `(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;` | exact | invokeinterface@131 in `ItemSheetedDecalTextureGenerator.setNormal` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (0 fields, 23 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract addVertex(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setColor(IIII)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setColor(I)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setUv(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setUv1(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setUv2(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setUv3(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setNormal(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public abstract setLineWidth(F)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public addVertex(FFFIFFIIFFF)V
public setColor(FFFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setLight(I)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setOverlay(I)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public putBlockBakedQuad(FFFLnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
public putBakedQuad(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;)V
public putBakedQuadWithGlint(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lnet/minecraft/client/resources/model/geometry/BakedQuad;Lcom/mojang/blaze3d/vertex/QuadInstance;Lcom/mojang/blaze3d/vertex/PoseStack$Pose;)V
public addVertex(Lorg/joml/Vector3fc;)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public addVertex(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lorg/joml/Vector3fc;)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public addVertex(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public addVertex(Lorg/joml/Matrix4fc;FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public addVertexWith2DPose(Lorg/joml/Matrix3x2fc;FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setNormal(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setNormal(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;Lorg/joml/Vector3fc;)Lcom/mojang/blaze3d/vertex/VertexConsumer;
```
