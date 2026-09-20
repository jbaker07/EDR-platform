---
type: "interface"
fqcn: "net.minecraft.client.gui.layouts.Layout"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.layouts.Layout

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/gui/layouts/LayoutElement`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `arrangeElements` | `()V` | exact | invokeinterface@4 in `OptimizedScrollableLayout.arrangeElements` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeight` | `()I` | inherited_exact | invokeinterface@7 in `OptimizedScrollableLayout$Container.contentHeight` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getWidth` | `()I` | inherited_exact | invokeinterface@17 in `OptimizedScrollableLayout.arrangeElements` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `removeChildren` | `()V` | exact | invokeinterface@16 in `OptimizedScrollableLayout.removeChildren` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setX` | `(I)V` | inherited_exact | invokeinterface@13 in `OptimizedScrollableLayout$Container.setX` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setY` | `(I)V` | inherited_exact | invokeinterface@19 in `OptimizedScrollableLayout$Container.setY` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setY` | `(I)V` | inherited_exact | invokeinterface@25 in `OptimizedScrollableLayout$Container.setScrollAmount` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `visitWidgets` | `(Ljava/util/function/Consumer;)V` | exact | invokeinterface@30 in `OptimizedScrollableLayout$Container.refreshChildren` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract visitChildren(Ljava/util/function/Consumer;)V
public visitWidgets(Ljava/util/function/Consumer;)V
public arrangeElements()V
public abstract removeChildren()V
private static synthetic lambda$arrangeElements$0(Lnet/minecraft/client/gui/layouts/LayoutElement;)V
private static synthetic lambda$visitWidgets$0(Ljava/util/function/Consumer;Lnet/minecraft/client/gui/layouts/LayoutElement;)V
```
