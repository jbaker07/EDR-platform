---
type: "interface"
fqcn: "net.minecraft.resources.DependantName"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.DependantName

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `fixed` | `(Ljava/lang/Object;)Lnet/minecraft/resources/DependantName;` | exact | invokestatic@2 in `ItemPropertiesMixin.modelId` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract get(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;
public static fixed(Ljava/lang/Object;)Lnet/minecraft/resources/DependantName;
private static synthetic lambda$fixed$0(Ljava/lang/Object;Lnet/minecraft/resources/ResourceKey;)Ljava/lang/Object;
```
