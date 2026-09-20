---
type: "interface"
fqcn: "net.minecraft.core.BlockMath"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.BlockMath

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getFaceTransformation` | `(Lcom/mojang/math/Transformation;Lnet/minecraft/core/Direction;)Lcom/m` | exact | invokestatic@80 in `ModelStateHelper.of` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final VANILLA_UV_TRANSFORM_LOCAL_TO_GLOBAL : Ljava/util/Map;
private static final VANILLA_UV_TRANSFORM_GLOBAL_TO_LOCAL : Ljava/util/Map;
public <init>()V
public static blockCenterToCorner(Lcom/mojang/math/Transformation;)Lcom/mojang/math/Transformation;
public static blockCornerToCenter(Lcom/mojang/math/Transformation;)Lcom/mojang/math/Transformation;
public static getFaceTransformation(Lcom/mojang/math/Transformation;Lnet/minecraft/core/Direction;)Lcom/mojang/math/Transformation;
static <clinit>()V
```
