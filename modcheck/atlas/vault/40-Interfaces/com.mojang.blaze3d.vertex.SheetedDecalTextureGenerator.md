---
type: "interface"
fqcn: "com.mojang.blaze3d.vertex.SheetedDecalTextureGenerator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.vertex.SheetedDecalTextureGenerator

System: [[20-Systems/com.mojang.blaze3d.vertex|com.mojang.blaze3d.vertex]]

`class` public; extends `java/lang/Object`; implements `com/mojang/blaze3d/vertex/VertexConsumer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lcom/mojang/blaze3d/vertex/` | exact | invokespecial@66 in `ExtendedBlockModelFeatureRenderer$BufferCache.getBuffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (9 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final delegate : Lcom/mojang/blaze3d/vertex/VertexConsumer;
private final cameraInversePose : Lorg/joml/Matrix4f;
private final normalInversePose : Lorg/joml/Matrix3f;
private final textureScale : F
private final worldPos : Lorg/joml/Vector3f;
private final normal : Lorg/joml/Vector3f;
private x : F
private y : F
private z : F
public <init>(Lcom/mojang/blaze3d/vertex/VertexConsumer;Lcom/mojang/blaze3d/vertex/PoseStack$Pose;F)V
public addVertex(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setColor(IIII)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setColor(I)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setUv(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setUv1(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setUv2(II)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setUv3(FF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public setNormal(FFF)Lcom/mojang/blaze3d/vertex/VertexConsumer;
public static setSheetedDecalUv(Lorg/joml/Vector3f;Lorg/joml/Vector3f;Lorg/joml/Matrix4fc;Lorg/joml/Matrix3fc;FLcom/mojang/blaze3d/vertex/VertexConsumer;)V
public setLineWidth(F)Lcom/mojang/blaze3d/vertex/VertexConsumer;
```
