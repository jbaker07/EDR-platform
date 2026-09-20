---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.advancements.AdvancementsScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.advancements.AdvancementsScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements `net/minecraft/client/multiplayer/ClientAdvancements$Listener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `selectedTab` | `Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractWindow` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (27 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final WINDOW_LOCATION : Lnet/minecraft/resources/Identifier;
public static final WINDOW_WIDTH : I
public static final WINDOW_HEIGHT : I
private static final WINDOW_INSIDE_X : I
private static final WINDOW_INSIDE_Y : I
public static final WINDOW_INSIDE_WIDTH : I
public static final WINDOW_INSIDE_HEIGHT : I
private static final WINDOW_TITLE_X : I
private static final WINDOW_TITLE_Y : I
private static final BACKGROUND_TEXTURE_WIDTH : I
private static final BACKGROUND_TEXTURE_HEIGHT : I
public static final BACKGROUND_TILE_WIDTH : I
public static final BACKGROUND_TILE_HEIGHT : I
public static final BACKGROUND_TILE_COUNT_X : I
public static final BACKGROUND_TILE_COUNT_Y : I
private static final SCROLL_SPEED : D
private static final VERY_SAD_LABEL : Lnet/minecraft/network/chat/Component;
private static final NO_ADVANCEMENTS_LABEL : Lnet/minecraft/network/chat/Component;
private static final TITLE : Lnet/minecraft/network/chat/Component;
private final layout : Lnet/minecraft/client/gui/layouts/HeaderAndFooterLayout;
private final lastScreen : Lnet/minecraft/client/gui/screens/Screen;
private leftPos : I
private topPos : I
private final advancements : Lnet/minecraft/client/multiplayer/ClientAdvancements;
private final tabs : Ljava/util/Map;
private selectedTab : Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;
private isScrolling : Z
public <init>(Lnet/minecraft/client/multiplayer/ClientAdvancements;)V
public <init>(Lnet/minecraft/client/multiplayer/ClientAdvancements;Lnet/minecraft/client/gui/screens/Screen;)V
protected init()V
protected repositionElements()V
public onClose()V
public removed()V
public tick()V
public mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z
public mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z
public mouseScrolled(DDDD)Z
private extractInside(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
public extractWindow(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
private extractTooltips(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
public onAdvancementsUpdated()V
public onSelectedTabChanged(Lnet/minecraft/advancements/AdvancementHolder;)V
public onAdvancementsCleared()V
public getAdvancementWidget(Lnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/client/gui/screens/advancements/AdvancementWidget;
private getTab(Lnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/client/gui/screens/advancements/AdvancementTab;
private synthetic lambda$onAdvancementsUpdated$0(Lnet/minecraft/advancements/AdvancementTree;Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/advancements/AdvancementProgress;)V
private static synthetic lambda$init$1(Lnet/minecraft/client/gui/screens/advancements/AdvancementsScreen;Lnet/minecraft/client/gui/components/AbstractWidget;)V
private synthetic lambda$init$0(Lnet/minecraft/client/gui/components/Button;)V
static <clinit>()V
```
