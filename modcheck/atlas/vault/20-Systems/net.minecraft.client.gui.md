---
type: "system"
package: "net.minecraft.client.gui"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui

Analyst note: [[_authored/systems/net.minecraft.client.gui|GUI: screens, HUD and widgets]]

794 classes (445 top-level) across 38 packages in the processed jar; 9 changed by Loom processing; 67 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.client.gui.Font|Font]] -- calls:3 -- by fabric-game-rule-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]] -- calls:27, injects_into:4, reads:9, wraps:1 -- by fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-menu-api-v1, fabric-networking-api-v1, fabric-registry-sync-v0, fabric-rendering-v1, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]] -- calls:14, injects_into:1 -- by fabric-creative-tab-api-v1, fabric-registry-sync-v0, fabric-rendering-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.Hud|Hud]] -- calls:1, injects_into:1, reads:1, wraps:22 -- by fabric-command-api-v2, fabric-content-registries-v0, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.components.AbstractContainerWidget|AbstractContainerWidget]] -- calls:5 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.AbstractScrollArea|AbstractScrollArea]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.AbstractWidget|AbstractWidget]] -- calls:4 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.Button|Button]] -- calls:10, reads:1 -- by fabric-client-gametest-api-v1, fabric-creative-tab-api-v1, fabric-game-rule-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.Button_Builder|Button$Builder]] -- calls:6 -- by fabric-game-rule-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.ChatComponent|ChatComponent]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.gui.components.CycleButton|CycleButton]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.EditBox|EditBox]] -- calls:8 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget|FocusableTextWidget]] -- calls:4 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget_BackgroundFill|FocusableTextWidget$BackgroundFill]] -- reads:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget_Builder|FocusableTextWidget$Builder]] -- calls:8 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.MultiLineLabel|MultiLineLabel]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.MultiLineTextWidget|MultiLineTextWidget]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.components.StringWidget|StringWidget]] -- calls:3 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.components.SubtitleOverlay|SubtitleOverlay]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugEntryCategory|DebugEntryCategory]] -- reads:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenDisplayer|DebugScreenDisplayer]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenEntries|DebugScreenEntries]] -- calls:1 -- by fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.gui.components.events.GuiEventListener|GuiEventListener]] -- calls:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.components.toasts.AdvancementToast|AdvancementToast]] -- reads:1, wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.layouts.HeaderAndFooterLayout|HeaderAndFooterLayout]] -- calls:12 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.layouts.Layout|Layout]] -- calls:8 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.layouts.LayoutElement|LayoutElement]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.layouts.LinearLayout|LinearLayout]] -- calls:5 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.navigation.ScreenRectangle|ScreenRectangle]] -- calls:8 -- by fabric-registry-sync-v0, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]] -- injects_into:6, reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBannerResultRenderer|GuiBannerResultRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBookModelRenderer|GuiBookModelRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiEntityRenderer|GuiEntityRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiProfilerChartRenderer|GuiProfilerChartRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiSkinRenderer|GuiSkinRenderer]] -- calls:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.render.pip.PictureInPictureRenderer|PictureInPictureRenderer]] -- calls:4 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.BackupConfirmScreen|BackupConfirmScreen]] -- calls:2 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.gui.screens.ConfirmScreen|ConfirmScreen]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.gui.screens.ConnectScreen|ConnectScreen]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.GenericMessageScreen|GenericMessageScreen]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens|MenuScreens]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens_ScreenConstructor|MenuScreens$ScreenConstructor]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]] -- calls:9, injects_into:7, reads:5 -- by fabric-client-gametest-api-v1, fabric-registry-sync-v0, fabric-resource-loader-v1, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTab|AdvancementTab]] -- calls:2, injects_into:2, reads:3, wraps:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTabType|AdvancementTabType]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementWidget|AdvancementWidget]] -- calls:2, injects_into:1, reads:2, wraps:8 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementsScreen|AdvancementsScreen]] -- reads:1, wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.debug.DebugOptionsScreen_OptionEntry|DebugOptionsScreen$OptionEntry]] -- wraps:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.debug.DebugOptionsScreen_OptionList|DebugOptionsScreen$OptionList]] -- wraps:2 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]] -- calls:2, injects_into:3 -- by fabric-creative-tab-api-v1, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen|AbstractRecipeBookScreen]] -- injects_into:1 -- by fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen|AbstractSignEditScreen]] -- calls:2 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] -- calls:6, injects_into:6, reads:1 -- by fabric-creative-tab-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.HangingSignEditScreen|HangingSignEditScreen]] -- wraps:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.MenuAccess|MenuAccess]] -- calls:1 -- by fabric-menu-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.SignEditScreen|SignEditScreen]] -- wraps:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]] -- injects_into:2 -- by fabric-rendering-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]] -- injects_into:2, reads:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList|TransferableSelectionList]] -- calls:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList_PackEntry|TransferableSelectionList$PackEntry]] -- injects_into:1, reads:5 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen_GameRuleEntry|AbstractGameRulesScreen$GameRuleEntry]] -- calls:2 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen_RuleList_1|AbstractGameRulesScreen$RuleList$1]] -- calls:1, reads:1, wraps:1 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]] -- calls:2, injects_into:3, reads:1, wraps:1 -- by fabric-client-gametest-api-v1, fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationContext|WorldCreationContext]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationUiState|WorldCreationUiState]] -- calls:9 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationUiState_WorldTypeEntry|WorldCreationUiState$WorldTypeEntry]] -- calls:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldOpenFlows|WorldOpenFlows]] -- calls:1, injects_into:1, wraps:1 -- by fabric-client-gametest-api-v1, fabric-registry-sync-v0

## Declared inventory

### `net.minecraft.client.gui` (11 top-level)

`ActiveTextCollector`, `BundleMouseActions`, `ComponentPath`, [[40-Interfaces/net.minecraft.client.gui.Font|Font]], `GlyphSource`, [[40-Interfaces/net.minecraft.client.gui.Gui|Gui]], [[40-Interfaces/net.minecraft.client.gui.GuiGraphicsExtractor|GuiGraphicsExtractor]], [[40-Interfaces/net.minecraft.client.gui.Hud|Hud]], `ItemSlotMouseAction`, `TextAlignment`, `package-info`

### `net.minecraft.client.gui.components` (60 top-level)

`AbstractButton`, [[40-Interfaces/net.minecraft.client.gui.components.AbstractContainerWidget|AbstractContainerWidget]], `AbstractOptionSliderButton`, [[40-Interfaces/net.minecraft.client.gui.components.AbstractScrollArea|AbstractScrollArea]], `AbstractSelectionList`, `AbstractSliderButton`, `AbstractStringWidget`, `AbstractTextAreaWidget`, [[40-Interfaces/net.minecraft.client.gui.components.AbstractWidget|AbstractWidget]], `BossHealthOverlay`, [[40-Interfaces/net.minecraft.client.gui.components.Button|Button]], [[40-Interfaces/net.minecraft.client.gui.components.ChatComponent|ChatComponent]], `Checkbox`, `CommandSuggestions`, `CommonButtons`, `ComponentRenderUtils`, `ContainerObjectSelectionList`, [[40-Interfaces/net.minecraft.client.gui.components.CycleButton|CycleButton]], `DebugScreenOverlay`, [[40-Interfaces/net.minecraft.client.gui.components.EditBox|EditBox]], `FittingMultiLineTextWidget`, [[40-Interfaces/net.minecraft.client.gui.components.FocusableTextWidget|FocusableTextWidget]], `FriendsButton`, `IMEPreeditOverlay`, `ImageButton`, `ImageWidget`, `ItemDisplayWidget`, `LerpingBossEvent`, `LoadingDotsWidget`, `LockIconButton`, `LogoRenderer`, `MultiLineEditBox`, [[40-Interfaces/net.minecraft.client.gui.components.MultiLineLabel|MultiLineLabel]], [[40-Interfaces/net.minecraft.client.gui.components.MultiLineTextWidget|MultiLineTextWidget]], `MultilineTextField`, `ObjectSelectionList`, `OptionsList`, `PlainTextButton`, `PlayerFaceExtractor`, `PlayerFaceWidget`, `PlayerSkinWidget`, `PlayerTabOverlay`, `PopupScreen`, `RealmsButton`, `Renderable`, `ResettableOptionWidget`, `ScrollableLayout`, `SelectableEntry`, `SplashRenderer`, `SpriteIconButton`, [[40-Interfaces/net.minecraft.client.gui.components.StringWidget|StringWidget]], [[40-Interfaces/net.minecraft.client.gui.components.SubtitleOverlay|SubtitleOverlay]], `TabButton`, `TabOrderedElement`, `TextCursorUtils`, `Tooltip`, `Whence`, `WidgetSprites`, `WidgetTooltipHolder`, `package-info`

### `net.minecraft.client.gui.components.debug` (37 top-level)

`DebugEntryBiome`, [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugEntryCategory|DebugEntryCategory]], `DebugEntryChunkGeneration`, `DebugEntryChunkRenderStats`, `DebugEntryChunkSourceStats`, `DebugEntryDayCount`, `DebugEntryDetailedMemory`, `DebugEntryEntityRenderStats`, `DebugEntryFps`, `DebugEntryGpuUtilization`, `DebugEntryHeightmap`, `DebugEntryLight`, `DebugEntryLocalDifficulty`, `DebugEntryLookingAt`, `DebugEntryLookingAtEntity`, `DebugEntryLookingAtEntityTags`, `DebugEntryMemory`, `DebugEntryNoop`, `DebugEntryParticleRenderStats`, `DebugEntryPlayerSpeed`, `DebugEntryPosition`, `DebugEntryPostEffects`, `DebugEntrySectionPosition`, `DebugEntrySimplePerformanceImpactors`, `DebugEntrySoundCache`, `DebugEntrySoundMood`, `DebugEntrySpawnCounts`, `DebugEntrySystemSpecs`, `DebugEntryTps`, `DebugEntryVersion`, [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenDisplayer|DebugScreenDisplayer]], [[40-Interfaces/net.minecraft.client.gui.components.debug.DebugScreenEntries|DebugScreenEntries]], `DebugScreenEntry`, `DebugScreenEntryList`, `DebugScreenEntryStatus`, `DebugScreenProfile`, `package-info`

### `net.minecraft.client.gui.components.debugchart` (7 top-level)

`AbstractDebugChart`, `BandwidthDebugChart`, `FpsDebugChart`, `PingDebugChart`, `ProfilerPieChart`, `TpsDebugChart`, `package-info`

### `net.minecraft.client.gui.components.events` (4 top-level)

`AbstractContainerEventHandler`, `ContainerEventHandler`, [[40-Interfaces/net.minecraft.client.gui.components.events.GuiEventListener|GuiEventListener]], `package-info`

### `net.minecraft.client.gui.components.spectator` (2 top-level)

`SpectatorGui`, `package-info`

### `net.minecraft.client.gui.components.tabs` (7 top-level)

`GridLayoutTab`, `LoadingTab`, `MenuTabBar`, `Tab`, `TabManager`, `TabNavigationBar`, `package-info`

### `net.minecraft.client.gui.components.toasts` (9 top-level)

[[40-Interfaces/net.minecraft.client.gui.components.toasts.AdvancementToast|AdvancementToast]], `FriendToast`, `NowPlayingToast`, `RecipeToast`, `SystemToast`, `Toast`, `ToastManager`, `TutorialToast`, `package-info`

### `net.minecraft.client.gui.contextualbar` (5 top-level)

`ContextualBar`, `ExperienceBar`, `JumpableVehicleBar`, `LocatorBar`, `package-info`

### `net.minecraft.client.gui.font` (17 top-level)

`ActiveArea`, `AllMissingGlyphProvider`, `AtlasGlyphProvider`, `CodepointMap`, `EmptyArea`, `FontManager`, `FontOption`, `FontSet`, `FontTexture`, `GlyphRenderTypes`, `GlyphStitcher`, `PlainTextRenderable`, `PlayerGlyphProvider`, `SingleSpriteSource`, `TextFieldHelper`, `TextRenderable`, `package-info`

### `net.minecraft.client.gui.font.glyphs` (6 top-level)

`BakedGlyph`, `BakedSheetGlyph`, `EffectGlyph`, `EmptyGlyph`, `SpecialGlyphs`, `package-info`

### `net.minecraft.client.gui.font.providers` (8 top-level)

`BitmapProvider`, `FreeTypeUtil`, `GlyphProviderDefinition`, `GlyphProviderType`, `ProviderReferenceDefinition`, `TrueTypeGlyphProviderDefinition`, `UnihexProvider`, `package-info`

### `net.minecraft.client.gui.layouts` (12 top-level)

`AbstractLayout`, `CommonLayouts`, `EqualSpacingLayout`, `FrameLayout`, `GridLayout`, [[40-Interfaces/net.minecraft.client.gui.layouts.HeaderAndFooterLayout|HeaderAndFooterLayout]], [[40-Interfaces/net.minecraft.client.gui.layouts.Layout|Layout]], [[40-Interfaces/net.minecraft.client.gui.layouts.LayoutElement|LayoutElement]], `LayoutSettings`, [[40-Interfaces/net.minecraft.client.gui.layouts.LinearLayout|LinearLayout]], `SpacerElement`, `package-info`

### `net.minecraft.client.gui.narration` (8 top-level)

`NarratableEntry`, `NarratedElementType`, `NarrationElementOutput`, `NarrationSupplier`, `NarrationThunk`, `NarrationTrigger`, `ScreenNarrationCollector`, `package-info`

### `net.minecraft.client.gui.navigation` (6 top-level)

`FocusNavigationEvent`, `ScreenAxis`, `ScreenDirection`, `ScreenPosition`, [[40-Interfaces/net.minecraft.client.gui.navigation.ScreenRectangle|ScreenRectangle]], `package-info`

### `net.minecraft.client.gui.render` (5 top-level)

`DynamicAtlasAllocator`, `GuiItemAtlas`, [[40-Interfaces/net.minecraft.client.gui.render.GuiRenderer|GuiRenderer]], `TextureSetup`, `package-info`

### `net.minecraft.client.gui.render.pip` (8 top-level)

[[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBannerResultRenderer|GuiBannerResultRenderer]], [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiBookModelRenderer|GuiBookModelRenderer]], [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiEntityRenderer|GuiEntityRenderer]], [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiProfilerChartRenderer|GuiProfilerChartRenderer]], [[40-Interfaces/net.minecraft.client.gui.render.pip.GuiSkinRenderer|GuiSkinRenderer]], `OversizedItemRenderer`, [[40-Interfaces/net.minecraft.client.gui.render.pip.PictureInPictureRenderer|PictureInPictureRenderer]], `package-info`

### `net.minecraft.client.gui.screens` (39 top-level)

`AccessibilityOnboardingScreen`, `AlertScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.BackupConfirmScreen|BackupConfirmScreen]], `BanNoticeScreens`, `ChatScreen`, `ConfirmLinkScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.ConfirmScreen|ConfirmScreen]], [[40-Interfaces/net.minecraft.client.gui.screens.ConnectScreen|ConnectScreen]], `CreateBuffetWorldScreen`, `CreateFlatWorldScreen`, `CreditsAndAttributionScreen`, `DatapackLoadFailureScreen`, `DeathScreen`, `DirectJoinServerScreen`, `DisconnectedScreen`, `ErrorScreen`, `FaviconTexture`, `FileFixerAbortedScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.GenericMessageScreen|GenericMessageScreen]], `GenericWaitingScreen`, `InBedChatScreen`, `LevelLoadingScreen`, `LoadingDotsText`, `LoadingOverlay`, `ManageServerScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.MenuScreens|MenuScreens]], `NoticeWithLinkScreen`, `OutOfMemoryScreen`, `Overlay`, `PauseScreen`, `PresetFlatWorldScreen`, `PrivacyConfirmLinkScreen`, `ProgressScreen`, `RecoverWorldDataScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.Screen|Screen]], `TitleScreen`, `WinScreen`, `WorldOptionsScreen`, `package-info`

### `net.minecraft.client.gui.screens.achievement` (2 top-level)

`StatsScreen`, `package-info`

### `net.minecraft.client.gui.screens.advancements` (6 top-level)

[[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTab|AdvancementTab]], [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementTabType|AdvancementTabType]], [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementWidget|AdvancementWidget]], `AdvancementWidgetType`, [[40-Interfaces/net.minecraft.client.gui.screens.advancements.AdvancementsScreen|AdvancementsScreen]], `package-info`

### `net.minecraft.client.gui.screens.debug` (3 top-level)

`DebugOptionsScreen`, `GameModeSwitcherScreen`, `package-info`

### `net.minecraft.client.gui.screens.dialog` (11 top-level)

`ButtonListDialogScreen`, `DialogConnectionAccess`, `DialogControlSet`, `DialogListDialogScreen`, `DialogScreen`, `DialogScreens`, `MultiButtonDialogScreen`, `ServerLinksDialogScreen`, `SimpleDialogScreen`, `WaitingForResponseScreen`, `package-info`

### `net.minecraft.client.gui.screens.dialog.body` (3 top-level)

`DialogBodyHandler`, `DialogBodyHandlers`, `package-info`

### `net.minecraft.client.gui.screens.dialog.input` (3 top-level)

`InputControlHandler`, `InputControlHandlers`, `package-info`

### `net.minecraft.client.gui.screens.friends` (12 top-level)

`AbstractFriendsEntryContainerWidget`, `AbstractFriendsTab`, `AddFriendWidget`, `FriendEntry`, `FriendsListConfirmScreen`, `FriendsOverlayScreen`, `FriendsOverlayTabButton`, `FriendsTab`, `IncomingEntry`, `OutgoingEntry`, `PendingTab`, `package-info`

### `net.minecraft.client.gui.screens.inventory` (49 top-level)

`AbstractCommandBlockEditScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractContainerScreen|AbstractContainerScreen]], `AbstractFurnaceScreen`, `AbstractMountInventoryScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen|AbstractRecipeBookScreen]], [[40-Interfaces/net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen|AbstractSignEditScreen]], `AnvilScreen`, `BeaconScreen`, `BlastFurnaceScreen`, `BookEditScreen`, `BookSignScreen`, `BookViewScreen`, `BrewingStandScreen`, `CartographyTableScreen`, `CommandBlockEditScreen`, `ContainerScreen`, `CrafterScreen`, `CraftingScreen`, `CreativeInventoryListener`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]], `CyclingSlotBackground`, `DispenserScreen`, `EffectsInInventory`, `EnchantmentNames`, `EnchantmentScreen`, `FurnaceScreen`, `GrindstoneScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.HangingSignEditScreen|HangingSignEditScreen]], `HopperScreen`, `HorseInventoryScreen`, `InventoryScreen`, `ItemCombinerScreen`, `JigsawBlockEditScreen`, `LecternScreen`, `LoomScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.MenuAccess|MenuAccess]], `MerchantScreen`, `MinecartCommandBlockEditScreen`, `NautilusInventoryScreen`, `PageButton`, `ShulkerBoxScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.SignEditScreen|SignEditScreen]], `SmithingScreen`, `SmokerScreen`, `StonecutterScreen`, `StructureBlockEditScreen`, `TestBlockEditScreen`, `TestInstanceBlockEditScreen`, `package-info`

### `net.minecraft.client.gui.screens.inventory.tooltip` (10 top-level)

`BelowOrAboveWidgetTooltipPositioner`, `ClientActivePlayersTooltip`, `ClientBundleTooltip`, `ClientTextTooltip`, [[40-Interfaces/net.minecraft.client.gui.screens.inventory.tooltip.ClientTooltipComponent|ClientTooltipComponent]], `ClientTooltipPositioner`, `DefaultTooltipPositioner`, `MenuTooltipPositioner`, `TooltipRenderUtil`, `package-info`

### `net.minecraft.client.gui.screens.multiplayer` (8 top-level)

`CodeOfConductScreen`, `JoinMultiplayerScreen`, `RestrictionsScreen`, `SafetyScreen`, `ServerReconfigScreen`, `ServerSelectionList`, `WarningScreen`, `package-info`

### `net.minecraft.client.gui.screens.options` (16 top-level)

`AccessibilityOptionsScreen`, `ChatOptionsScreen`, `FontOptionsScreen`, `HasDifficultyReaction`, `HasGamemasterPermissionReaction`, `InWorldGameRulesScreen`, `LanguageSelectScreen`, `MouseSettingsScreen`, `OnlineOptionsScreen`, `OptionsScreen`, `OptionsSubScreen`, `SkinCustomizationScreen`, `SoundOptionsScreen`, `UnsupportedGraphicsWarningScreen`, `VideoSettingsScreen`, `package-info`

### `net.minecraft.client.gui.screens.options.controls` (4 top-level)

`ControlsScreen`, `KeyBindsList`, `KeyBindsScreen`, `package-info`

### `net.minecraft.client.gui.screens.packs` (4 top-level)

[[40-Interfaces/net.minecraft.client.gui.screens.packs.PackSelectionModel|PackSelectionModel]], `PackSelectionScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.packs.TransferableSelectionList|TransferableSelectionList]], `package-info`

### `net.minecraft.client.gui.screens.recipebook` (13 top-level)

`CraftingRecipeBookComponent`, `FurnaceRecipeBookComponent`, `GhostSlots`, `OverlayRecipeComponent`, `RecipeBookComponent`, `RecipeBookPage`, `RecipeBookTabButton`, `RecipeButton`, `RecipeCollection`, `RecipeUpdateListener`, `SearchRecipeBookCategory`, `SlotSelectTime`, `package-info`

### `net.minecraft.client.gui.screens.reporting` (9 top-level)

`AbstractReportScreen`, `ChatReportScreen`, `ChatSelectionLogFiller`, `ChatSelectionScreen`, `NameReportScreen`, `ReportPlayerScreen`, `ReportReasonSelectionScreen`, `SkinReportScreen`, `package-info`

### `net.minecraft.client.gui.screens.social` (7 top-level)

`PlayerEntry`, `PlayerSocialManager`, `PresenceHandler`, `RemoteFriendListUpdateHandler`, `SocialInteractionsPlayerList`, `SocialInteractionsScreen`, `package-info`

### `net.minecraft.client.gui.screens.telemetry` (3 top-level)

`TelemetryEventWidget`, `TelemetryInfoScreen`, `package-info`

### `net.minecraft.client.gui.screens.worldselection` (20 top-level)

`AbstractGameRulesScreen`, `ConfirmExperimentalFeaturesScreen`, `CreateWorldCallback`, [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.CreateWorldScreen|CreateWorldScreen]], `DataPackReloadCookie`, `EditWorldScreen`, `ExperimentsScreen`, `FileFixerProgressScreen`, `InitialWorldCreationOptions`, `OptimizeWorldScreen`, `PresetEditor`, `SelectWorldScreen`, `SwitchGrid`, [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationContext|WorldCreationContext]], `WorldCreationContextMapper`, `WorldCreationGameRulesScreen`, [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldCreationUiState|WorldCreationUiState]], [[40-Interfaces/net.minecraft.client.gui.screens.worldselection.WorldOpenFlows|WorldOpenFlows]], `WorldSelectionList`, `package-info`

### `net.minecraft.client.gui.spectator` (7 top-level)

`PlayerMenuItem`, `RootSpectatorMenuCategory`, `SpectatorMenu`, `SpectatorMenuCategory`, `SpectatorMenuItem`, `SpectatorMenuListener`, `package-info`

### `net.minecraft.client.gui.spectator.categories` (4 top-level)

`SpectatorPage`, `TeleportToPlayerMenuCategory`, `TeleportToTeamMenuCategory`, `package-info`

