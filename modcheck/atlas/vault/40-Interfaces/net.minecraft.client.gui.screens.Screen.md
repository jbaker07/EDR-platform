---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.Screen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.Screen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/components/events/AbstractContainerEventHandler`; implements `net/minecraft/client/gui/components/Renderable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokespecial@2 in `DetailsScreen.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokespecial@2 in `CreateWorldScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokespecial@2 in `AbstractContainerScreenMixin.<init>` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `extractBackground` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | exact | invokespecial@6 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getFont` | `()Lnet/minecraft/client/gui/Font;` | exact | invokevirtual@8 in `Screens.getFont` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getTitle` | `()Lnet/minecraft/network/chat/Component;` | exact | invokevirtual@10 in `ClientGameTestImpl.isExperimentalWarningScreen` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `mouseDragged` | `(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z` | inherited_exact | invokespecial@5 in `AbstractContainerScreenMixin.callSuperMouseReleased` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `mouseReleased` | `(Lnet/minecraft/client/input/MouseButtonEvent;)Z` | inherited_exact | invokespecial@2 in `AbstractContainerScreenMixin.callSuperMouseReleased` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `resize` | `(II)V` | exact | invokespecial@3 in `DetailsScreen.resize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `extractPanorama` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `extractRenderStateWithTooltipAndSubtitles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `extractRenderStateWithTooltipAndSubtitles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `init` | `(II)V` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `init` | `(II)V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `resize` | `(II)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `resize` | `(II)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| reads | `FOOTER_SEPARATOR` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@82 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `HEADER_SEPARATOR` | `Lnet/minecraft/resources/Identifier;` | exact | getstatic@52 in `DetailsScreen.extractBackground` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `children` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |
| reads | `narratables` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |
| reads | `renderables` | `Ljava/util/List;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |

## Declared members (30 fields, 77 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final SCREEN_USAGE_NARRATION : Lnet/minecraft/network/chat/Component;
public static final MENU_BACKGROUND : Lnet/minecraft/resources/Identifier;
public static final HEADER_SEPARATOR : Lnet/minecraft/resources/Identifier;
public static final FOOTER_SEPARATOR : Lnet/minecraft/resources/Identifier;
private static final INWORLD_MENU_BACKGROUND : Lnet/minecraft/resources/Identifier;
public static final INWORLD_HEADER_SEPARATOR : Lnet/minecraft/resources/Identifier;
public static final INWORLD_FOOTER_SEPARATOR : Lnet/minecraft/resources/Identifier;
protected static final FADE_IN_TIME : F
protected final title : Lnet/minecraft/network/chat/Component;
private final children : Ljava/util/List;
private final narratables : Ljava/util/List;
protected final minecraft : Lnet/minecraft/client/Minecraft;
private initialized : Z
public width : I
public height : I
private final renderables : Ljava/util/List;
protected final font : Lnet/minecraft/client/gui/Font;
private static final NARRATE_SUPPRESS_AFTER_INIT_TIME : J
private static final NARRATE_DELAY_NARRATOR_ENABLED : J
private static final NARRATE_DELAY_MOUSE_MOVE : J
private static final NARRATE_DELAY_MOUSE_ACTION : J
private static final NARRATE_DELAY_KEYBOARD_ACTION : J
private final narrationState : Lnet/minecraft/client/gui/narration/ScreenNarrationCollector;
private narrationSuppressTime : J
private nextNarrationTime : J
protected narratorButton : Lnet/minecraft/client/gui/components/CycleButton;
private lastNarratable : Lnet/minecraft/client/gui/narration/NarratableEntry;
private nextNarrationTriggeredBy : Lnet/minecraft/client/gui/narration/NarrationTrigger;
protected final screenExecutor : Ljava/util/concurrent/Executor;
protected <init>(Lnet/minecraft/network/chat/Component;)V
protected <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/Font;Lnet/minecraft/network/chat/Component;)V
public getTitle()Lnet/minecraft/network/chat/Component;
public getNarrationMessage()Lnet/minecraft/network/chat/Component;
public final extractRenderStateWithTooltipAndSubtitles(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
private createTabEvent(Z)Lnet/minecraft/client/gui/navigation/FocusNavigationEvent$TabNavigation;
private createArrowEvent(Lnet/minecraft/client/gui/navigation/ScreenDirection;)Lnet/minecraft/client/gui/navigation/FocusNavigationEvent$ArrowNavigation;
protected setInitialFocus()V
protected setInitialFocus(Lnet/minecraft/client/gui/components/events/GuiEventListener;)V
public clearFocus()V
public isInputCaptured()Z
protected changeFocus(Lnet/minecraft/client/gui/ComponentPath;)V
public shouldCloseOnEsc()Z
public onClose()V
protected addRenderableWidget(Lnet/minecraft/client/gui/components/events/GuiEventListener;)Lnet/minecraft/client/gui/components/events/GuiEventListener;
protected addRenderableOnly(Lnet/minecraft/client/gui/components/Renderable;)Lnet/minecraft/client/gui/components/Renderable;
protected addWidget(Lnet/minecraft/client/gui/components/events/GuiEventListener;)Lnet/minecraft/client/gui/components/events/GuiEventListener;
protected removeWidget(Lnet/minecraft/client/gui/components/events/GuiEventListener;)V
protected clearWidgets()V
public static getTooltipFromItem(Lnet/minecraft/client/Minecraft;Lnet/minecraft/world/item/ItemStack;)Ljava/util/List;
protected insertText(Ljava/lang/String;Z)V
protected static defaultHandleGameClickEvent(Lnet/minecraft/network/chat/ClickEvent;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/Screen;)V
protected static defaultHandleClickEvent(Lnet/minecraft/network/chat/ClickEvent;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/Screen;)V
protected static clickUrlAction(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/Screen;Ljava/net/URI;)Z
protected static clickCommandAction(Lnet/minecraft/client/player/LocalPlayer;Ljava/lang/String;Lnet/minecraft/client/gui/screens/Screen;)V
public final init(II)V
protected rebuildWidgets()V
protected fadeWidgets(F)V
public children()Ljava/util/List;
protected init()V
public tick()V
public removed()V
public added()V
public extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected extractBlurredBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
protected extractPanorama(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V
protected extractMenuBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
protected extractMenuBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIII)V
public static extractMenuBackgroundTexture(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/resources/Identifier;IIFFII)V
public extractTransparentBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
public isPauseScreen()Z
public isInGameUi()Z
public isAllowedInPortal()Z
protected repositionElements()V
public resize(II)V
protected isValidCharacterForName(Ljava/lang/String;II)Z
public isMouseOver(DD)Z
public onFilesDrop(Ljava/util/List;)V
private scheduleNarration(JZLnet/minecraft/client/gui/narration/NarrationTrigger;)V
private suppressNarration(J)V
private setNarrationSuppressTime(J)V
public scheduleNarration()V
public afterMouseMove()V
public afterMouseAction()V
public afterKeyboardAction()V
private shouldRunNarration()Z
public handleDelayedNarration()V
public triggerImmediateNarration(Z)V
private runNarration(ZLnet/minecraft/client/gui/narration/NarrationTrigger;)V
protected shouldNarrateNavigation()Z
protected updateNarrationState(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
protected updateNarratedWidget(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
protected getUsageNarration()Lnet/minecraft/network/chat/Component;
public static findNarratableWidget(Ljava/util/List;Lnet/minecraft/client/gui/narration/NarratableEntry;)Lnet/minecraft/client/gui/screens/Screen$NarratableSearchResult;
public updateNarratorStatus(ZLnet/minecraft/client/gui/narration/NarrationTrigger;)V
public getFont()Lnet/minecraft/client/gui/Font;
public showsActiveEffects()Z
public canInterruptWithAnotherScreen()Z
public getRectangle()Lnet/minecraft/client/gui/navigation/ScreenRectangle;
public getBackgroundMusic()Lnet/minecraft/sounds/Music;
private static synthetic lambda$updateNarratedWidget$0(Lnet/minecraft/client/gui/narration/NarratableEntry;)Ljava/util/stream/Stream;
private static synthetic lambda$clickUrlAction$0(Ljava/net/URI;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/Screen;Z)V
private synthetic lambda$new$0(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V
private synthetic lambda$new$1(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V
static <clinit>()V
```
