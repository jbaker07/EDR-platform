---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.MovingBlockFeatureRenderer$Submit

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/renderer/feature/submit/TranslucentSubmit`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `forceTranslucent` | `()Z` | exact | invokevirtual@17 in `MovingBlockFeatureRendererMixin.tesselateBlockProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `outlineColor` | `()I` | exact | invokevirtual@12 in `MovingBlockFeatureRendererMixin.tesselateBlockProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final pose : Lorg/joml/Matrix4fc;
private final movingBlockRenderState : Lnet/minecraft/client/renderer/block/MovingBlockRenderState;
private final outlineColor : I
private final forceTranslucent : Z
public <init>(Lorg/joml/Matrix4fc;Lnet/minecraft/client/renderer/block/MovingBlockRenderState;IZ)V
public distanceToCameraSq()F
public featureType()Lnet/minecraft/client/renderer/feature/FeatureRendererType;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public pose()Lorg/joml/Matrix4fc;
public movingBlockRenderState()Lnet/minecraft/client/renderer/block/MovingBlockRenderState;
public outlineColor()I
public forceTranslucent()Z
```
