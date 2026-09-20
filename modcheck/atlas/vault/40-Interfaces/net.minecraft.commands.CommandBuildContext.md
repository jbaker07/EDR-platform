---
type: "interface"
fqcn: "net.minecraft.commands.CommandBuildContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.CommandBuildContext

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderLookup$Provider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `simple` | `(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/flag/F` | exact | invokestatic@30 in `ClientPacketListenerMixin.onGameJoin` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (0 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static simple(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/commands/CommandBuildContext;
public abstract enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
```
