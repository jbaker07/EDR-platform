---
type: "interface"
fqcn: "com.mojang.blaze3d.vertex.PoseStack"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.vertex.PoseStack

System: [[20-Systems/com.mojang.blaze3d.vertex|com.mojang.blaze3d.vertex]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `last` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@104 in `MovingBlockFeatureRendererMixin$1.accept` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `last` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@1 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `last` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@1 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `last` | `()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;` | exact | invokevirtual@1 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `mulPose` | `(Lorg/joml/Matrix4fc;)V` | exact | invokevirtual@122 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `popPose` | `()V` | exact | invokevirtual@155 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `pushPose` | `()V` | exact | invokevirtual@114 in `BlockModelRenderStateMixin.submitMesh` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final poses : Ljava/util/List;
private lastIndex : I
public <init>()V
public translate(DDD)V
public translate(FFF)V
public translate(Lnet/minecraft/world/phys/Vec3;)V
public scale(FFF)V
public rotate(Lorg/joml/Quaternionfc;)V
public rotate(Lcom/mojang/math/Axis;F)V
public rotateDegrees(Lcom/mojang/math/Axis;F)V
public rotateAround(Lorg/joml/Quaternionfc;FFF)V
public pushPose()V
public popPose()V
public last()Lcom/mojang/blaze3d/vertex/PoseStack$Pose;
public isEmpty()Z
public setIdentity()V
public mulPose(Lorg/joml/Matrix4fc;)V
public mulPose(Lcom/mojang/math/Transformation;)V
```
