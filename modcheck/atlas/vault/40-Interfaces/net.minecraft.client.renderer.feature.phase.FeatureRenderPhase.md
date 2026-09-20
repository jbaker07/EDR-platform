---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.phase.FeatureRenderPhase"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.phase.FeatureRenderPhase

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokeinterface@52 in `SubmitNodeCollectionMixin.submitBlockModel` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokeinterface@74 in `SubmitNodeCollectionMixin.submitItem` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submit` | `(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V` | exact | invokeinterface@14 in `SubmitRenderPhase.submit` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract submit(Lnet/minecraft/client/renderer/feature/submit/SubmitNode;)V
public abstract sortInto(Lnet/minecraft/client/renderer/feature/phase/FeatureRenderPhase$Output;)V
public abstract isEmpty()Z
```
