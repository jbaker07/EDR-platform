---
type: "interface"
fqcn: "com.mojang.math.Transformation"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.math.Transformation

System: [[20-Systems/com.mojang.math|com.mojang.math]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `compose` | `(Lcom/mojang/math/Transformation;)Lcom/mojang/math/Transformation;` | exact | invokevirtual@46 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getMatrix` | `()Lorg/joml/Matrix4fc;` | exact | invokevirtual@1 in `ModelStateHelper.of` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getMatrix` | `()Lorg/joml/Matrix4fc;` | exact | invokevirtual@83 in `ModelStateHelper.of` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getMatrix` | `()Lorg/joml/Matrix4fc;` | exact | invokevirtual@6 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getMatrix` | `()Lorg/joml/Matrix4fc;` | exact | invokevirtual@23 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getMatrix` | `()Lorg/joml/Matrix4fc;` | exact | invokevirtual@6 in `ModelStateHelper.asQuadTransform` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (12 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final ZERO_TRANSLATION : Lorg/joml/Vector3fc;
private static final UNIT_SCALE : Lorg/joml/Vector3fc;
private static final ZERO_ROTATION : Lorg/joml/Quaternionfc;
private final matrix : Lorg/joml/Matrix4fc;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final EXTENDED_CODEC : Lcom/mojang/serialization/Codec;
private decomposed : Z
private translation : Lorg/joml/Vector3fc;
private leftRotation : Lorg/joml/Quaternionfc;
private scale : Lorg/joml/Vector3fc;
private rightRotation : Lorg/joml/Quaternionfc;
public static final IDENTITY : Lcom/mojang/math/Transformation;
public <init>(Lorg/joml/Matrix4fc;)V
public <init>(Lorg/joml/Vector3fc;Lorg/joml/Quaternionfc;Lorg/joml/Vector3fc;Lorg/joml/Quaternionfc;)V
public compose(Lcom/mojang/math/Transformation;)Lcom/mojang/math/Transformation;
public inverse()Lcom/mojang/math/Transformation;
private ensureDecomposed()V
private static compose(Lorg/joml/Vector3fc;Lorg/joml/Quaternionfc;Lorg/joml/Vector3fc;Lorg/joml/Quaternionfc;)Lorg/joml/Matrix4f;
public getMatrix()Lorg/joml/Matrix4fc;
public getMatrixCopy()Lorg/joml/Matrix4f;
public translation()Lorg/joml/Vector3fc;
public leftRotation()Lorg/joml/Quaternionfc;
public scale()Lorg/joml/Vector3fc;
public rightRotation()Lorg/joml/Quaternionfc;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public slerp(Lcom/mojang/math/Transformation;F)Lcom/mojang/math/Transformation;
public static compose(Lorg/joml/Matrix4fc;Ljava/util/Optional;)Lorg/joml/Matrix4fc;
private static synthetic lambda$static$1()Lcom/mojang/math/Transformation;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
