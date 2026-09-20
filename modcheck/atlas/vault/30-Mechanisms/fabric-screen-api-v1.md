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

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT|ScreenEvents.AFTER_INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.BEFORE_INIT|ScreenEvents.BEFORE_INIT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `<init>` | injects_into `@Inject at RETURN` | both | `MinecraftMixin.onInit` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `doWorldLoad` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/LevelLoadingScreen;tick()V` | both | `MinecraftMixin.beforeLoadingScreenTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `doWorldLoad` | injects_into `@Inject at INVOKE Lnet/minecraft/client/Minecraft;renderFrame(Z)V` | both | `MinecraftMixin.afterLoadingScreenTick` |
| [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] | `exitWorldAndClose` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;removed()V` | both | `MinecraftMixin.onScreenRemoveBecauseStopping` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] | `setScreen` | injects_into `@Inject at HEAD` | both | `GuiMixin.checkThreadOnDev` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] | `setScreen` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;removed()V` | both | `GuiMixin.onScreenRemove` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] | `tick` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;tick()V` | both | `GuiMixin.beforeScreenTick` |
| [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] | `tick` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;tick()V` | both | `GuiMixin.afterScreenTick` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `extractRenderStateWithTooltipAndSubtitles` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;extractBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | both | `ScreenMixin.extractBackground` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `extractRenderStateWithTooltipAndSubtitles` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | both | `ScreenMixin.extractForeground` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `init(II)V` | injects_into `@Inject at HEAD` | both | `ScreenMixin.beforeInitScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `init(II)V` | injects_into `@Inject at TAIL` | both | `ScreenMixin.afterInitScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `resize` | injects_into `@Inject at HEAD` | both | `ScreenMixin.beforeResizeScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] | `resize` | injects_into `@Inject at TAIL` | both | `ScreenMixin.afterResizeScreen` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]] | `extractRenderState` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/inventory/AbstractContainerScreen;extractContents(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | both | `AbstractContainerScreenMixin.extractRenderState` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]] | `mouseDragged` | injects_into `@Inject at HEAD` | both | `AbstractContainerScreenMixin.callSuperMouseReleased` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]] | `mouseReleased` | injects_into `@Inject at HEAD` | both | `AbstractContainerScreenMixin.callSuperMouseReleased` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen|AbstractRecipeBookScreen]] | `extractRenderState` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/screens/recipebook/RecipeBookComponent;extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | both | `AbstractRecipeBookScreenMixin.extractRenderState` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents|ScreenEvents]] (class, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenKeyboardEvents|ScreenKeyboardEvents]] (class, 9 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.ScreenMouseEvents|ScreenMouseEvents]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.screen.v1.Screens|Screens]] (class, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
