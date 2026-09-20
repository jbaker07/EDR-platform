---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.phase.SimpleFeatureRenderPhase

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/feature/phase/FeatureRenderPhase`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokevirtual@66 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokevirtual@112 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokevirtual@103 in `SubmitNodeCollectionMixin.submitBreakingBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokevirtual@137 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokevirtual@179 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (1 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private submitsByFeature : [Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase$FeatureSubmits;
public <init>()V
public submit(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V
public sortInto(Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase$Output;)V
private static sortFeatureInto(Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase$Output;Lnet/minecraft/client/renderer/feature/phase/SimpleFeatureRenderPhase$FeatureSubmits;)V
private static maybeShuffle(Ljava/util/Collection;)Ljava/util/Collection;
private static maybeShuffle([Ljava/lang/Object;)[Ljava/lang/Object;
public clear()V
public isEmpty()Z
```
