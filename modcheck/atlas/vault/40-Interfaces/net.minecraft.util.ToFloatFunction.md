---
type: "interface"
fqcn: "net.minecraft.util.ToFloatFunction"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ToFloatFunction

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `applyAsFloat` | `(Ljava/lang/Object;)F` | exact | invokeinterface@3 in `SimpleConfiguredFluidBehavior$Builder.lambda$movementSlowdown$2` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `applyAsFloat` | `(Ljava/lang/Object;)F` | exact | invokeinterface@5 in `SimpleConfiguredFluidBehavior.travelInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `applyAsFloat` | `(Ljava/lang/Object;)F` | exact | invokeinterface@5 in `SimpleConfiguredFluidBehavior.travelFlyingInFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract applyAsFloat(Ljava/lang/Object;)F
```
