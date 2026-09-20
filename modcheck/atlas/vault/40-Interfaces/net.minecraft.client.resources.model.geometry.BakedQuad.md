---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.BakedQuad"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.BakedQuad

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml` | exact | invokespecial@265 in `QuadView.toBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `<init>` | `(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml` | exact | invokespecial@52 in `AoCalculator.calcVanilla` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `direction` | `()Lnet/minecraft/core/Direction;` | exact | invokevirtual@174 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo` | `()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInf` | exact | invokevirtual@129 in `SimpleUnbakedExtraModel.bakeResolved` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `materialInfo` | `()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInf` | exact | invokevirtual@1 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo` | `()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInf` | exact | invokevirtual@68 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo` | `()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInf` | exact | invokevirtual@149 in `ExtendedBlockModelFeatureRenderer.putPartQuads` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `materialInfo` | `()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInf` | exact | invokevirtual@1 in `ExtendedBlockModelFeatureRenderer.putQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV0` | `()J` | exact | invokevirtual@55 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV1` | `()J` | exact | invokevirtual@60 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV2` | `()J` | exact | invokevirtual@66 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `packedUV3` | `()J` | exact | invokevirtual@72 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position0` | `()Lorg/joml/Vector3fc;` | exact | invokevirtual@8 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position1` | `()Lorg/joml/Vector3fc;` | exact | invokevirtual@18 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position2` | `()Lorg/joml/Vector3fc;` | exact | invokevirtual@28 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `position3` | `()Lorg/joml/Vector3fc;` | exact | invokevirtual@38 in `MutableQuadViewImpl.fromBakedQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (13 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final position0 : Lorg/joml/Vector3fc;
private final position1 : Lorg/joml/Vector3fc;
private final position2 : Lorg/joml/Vector3fc;
private final position3 : Lorg/joml/Vector3fc;
private final packedUV0 : J
private final packedUV1 : J
private final packedUV2 : J
private final packedUV3 : J
private final direction : Lnet/minecraft/core/Direction;
private final materialInfo : Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;
public static final VERTEX_COUNT : I
public static final FLAG_TRANSLUCENT : I
public static final FLAG_ANIMATED : I
public <init>(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;JJJJLnet/minecraft/core/Direction;Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;)V
public position(I)Lorg/joml/Vector3fc;
public packedUV(I)J
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public position0()Lorg/joml/Vector3fc;
public position1()Lorg/joml/Vector3fc;
public position2()Lorg/joml/Vector3fc;
public position3()Lorg/joml/Vector3fc;
public packedUV0()J
public packedUV1()J
public packedUV2()J
public packedUV3()J
public direction()Lnet/minecraft/core/Direction;
public materialInfo()Lnet/minecraft/client/resources/model/geometry/BakedQuad$MaterialInfo;
```
