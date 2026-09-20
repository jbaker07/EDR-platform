---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.submit.TranslucentSubmit"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.submit.TranslucentSubmit

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/renderer/feature/submit/SubmitNode`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `computeDistanceToCameraSq` | `(Lorg/joml/Matrix4fc;)F` | exact | invokestatic@7 in `ExtendedItemSubmit.distanceToCameraSq` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `computeDistanceToCameraSq` | `(Lorg/joml/Matrix4fc;FFF)F` | exact | invokestatic@13 in `ExtendedBlockModelSubmit.distanceToCameraSq` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract distanceToCameraSq()F
public abstract featureType()Lnet/minecraft/client/renderer/feature/FeatureRendererType;
public static computeDistanceToCameraSq(Lorg/joml/Matrix4fc;)F
public static computeDistanceToCameraSq(Lorg/joml/Matrix4fc;FFF)F
```
