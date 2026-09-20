---
type: "interface"
fqcn: "net.minecraft.client.gui.components.events.GuiEventListener"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.events.GuiEventListener

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/gui/components/TabOrderedElement`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBorderForArrowNavigation` | `(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/` | exact | invokeinterface@11 in `OptimizedScrollableLayout$Container.getBorderForArrowNavigation` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getRectangle` | `()Lnet/minecraft/client/gui/navigation/ScreenRectangle;` | exact | invokeinterface@28 in `OptimizedScrollableLayout$Container.setFocused` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (0 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public mouseMoved(DD)V
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public mouseScrolled(DDDD)Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public keyReleased(Lnet/minecraft/client/input/KeyEvent;)Z
public charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z
public preeditUpdated(Lnet/minecraft/client/input/PreeditEvent;)Z
public nextFocusPath(Lnet/minecraft/client/gui/navigation/FocusNavigationEvent;)Lnet/minecraft/client/gui/ComponentPath;
public isMouseOver(DD)Z
public abstract setFocused(Z)V
public abstract isFocused()Z
public shouldTakeFocusAfterInteraction()Z
public capturesInput()Z
public getCurrentFocusPath()Lnet/minecraft/client/gui/ComponentPath;
public getRectangle()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public getBorderForArrowNavigation(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/client/gui/navigation/ScreenRectangle;
```
