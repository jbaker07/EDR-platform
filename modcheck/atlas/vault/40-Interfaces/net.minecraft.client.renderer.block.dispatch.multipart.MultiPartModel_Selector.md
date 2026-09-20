---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.dispatch.multipart.MultiPartModel$Selector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `model` | `()Ljava/lang/Object;` | exact | invokevirtual@20 in `MultiPartModelMixin.particleMaterial` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final condition : Ljava/util/function/Predicate;
private final model : Ljava/lang/Object;
public <init>(Ljava/util/function/Predicate;Ljava/lang/Object;)V
public with(Ljava/lang/Object;)Lnet/minecraft/client/renderer/block/dispatch/multipart/MultiPartModel$Selector;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public condition()Ljava/util/function/Predicate;
public model()Ljava/lang/Object;
```
