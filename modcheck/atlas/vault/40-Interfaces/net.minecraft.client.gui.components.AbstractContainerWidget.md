---
type: "interface"
fqcn: "net.minecraft.client.gui.components.AbstractContainerWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.AbstractContainerWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/components/AbstractScrollArea`; implements `net/minecraft/client/gui/components/events/ContainerEventHandler`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/c` | exact | invokespecial@21 in `OptimizedScrollableLayout$Container.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setFocused` | `(Lnet/minecraft/client/gui/components/events/GuiEventListener;)V` | exact | invokespecial@2 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setScrollAmount` | `(D)V` | inherited_exact | invokespecial@2 in `OptimizedScrollableLayout$Container.setScrollAmount` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setX` | `(I)V` | inherited_exact | invokespecial@2 in `OptimizedScrollableLayout$Container.setX` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `setY` | `(I)V` | inherited_exact | invokespecial@2 in `OptimizedScrollableLayout$Container.setY` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (2 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private focused : Lnet/minecraft/client/gui/components/events/GuiEventListener;
private isDragging : Z
public <init>(IIIILnet/minecraft/network/chat/Component;)V
public <init>(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/components/AbstractScrollArea$ScrollbarSettings;)V
public final isDragging()Z
public final setDragging(Z)V
public getFocused()Lnet/minecraft/client/gui/components/events/GuiEventListener;
public setFocused(Lnet/minecraft/client/gui/components/events/GuiEventListener;)V
public nextFocusPath(Lnet/minecraft/client/gui/navigation/FocusNavigationEvent;)Lnet/minecraft/client/gui/ComponentPath;
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public isFocused()Z
public setFocused(Z)V
```
