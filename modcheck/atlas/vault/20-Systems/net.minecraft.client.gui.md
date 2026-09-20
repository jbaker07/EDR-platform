---
type: "system"
package: "net.minecraft.client.gui"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui

Analyst note: [[_authored/systems/net.minecraft.client.gui|GUI: screens, HUD and widgets]]

794 classes in the jar. Hooked types: 53

- [[40-Interfaces/net.minecraft.client.gui.Font|Font]] -- calls:3 -- by fabric-game-rule-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] -- calls:14, injects_into:4, reads:1 -- by fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-menu-api-v1, fabric-networking-api-v1, fabric-registry-sync-v0, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]] -- calls:6, injects_into:1 -- by fabric-registry-sync-v0, fabric-rendering-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.Hud|Hud]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.gui.components.AbstractScrollArea|AbstractScrollArea]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.Button|Button]] -- calls:9 -- by fabric-client-gametest-api-v1, fabric-game-rule-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.Button_Builder|Button$Builder]] -- calls:6 -- by fabric-game-rule-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.ChatComponent|ChatComponent]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.gui.components.CycleButton|CycleButton]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.EditBox|EditBox]] -- calls:7 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget|FocusableTextWidget]] -- calls:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget_Builder|FocusableTextWidget$Builder]] -- calls:4 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.MultiLineLabel|MultiLineLabel]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.MultiLineTextWidget|MultiLineTextWidget]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.components.StringWidget|StringWidget]] -- calls:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenEntries|DebugScreenEntries]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.gui.layouts.HeaderAndFooterLayout|HeaderAndFooterLayout]] -- calls:8 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.layouts.Layout|Layout]] -- calls:3 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.layouts.LayoutElement|LayoutElement]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.layouts.LinearLayout|LinearLayout]] -- calls:4 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.navigation.ScreenRectangle|ScreenRectangle]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] -- injects_into:5 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBannerResultRenderer|GuiBannerResultRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBookModelRenderer|GuiBookModelRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiEntityRenderer|GuiEntityRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiProfilerChartRenderer|GuiProfilerChartRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiSkinRenderer|GuiSkinRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.PictureInPictureRenderer|PictureInPictureRenderer]] -- calls:3 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.BackupConfirmScreen|BackupConfirmScreen]] -- calls:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.screens.ConfirmScreen|ConfirmScreen]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.gui.screens.ConnectScreen|ConnectScreen]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.GenericMessageScreen|GenericMessageScreen]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens|MenuScreens]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens_ScreenConstructor|MenuScreens$ScreenConstructor]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] -- calls:8, injects_into:7 -- by fabric-client-gametest-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTab|AdvancementTab]] -- calls:2, injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementWidget|AdvancementWidget]] -- calls:1, injects_into:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.debug.DebugOptionsScreen_OptionList|DebugOptionsScreen$OptionList]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]] -- calls:2, injects_into:3 -- by fabric-creative-tab-api-v1, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen|AbstractRecipeBookScreen]] -- injects_into:1 -- by fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen|AbstractSignEditScreen]] -- calls:2 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] -- injects_into:6 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.MenuAccess|MenuAccess]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]] -- injects_into:2 -- by fabric-rendering-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]] -- injects_into:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList|TransferableSelectionList]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList_PackEntry|TransferableSelectionList$PackEntry]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen_GameRuleEntry|AbstractGameRulesScreen$GameRuleEntry]] -- calls:2 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] -- calls:2, injects_into:3, wraps:1 -- by fabric-client-gametest-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationContext|WorldCreationContext]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationUiState|WorldCreationUiState]] -- calls:6 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationUiState_WorldTypeEntry|WorldCreationUiState$WorldTypeEntry]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldOpenFlows|WorldOpenFlows]] -- calls:1 -- by fabric-client-gametest-api-v1
