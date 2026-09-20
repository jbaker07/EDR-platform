---
type: "interface"
fqcn: "com.mojang.math.MatrixUtil"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.math.MatrixUtil

System: [[20-Systems/com.mojang.math|com.mojang.math]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@6 in `ModelStateHelper.of` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@9 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@26 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@94 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@115 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@11 in `ModelStateHelper.asQuadTransform` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `isIdentity` | `(Lorg/joml/Matrix4fc;)Z` | exact | invokestatic@21 in `ModelStateHelper.lambda$asQuadTransform$1` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final G : F
private static final PI_4 : Lcom/mojang/math/GivensParameters;
private <init>()V
public static mulComponentWise(Lorg/joml/Matrix4f;F)Lorg/joml/Matrix4f;
private static approxGivensQuat(FFF)Lcom/mojang/math/GivensParameters;
private static qrGivensQuat(FF)Lcom/mojang/math/GivensParameters;
private static similarityTransform(Lorg/joml/Matrix3f;Lorg/joml/Matrix3f;)V
private static stepJacobi(Lorg/joml/Matrix3f;Lorg/joml/Matrix3f;Lorg/joml/Quaternionf;Lorg/joml/Quaternionf;)V
public static eigenvalueJacobi(Lorg/joml/Matrix3f;ILorg/joml/Quaternionf;)V
public static svdDecompose(Lorg/joml/Matrix4fc;Lorg/joml/Vector3f;Lorg/joml/Quaternionf;Lorg/joml/Vector3f;Lorg/joml/Quaternionf;)V
private static svdDecompose(Lorg/joml/Matrix3f;Lorg/joml/Quaternionf;Lorg/joml/Vector3f;Lorg/joml/Quaternionf;)V
public static checkPropertyRaw(Lorg/joml/Matrix4fc;I)Z
public static checkProperty(Lorg/joml/Matrix4fc;I)Z
public static isIdentity(Lorg/joml/Matrix4fc;)Z
public static isPureTranslation(Lorg/joml/Matrix4fc;)Z
static <clinit>()V
```
