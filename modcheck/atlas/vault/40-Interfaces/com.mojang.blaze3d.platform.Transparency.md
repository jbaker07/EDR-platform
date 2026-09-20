---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.Transparency"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.Transparency

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@89 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@107 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@125 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@146 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@164 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `hasTranslucent` | `()Z` | exact | invokevirtual@182 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `TRANSLUCENT` | `Lcom/mojang/blaze3d/platform/Transparency;` | exact | getstatic@51 in `MutableQuadView.postMaterialBake` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final hasTransparent : Z
private final hasTranslucent : Z
public static final NONE : Lcom/mojang/blaze3d/platform/Transparency;
public static final TRANSPARENT : Lcom/mojang/blaze3d/platform/Transparency;
public static final TRANSLUCENT : Lcom/mojang/blaze3d/platform/Transparency;
public static final TRANSPARENT_AND_TRANSLUCENT : Lcom/mojang/blaze3d/platform/Transparency;
public <init>(ZZ)V
public static of(ZZ)Lcom/mojang/blaze3d/platform/Transparency;
public or(Lcom/mojang/blaze3d/platform/Transparency;)Lcom/mojang/blaze3d/platform/Transparency;
public isOpaque()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public hasTransparent()Z
public hasTranslucent()Z
static <clinit>()V
```
