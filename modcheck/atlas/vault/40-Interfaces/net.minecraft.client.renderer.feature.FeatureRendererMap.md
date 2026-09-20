---
type: "interface"
fqcn: "net.minecraft.client.renderer.feature.FeatureRendererMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.feature.FeatureRendererMap

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `put` | `(Lnet/minecraft/client/renderer/feature/FeatureRendererType;Lnet/minec` | exact | invokevirtual@17 in `FeatureRendererRegistryImpl$FeatureRendererRegistration.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private renderers : [Lnet/minecraft/client/renderer/feature/FeatureRenderer;
public <init>()V
public put(Lnet/minecraft/client/renderer/feature/FeatureRendererType;Lnet/minecraft/client/renderer/feature/FeatureRenderer;)V
public get(Lnet/minecraft/client/renderer/feature/FeatureRendererType;)Lnet/minecraft/client/renderer/feature/FeatureRenderer;
public getOrThrow(Lnet/minecraft/client/renderer/feature/FeatureRendererType;)Lnet/minecraft/client/renderer/feature/FeatureRenderer;
public values()Ljava/lang/Iterable;
public close()V
```
