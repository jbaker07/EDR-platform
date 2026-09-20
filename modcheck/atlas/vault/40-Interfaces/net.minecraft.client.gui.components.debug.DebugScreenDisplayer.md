---
type: "interface"
fqcn: "net.minecraft.client.gui.components.debug.DebugScreenDisplayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.debug.DebugScreenDisplayer

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addLine` | `(Ljava/lang/String;)V` | exact | invokeinterface@17 in `DebugOverlayClient$ActiveRendererDebugOverlayEntry.display` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (0 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract addPriorityLine(Ljava/lang/String;)V
public abstract addLine(Ljava/lang/String;)V
public abstract addToGroup(Lnet/minecraft/resources/Identifier;Ljava/util/Collection;)V
public abstract addToGroup(Lnet/minecraft/resources/Identifier;Ljava/lang/String;)V
```
