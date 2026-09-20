---
type: "interface"
fqcn: "net.minecraft.world.MenuProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.MenuProvider

System: [[20-Systems/net.minecraft.world|net.minecraft.world]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/inventory/MenuConstructor`, `net/fabricmc/fabric/api/menu/v1/FabricMenuProvider`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `shouldCloseCurrentScreen` | `()Z` | inherited_exact | invokeinterface@1 in `ServerPlayerMixin.fabric_closeContainerScreenIfAllowed` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getDisplayName()Lnet/minecraft/network/chat/Component;
```
