---
type: "interface"
fqcn: "com.mojang.blaze3d.vertex.PoseStack$Pose"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.vertex.PoseStack$Pose

System: [[20-Systems/com.mojang.blaze3d.vertex|com.mojang.blaze3d.vertex]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `copy` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@4 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `copy` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@4 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `copy` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@4 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `normal` | `()Lorg/joml/Matrix3f;` | exact | invokevirtual@222 in `ExtendedItemFeatureRenderer.bufferMain` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix4f;` | exact | invokevirtual@4 in `ExtendedBlockModelSubmit.distanceToCameraSq` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix4f;` | exact | invokevirtual@4 in `ExtendedItemSubmit.distanceToCameraSq` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix4f;` | exact | invokevirtual@13 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `pose` | `()Lorg/joml/Matrix4f;` | exact | invokevirtual@206 in `ExtendedItemFeatureRenderer.bufferMain` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `transformNormal` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3f;)Lorg/joml/Vector3f;` | exact | invokevirtual@32 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `transformNormal` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3f;)Lorg/joml/Vector3f;` | exact | invokevirtual@211 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `transformNormal` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3f;)Lorg/joml/Vector3f;` | exact | invokevirtual@294 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `transformNormal` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3f;)Lorg/joml/Vector3f;` | exact | invokevirtual@364 in `QuadViewImpl.buffer` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (3 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final pose : Lorg/joml/Matrix4f;
private final normal : Lorg/joml/Matrix3f;
private trustedNormals : Z
public <init>()V
private computeNormalMatrix()V
public set(Lcom/mojang/blaze3d/vertex/PoseStack$Pose;)V
public pose()Lorg/joml/Matrix4f;
public normal()Lorg/joml/Matrix3f;
public transformNormal(Lorg/joml/Vector3fc;Lorg/joml/Vector3f;)Lorg/joml/Vector3f;
public transformNormal(FFFLorg/joml/Vector3f;)Lorg/joml/Vector3f;
public translate(FFF)Lorg/joml/Matrix4f;
public scale(FFF)V
public rotate(Lorg/joml/Quaternionfc;)V
public rotate(Lcom/mojang/math/Axis;F)V
public rotateDegrees(Lcom/mojang/math/Axis;F)V
public rotateAround(Lorg/joml/Quaternionfc;FFF)V
public setIdentity()V
public mulPose(Lorg/joml/Matrix4fc;)V
public mulPose(Lcom/mojang/math/Transformation;)V
public copy()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;
```
