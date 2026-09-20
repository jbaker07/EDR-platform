---
type: "mechanism"
module: "fabric-screen-api-v1"
version: "5.2.4+48607d035d"
sha256: "6d0660544189cee8ed9a0d7a668ff69c30068799de633e4813924c0b6d3d49ed"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-screen-api-v1

**Version** `5.2.4+48607d035d` -- **artifact sha256** `6d0660544189cee8ed9a0d7a668ff69c30068799de633e4813924c0b6d3d49ed`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-screen-api-v1.mixins.json"]`
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT|ScreenEvents.AFTER_INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.BEFORE_INIT|ScreenEvents.BEFORE_INIT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.KeyboardHandler|KeyboardHandler]].`charTyped` | `(JLnet/minecraft/client/input/CharacterEvent;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;charTyped(Lnet/minecraft/client/input/CharacterEvent;)Z` (inherited_exact) | both | 1000 (default) | `KeyboardHandlerMixin.invokeCharTypedEvents` |
| [[40-Interfaces/net.minecraft.client.KeyboardHandler|KeyboardHandler]].`keyPress` | `(JILnet/minecraft/client/input/KeyEvent;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z` (exact) | both | 1000 (default) | `KeyboardHandlerMixin.invokeKeyPressedEvents` |
| [[40-Interfaces/net.minecraft.client.KeyboardHandler|KeyboardHandler]].`keyPress` | `(JILnet/minecraft/client/input/KeyEvent;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;keyReleased(Lnet/minecraft/client/input/KeyEvent;)Z` (inherited_exact) | both | 1000 (default) | `KeyboardHandlerMixin.invokeKeyReleasedEvents` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`<init>` | `(Lnet/minecraft/client/main/GameConfig;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `MinecraftMixin.onInit` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Z)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/LevelLoadingScreen;tick()V` (exact) | both | 1000 (default) | `MinecraftMixin.beforeLoadingScreenTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`doWorldLoad` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Z)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/Minecraft;renderFrame(Z)V` (exact) | both | 1000 (default) | `MinecraftMixin.afterLoadingScreenTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]].`exitWorldAndClose` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;removed()V` (exact) | both | 1000 (default) | `MinecraftMixin.onScreenRemoveBecauseStopping` |
| [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`handleAccumulatedMovement` | `()V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;mouseDragged(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z` (inherited_exact) | both | 1000 (default) | `MouseHandlerMixin.invokeMouseDragEvents` |
| [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`onButton` | `(JLnet/minecraft/client/input/MouseButtonInfo;I)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;mouseClicked(Lnet/minecraft/client/input/MouseButtonEvent;Z)Z` (inherited_exact) | both | 1000 (default) | `MouseHandlerMixin.invokeMouseClickedEvents` |
| [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`onButton` | `(JLnet/minecraft/client/input/MouseButtonInfo;I)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;mouseReleased(Lnet/minecraft/client/input/MouseButtonEvent;)Z` (inherited_exact) | both | 1000 (default) | `MouseHandlerMixin.invokeMousePressedEvents` |
| [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]].`onScroll` | `(JDD)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;mouseScrolled(DDDD)Z` (inherited_exact) | both | 1000 (default) | `MouseHandlerMixin.invokeMouseScrollEvents` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]].`extractRenderState` | `(Lnet/minecraft/client/DeltaTracker;ZZ)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/client/gui/screens/Screen;extractRenderStateWithTooltipAndSubtitles(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` (exact) | both | 1000 (default) | `GuiMixin.onExtractGui` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]].`setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `GuiMixin.checkThreadOnDev` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]].`setScreen` | `(Lnet/minecraft/client/gui/screens/Screen;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;removed()V` (exact) | both | 1000 (default) | `GuiMixin.onScreenRemove` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]].`tick` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;tick()V` (exact) | both | 1000 (default) | `GuiMixin.beforeScreenTick` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]].`tick` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;tick()V` (exact) | both | 1000 (default) | `GuiMixin.afterScreenTick` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`extractRenderStateWithTooltipAndSubtitles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` (exact) | both | 1000 (default) | `ScreenMixin.extractBackground` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`extractRenderStateWithTooltipAndSubtitles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/Screen;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` (exact) | both | 1000 (default) | `ScreenMixin.extractForeground` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`init` | `(II)V` | exact | @Inject | HEAD | both | 1000 (default) | `ScreenMixin.beforeInitScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`init` | `(II)V` | exact | @Inject | TAIL | both | 1000 (default) | `ScreenMixin.afterInitScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`resize` | `(II)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ScreenMixin.beforeResizeScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]].`resize` | `(II)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ScreenMixin.afterResizeScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]].`extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractContents(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` (exact) | both | 1000 (default) | `AbstractContainerScreenMixin.extractRenderState` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]].`mouseDragged` | `(Lnet/minecraft/client/input/MouseButtonEvent;DD)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `AbstractContainerScreenMixin.callSuperMouseReleased` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]].`mouseReleased` | `(Lnet/minecraft/client/input/MouseButtonEvent;)Z` | name_only | @Inject | HEAD | both | 1000 (default) | `AbstractContainerScreenMixin.callSuperMouseReleased` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen|AbstractRecipeBookScreen]].`extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | name_only | @Inject | INVOKE `Lnet/minecraft/client/gui/screens/recipebook/RecipeBookComponent;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` (exact) | both | 1000 (default) | `AbstractRecipeBookScreenMixin.extractRenderState` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents|ScreenEvents]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenKeyboardEvents|ScreenKeyboardEvents]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenMouseEvents|ScreenMouseEvents]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.Screens|Screens]] (class, 3 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
