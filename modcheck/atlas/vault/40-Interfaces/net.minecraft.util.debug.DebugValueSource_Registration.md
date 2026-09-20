---
type: "interface"
fqcn: "net.minecraft.util.debug.DebugValueSource$Registration"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.debug.DebugValueSource$Registration

System: [[20-Systems/net.minecraft.util.debug|net.minecraft.util.debug]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `register` | `(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/util/debug` | exact | invokeinterface@56 in `EntityDebugSubscriptionRegistryImpl.addDebugValues` | unknown | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract register(Lnet/minecraft/util/debug/DebugSubscription;Lnet/minecraft/util/debug/DebugValueSource$ValueGetter;)V
```
