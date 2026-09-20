---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ResolvableModel$Resolver"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ResolvableModel$Resolver

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `markDependency` | `(Lnet/minecraft/resources/Identifier;)V` | exact | invokeinterface@5 in `SimpleUnbakedExtraModel.resolveDependencies` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract markDependency(Lnet/minecraft/resources/Identifier;)V
```
