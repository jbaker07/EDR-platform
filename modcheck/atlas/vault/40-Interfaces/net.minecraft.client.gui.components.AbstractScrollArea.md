---
type: "interface"
fqcn: "net.minecraft.client.gui.components.AbstractScrollArea"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.AbstractScrollArea

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/components/AbstractWidget`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `defaultSettings` | `(I)Lnet/minecraft/client/gui/components/AbstractScrollArea$ScrollbarSe` | exact | invokestatic@30 in `OptimizedScrollableLayout.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (7 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SCROLLBAR_WIDTH : I
private static final SCROLLBAR_MIN_HEIGHT : I
private static final SCROLLER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final SCROLLER_BACKGROUND_SPRITE : Lnet/minecraft/resources/Identifier;
private final scrollbarSettings : Lnet/minecraft/client/gui/components/AbstractScrollArea$ScrollbarSettings;
private scrollAmount : D
private scrolling : Z
public <init>(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/components/AbstractScrollArea$ScrollbarSettings;)V
public mouseScrolled(DDDD)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public onRelease(Lnet/minecraft/client/input/MouseButtonEvent;)V
public scrollAmount()D
public setScrollAmount(D)V
public updateScrolling(Lnet/minecraft/client/input/MouseButtonEvent;)Z
protected isOverScrollbar(DD)Z
public refreshScrollAmount()V
public maxScrollAmount()I
protected scrollable()Z
public scrollbarWidth()I
protected scrollerHeight()I
protected scrollBarX()I
public scrollBarY()I
protected extractScrollbar(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
protected abstract contentHeight()I
protected scrollRate()D
public static defaultSettings(I)Lnet/minecraft/client/gui/components/AbstractScrollArea$ScrollbarSettings;
static <clinit>()V
```
