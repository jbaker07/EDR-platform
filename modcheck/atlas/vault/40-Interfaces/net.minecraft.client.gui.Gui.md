---
type: "interface"
fqcn: "net.minecraft.client.gui.Gui"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.Gui

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `overlay` | `()Lnet/minecraft/client/gui/screens/Overlay;` | exact | invokevirtual@11 in `MinecraftMixin.onTick` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@39 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$3` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@75 in `FabricClientGameTestRunner.lambda$setupAndCheckFinalGameTestState$3` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `ClientGameTestContextImpl.lambda$tryClickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `ClientGameTestContextImpl.lambda$clickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@35 in `ClientGameTestContextImpl.lambda$clickScreenButton$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@5 in `ClientGameTestContextImpl.lambda$waitForScreen$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `ClientGameTestContextImpl.lambda$waitForScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@21 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@13 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@32 in `ClientGameTestImpl.isWorldLoadingFinished` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `ClientGameTestImpl.lambda$waitForWorldLoad$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `ClientGameTestImpl.lambda$waitForWorldLoad$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@23 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@8 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@43 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@58 in `ClientNetworkingImpl.getLoginConnection` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `MinecraftMixin.onScreenRemoveBecauseStopping` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@20 in `MinecraftMixin.onScreenRemoveBecauseStopping` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `screen` | `()Lnet/minecraft/client/gui/screens/Screen;` | exact | invokevirtual@4 in `MinecraftMixin.beforeLoadingScreenTick` | unknown | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@13 in `ClientGameTestContextImpl.lambda$setScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@5 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@83 in `ClientCommandInternals.openSendConfirmationWindow` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@15 in `ClientCommandInternals.lambda$openSendConfirmationWindow$0` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@154 in `ClientNetworking.openScreen` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | exact | invokevirtual@11 in `DetailsScreen.onClose` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `tick` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `tick` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@7 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@58 in `HudStatusBarHeightRegistryImpl.getHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@29 in `HudStatusBarHeightRegistryImpl.getElementHeight` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@6 in `HudStatusBarHeightRegistryImpl.lambda$replaceVanillaElement$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@6 in `HudStatusBarHeightRegistryImpl.lambda$static$4` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@6 in `HudStatusBarHeightRegistryImpl.lambda$static$3` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `hud` | `Lnet/minecraft/client/gui/Hud;` | exact | getfield@16 in `HudStatusBarHeightRegistryImpl.lambda$static$1` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |
| reads | `screen` | `Lnet/minecraft/client/gui/screens/Screen;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | declared |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/DeltaTracker;ZZ)V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (15 fields, 34 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final TRACY_CURRENT_SCREEN : Lcom/mojang/jtracy/SectionCategory;
private static final SOCIAL_INTERACTIONS_NOT_AVAILABLE : Lnet/minecraft/network/chat/Component;
public static final SAVING_LEVEL : Lnet/minecraft/network/chat/Component;
private final minecraft : Lnet/minecraft/client/Minecraft;
public final hud : Lnet/minecraft/client/gui/Hud;
private final guiRenderState : Lnet/minecraft/client/renderer/state/gui/GuiRenderState;
private screenTracySection : Lcom/mojang/jtracy/Section;
private screen : Lnet/minecraft/client/gui/screens/Screen;
private overlay : Lnet/minecraft/client/gui/screens/Overlay;
private clientLevelTeardownInProgress : Z
private final splashManager : Lnet/minecraft/client/resources/SplashManager;
private final toastManager : Lnet/minecraft/client/gui/components/toasts/ToastManager;
private final chatListener : Lnet/minecraft/client/multiplayer/chat/ChatListener;
private socialInteractionsToast : Lnet/minecraft/client/gui/components/toasts/TutorialToast;
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/Hud;Lnet/minecraft/client/renderer/state/gui/GuiRenderState;)V
public registerReloadListeners(Lnet/minecraft/server/packs/resources/ReloadableResourceManager;)V
public tick()V
public update()V
public extractRenderState(Lnet/minecraft/client/DeltaTracker;ZZ)V
public screen()Lnet/minecraft/client/gui/screens/Screen;
public setScreen(Lnet/minecraft/client/gui/screens/Screen;)V
public overlay()Lnet/minecraft/client/gui/screens/Overlay;
public setOverlay(Lnet/minecraft/client/gui/screens/Overlay;)V
public isPausing()Z
public splashManager()Lnet/minecraft/client/resources/SplashManager;
public toastManager()Lnet/minecraft/client/gui/components/toasts/ToastManager;
public chatListener()Lnet/minecraft/client/multiplayer/chat/ChatListener;
public addSocialInteractionsToast()V
public setPauseScreen(ZZ)V
public handleKeybinds()V
public openChatScreen(Lnet/minecraft/client/gui/components/ChatComponent$ChatMethod;)V
public openChatAndAddText(Lnet/minecraft/client/gui/components/ChatComponent$ChatMethod;Ljava/lang/String;)V
public buildInitialScreens(Lnet/minecraft/client/GameLoadCookie;)Ljava/lang/Runnable;
private addInitialScreens(Ljava/util/List;)Z
public canInterruptScreen()Z
public setClientLevelTeardownInProgress(Z)V
private renderActiveTextDebug()V
private synthetic lambda$renderActiveTextDebug$0(Lnet/minecraft/client/renderer/state/gui/GuiTextRenderState;)V
private static synthetic lambda$addInitialScreens$3(Lcom/mojang/authlib/GameProfile;Ljava/lang/Runnable;)Lnet/minecraft/client/gui/screens/Screen;
private static synthetic lambda$addInitialScreens$1(Lcom/mojang/authlib/minecraft/BanDetails;Ljava/lang/Runnable;)Lnet/minecraft/client/gui/screens/Screen;
private static synthetic lambda$addInitialScreens$2(Ljava/lang/Runnable;Z)V
private synthetic lambda$addInitialScreens$0(Ljava/lang/Runnable;)Lnet/minecraft/client/gui/screens/Screen;
private synthetic lambda$buildInitialScreens$1(Lnet/minecraft/client/gui/screens/Screen;)V
private synthetic lambda$buildInitialScreens$0(Lnet/minecraft/client/GameLoadCookie;Z)V
private synthetic lambda$extractRenderState$1()Ljava/lang/String;
private synthetic lambda$extractRenderState$0()Ljava/lang/String;
private synthetic lambda$update$0()Ljava/lang/String;
static <clinit>()V
```
