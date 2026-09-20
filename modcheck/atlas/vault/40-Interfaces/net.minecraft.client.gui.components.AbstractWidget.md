---
type: "interface"
fqcn: "net.minecraft.client.gui.components.AbstractWidget"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.AbstractWidget

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `java/lang/Object`; implements `net/minecraft/client/gui/layouts/LayoutElement`, `net/minecraft/client/gui/components/Renderable`, `net/minecraft/client/gui/components/events/GuiEventListener`, `net/minecraft/client/gui/narration/NarratableEntry`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | exact | invokevirtual@108 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@70 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@65 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@83 in `OptimizedScrollableLayout$Container.extractWidgetRenderState` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (12 fields, 51 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected width : I
protected height : I
private x : I
private y : I
protected message : Lnet/minecraft/network/chat/Component;
protected isHovered : Z
public active : Z
public visible : Z
protected alpha : F
private tabOrderGroup : I
private focused : Z
private final tooltip : Lnet/minecraft/client/gui/components/WidgetTooltipHolder;
public <init>(IIIILnet/minecraft/network/chat/Component;)V
public getHeight()I
public final extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected extractTooltipForNextRenderPass(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
protected handleCursor(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
public setTooltip(Lnet/minecraft/client/gui/components/Tooltip;)V
public setTooltipDelay(Ljava/time/Duration;)V
protected createNarrationMessage()Lnet/minecraft/network/chat/MutableComponent;
public static wrapDefaultNarrationMessage(Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/MutableComponent;
protected abstract extractWidgetRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected extractScrollingStringOverContents(Lnet/minecraft/client/gui/ActiveTextCollector;Lnet/minecraft/network/chat/Component;I)V
public onClick(Lnet/minecraft/client/input/MouseButtonEvent;Z)V
public onRelease(Lnet/minecraft/client/input/MouseButtonEvent;)V
protected onDrag(Lnet/minecraft/client/input/MouseButtonEvent;DD)V
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
protected isValidClickButton(Lnet/minecraft/client/input/MouseButtonInfo;)Z
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public nextFocusPath(Lnet/minecraft/client/gui/navigation/FocusNavigationEvent;)Lnet/minecraft/client/gui/ComponentPath;
public isMouseOver(DD)Z
public playDownSound(Lnet/minecraft/client/sounds/SoundManager;)V
public static playButtonClickSound(Lnet/minecraft/client/sounds/SoundManager;)V
public getWidth()I
public setWidth(I)V
public setHeight(I)V
public setAlpha(F)V
public getAlpha()F
public setMessage(Lnet/minecraft/network/chat/Component;)V
public getMessage()Lnet/minecraft/network/chat/Component;
public isFocused()Z
public isHovered()Z
public isHoveredOrFocused()Z
public isActive()Z
public setFocused(Z)V
public narrationPriority()Lnet/minecraft/client/gui/narration/NarratableEntry$NarrationPriority;
public final updateNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
protected abstract updateWidgetNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
protected defaultButtonNarrationText(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
public getX()I
public setX(I)V
public getY()I
public setY(I)V
public getRight()I
public getBottom()I
public visitWidgets(Ljava/util/function/Consumer;)V
public setSize(II)V
public getRectangle()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
private areCoordinatesInRectangle(DD)Z
public setRectangle(IIII)V
public getTabOrderGroup()I
public setTabOrderGroup(I)V
```
