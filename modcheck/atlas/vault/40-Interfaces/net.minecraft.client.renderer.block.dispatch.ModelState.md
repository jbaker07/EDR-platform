---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.ModelState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.ModelState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `faceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@5 in `ModelStateHelper$4.faceTransformation` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `faceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@89 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `faceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@110 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `faceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@193 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `faceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@201 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `inverseFaceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@5 in `ModelStateHelper$4.inverseFaceTransformation` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `inverseFaceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@231 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `inverseFaceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@239 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `inverseFaceTransformation` | `(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;` | exact | invokeinterface@12 in `ModelStateHelper.lambda$asQuadTransform$1` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `transformation` | `()Lcom/mojang/math/Transformation;` | exact | invokeinterface@1 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `transformation` | `()Lcom/mojang/math/Transformation;` | exact | invokeinterface@18 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `transformation` | `()Lcom/mojang/math/Transformation;` | exact | invokeinterface@35 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `transformation` | `()Lcom/mojang/math/Transformation;` | exact | invokeinterface@41 in `ModelStateHelper.multiply` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `transformation` | `()Lcom/mojang/math/Transformation;` | exact | invokeinterface@1 in `ModelStateHelper.asQuadTransform` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (1 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_TRANSFORM : Lorg/joml/Matrix4fc;
public transformation()Lcom/mojang/math/Transformation;
public faceTransformation(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;
public inverseFaceTransformation(Lnet/minecraft/core/Direction;)Lorg/joml/Matrix4fc;
static <clinit>()V
```
