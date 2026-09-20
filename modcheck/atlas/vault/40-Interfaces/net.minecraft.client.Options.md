---
type: "interface"
fqcn: "net.minecraft.client.Options"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Options

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `chunkSectionFadeInTime` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@29 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `cloudStatus` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@8 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `cutoutLeaves` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@6 in `FabricBlockStateModel.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `cutoutLeaves` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@6 in `SingleVariantMixin.emitQuads` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getEffectiveRenderDistance` | `()I` | exact | invokevirtual@4 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getSoundSourceOptionInstance` | `(Lnet/minecraft/sounds/SoundSource;)Lnet/minecraft/client/OptionInstan` | exact | invokevirtual@59 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `maxAnisotropyBit` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@18 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `renderDistance` | `()Lnet/minecraft/client/OptionInstance;` | exact | invokevirtual@45 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `load` | `()V` | exact | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| injects_into | `load` | `()V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `advancedItemTooltips` | `Z` | exact | getfield@8 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `advancedItemTooltips` | `Z` | exact | getfield@8 in `FluidVariantRendering.getTooltip` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `keyAttack` | `Lnet/minecraft/client/KeyMapping;` | exact | getfield@4 in `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `keyAttack` | `Lnet/minecraft/client/KeyMapping;` | exact | getfield@20 in `MinecraftMixin.injectHandleInputEventsForPreAttackCallback` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `keyMappings` | `[Lnet/minecraft/client/KeyMapping;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | declared |
| reads | `resourcePacks` | `Ljava/util/List;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| wraps | `updateResourcePacks` | `(Lnet/minecraft/server/packs/repository/PackRepository;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| writes | `onboardAccessibility` | `Z` | exact | putfield@41 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| writes | `tutorialStep` | `Lnet/minecraft/client/tutorial/TutorialSteps;` | exact | putfield@4 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (245 fields, 264 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final GSON : Lcom/google/gson/Gson;
private static final LIST_OF_STRINGS_TYPE : Lcom/google/gson/reflect/TypeToken;
public static final RENDER_DISTANCE_SHORT : I
public static final RENDER_DISTANCE_FAR : I
public static final RENDER_DISTANCE_REALLY_FAR : I
public static final RENDER_DISTANCE_EXTREME : I
private static final OPTION_SPLITTER : Lcom/google/common/base/Splitter;
private static final DEFAULT_SOUND_DEVICE : Ljava/lang/String;
private static final TOOLTIP_NEEDS_RESTART : Lnet/minecraft/network/chat/Component;
private static final ACCESSIBILITY_TOOLTIP_DARK_MOJANG_BACKGROUND : Lnet/minecraft/network/chat/Component;
private final darkMojangStudiosBackground : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_HIDE_LIGHTNING_FLASHES : Lnet/minecraft/network/chat/Component;
private final hideLightningFlash : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_HIDE_SPLASH_TEXTS : Lnet/minecraft/network/chat/Component;
private final hideSplashTexts : Lnet/minecraft/client/OptionInstance;
private final sensitivity : Lnet/minecraft/client/OptionInstance;
private final renderDistance : Lnet/minecraft/client/OptionInstance;
private final simulationDistance : Lnet/minecraft/client/OptionInstance;
private serverRenderDistance : I
private final entityDistanceScaling : Lnet/minecraft/client/OptionInstance;
public static final UNLIMITED_FRAMERATE_CUTOFF : I
private final framerateLimit : Lnet/minecraft/client/OptionInstance;
private preferredGraphicsBackendFromStartup : Lnet/minecraft/client/PreferredGraphicsApi;
private static final GRAPHICS_API_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static final GRAPHICS_API_TOOLTIP_VULKAN : Lnet/minecraft/network/chat/Component;
private final preferredGraphicsBackend : Lnet/minecraft/client/OptionInstance;
private isApplyingGraphicsPreset : Z
private final graphicsPreset : Lnet/minecraft/client/OptionInstance;
private static final INACTIVITY_FPS_LIMIT_TOOLTIP_MINIMIZED : Lnet/minecraft/network/chat/Component;
private static final INACTIVITY_FPS_LIMIT_TOOLTIP_AFK : Lnet/minecraft/network/chat/Component;
private final inactivityFpsLimit : Lnet/minecraft/client/OptionInstance;
private final cloudStatus : Lnet/minecraft/client/OptionInstance;
private final cloudRange : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_WEATHER_RADIUS : Lnet/minecraft/network/chat/Component;
private final weatherRadius : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_CUTOUT_LEAVES : Lnet/minecraft/network/chat/Component;
private final cutoutLeaves : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_VIGNETTE : Lnet/minecraft/network/chat/Component;
private final vignette : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_IMPROVED_TRANSPARENCY : Lnet/minecraft/network/chat/Component;
private final improvedTransparency : Lnet/minecraft/client/OptionInstance;
private final ambientOcclusion : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_CHUNK_FADE : Lnet/minecraft/network/chat/Component;
private final chunkSectionFadeInTime : Lnet/minecraft/client/OptionInstance;
private static final PRIORITIZE_CHUNK_TOOLTIP_NONE : Lnet/minecraft/network/chat/Component;
private static final PRIORITIZE_CHUNK_TOOLTIP_PLAYER_AFFECTED : Lnet/minecraft/network/chat/Component;
private static final PRIORITIZE_CHUNK_TOOLTIP_NEARBY : Lnet/minecraft/network/chat/Component;
private final prioritizeChunkUpdates : Lnet/minecraft/client/OptionInstance;
public resourcePacks : Ljava/util/List;
public incompatibleResourcePacks : Ljava/util/List;
private final chatVisibility : Lnet/minecraft/client/OptionInstance;
private final chatOpacity : Lnet/minecraft/client/OptionInstance;
private final chatLineSpacing : Lnet/minecraft/client/OptionInstance;
private static final MENU_BACKGROUND_BLURRINESS_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static final BLURRINESS_DEFAULT_VALUE : I
private final menuBackgroundBlurriness : Lnet/minecraft/client/OptionInstance;
private final textBackgroundOpacity : Lnet/minecraft/client/OptionInstance;
private final panoramaSpeed : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_CONTRAST_MODE : Lnet/minecraft/network/chat/Component;
private final highContrast : Lnet/minecraft/client/OptionInstance;
private static final HIGH_CONTRAST_BLOCK_OUTLINE_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final highContrastBlockOutline : Lnet/minecraft/client/OptionInstance;
private final narratorHotkey : Lnet/minecraft/client/OptionInstance;
public fullscreenVideoModeString : Ljava/lang/String;
public hideServerAddress : Z
public advancedItemTooltips : Z
public pauseOnLostFocus : Z
private final modelParts : Ljava/util/Set;
private final mainHand : Lnet/minecraft/client/OptionInstance;
public overrideWidth : I
public overrideHeight : I
private final chatScale : Lnet/minecraft/client/OptionInstance;
private final chatWidth : Lnet/minecraft/client/OptionInstance;
private final chatHeightUnfocused : Lnet/minecraft/client/OptionInstance;
private final chatHeightFocused : Lnet/minecraft/client/OptionInstance;
private final chatDelay : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_NOTIFICATION_DISPLAY_TIME : Lnet/minecraft/network/chat/Component;
private final notificationDisplayTime : Lnet/minecraft/client/OptionInstance;
private final mipmapLevels : Lnet/minecraft/client/OptionInstance;
private static final GRAPHICS_TOOLTIP_ANISOTROPIC_FILTERING : Lnet/minecraft/network/chat/Component;
private final maxAnisotropyBit : Lnet/minecraft/client/OptionInstance;
private static final FILTERING_NONE_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static final FILTERING_RGSS_TOOLTIP : Lnet/minecraft/network/chat/Component;
private static final FILTERING_ANISOTROPIC_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final textureFiltering : Lnet/minecraft/client/OptionInstance;
private useNativeTransport : Z
private final attackIndicator : Lnet/minecraft/client/OptionInstance;
public tutorialStep : Lnet/minecraft/client/tutorial/TutorialSteps;
public joinedFirstServer : Z
private final biomeBlendRadius : Lnet/minecraft/client/OptionInstance;
private final mouseWheelSensitivity : Lnet/minecraft/client/OptionInstance;
private static final ALLOW_CURSOR_CHANGES_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final allowCursorChanges : Lnet/minecraft/client/OptionInstance;
private static final QUIT_SHORTCUTS_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final quitShortcuts : Lnet/minecraft/client/OptionInstance;
private static final CTRL_CLICK_EMULATES_RIGHT_CLICK_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final ctrlClickEmulatesRightClick : Lnet/minecraft/client/OptionInstance;
public glDebugVerbosity : I
private final autoJump : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_ROTATE_WITH_MINECART : Lnet/minecraft/network/chat/Component;
private final rotateWithMinecart : Lnet/minecraft/client/OptionInstance;
private final operatorItemsTab : Lnet/minecraft/client/OptionInstance;
private final autoSuggestions : Lnet/minecraft/client/OptionInstance;
private final chatColors : Lnet/minecraft/client/OptionInstance;
private final chatLinks : Lnet/minecraft/client/OptionInstance;
private final chatLinksPrompt : Lnet/minecraft/client/OptionInstance;
private final enableVsync : Lnet/minecraft/client/OptionInstance;
private final entityShadows : Lnet/minecraft/client/OptionInstance;
private final forceUnicodeFont : Lnet/minecraft/client/OptionInstance;
private final japaneseGlyphVariants : Lnet/minecraft/client/OptionInstance;
private final invertXMouse : Lnet/minecraft/client/OptionInstance;
private final invertYMouse : Lnet/minecraft/client/OptionInstance;
private final discreteMouseScroll : Lnet/minecraft/client/OptionInstance;
private static final REALMS_NOTIFICATIONS_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final realmsNotifications : Lnet/minecraft/client/OptionInstance;
private static final ALLOW_SERVER_LISTING_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final allowServerListing : Lnet/minecraft/client/OptionInstance;
private final reducedDebugInfo : Lnet/minecraft/client/OptionInstance;
private static final IN_GAME_NOTIFICATION_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final inGameNotification : Lnet/minecraft/client/OptionInstance;
private final sharePresence : Lnet/minecraft/client/OptionInstance;
private final soundSourceVolumes : Ljava/util/Map;
private static final CLOSED_CAPTIONS_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final showSubtitles : Lnet/minecraft/client/OptionInstance;
private static final DIRECTIONAL_AUDIO_TOOLTIP_ON : Lnet/minecraft/network/chat/Component;
private static final DIRECTIONAL_AUDIO_TOOLTIP_OFF : Lnet/minecraft/network/chat/Component;
private final directionalAudio : Lnet/minecraft/client/OptionInstance;
private final backgroundForChatOnly : Lnet/minecraft/client/OptionInstance;
private final fullscreen : Lnet/minecraft/client/OptionInstance;
private static final TOOLTIP_EXCLUSIVE_FULLSCREEN_ON : Lnet/minecraft/network/chat/Component;
private static final TOOLTIP_EXCLUSIVE_FULLSCREEN_OFF : Lnet/minecraft/network/chat/Component;
private final exclusiveFullscreen : Lnet/minecraft/client/OptionInstance;
private static final MAC_FULLSCREEN_MENU_VISIBILITY_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final macFullscreenMenuVisibility : Lnet/minecraft/client/OptionInstance;
private final bobView : Lnet/minecraft/client/OptionInstance;
private static final KEY_TOGGLE : Lnet/minecraft/network/chat/Component;
private static final KEY_HOLD : Lnet/minecraft/network/chat/Component;
private final toggleCrouch : Lnet/minecraft/client/OptionInstance;
private final toggleSprint : Lnet/minecraft/client/OptionInstance;
private final toggleAttack : Lnet/minecraft/client/OptionInstance;
private final toggleUse : Lnet/minecraft/client/OptionInstance;
private static final SPRINT_WINDOW_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final sprintWindow : Lnet/minecraft/client/OptionInstance;
public skipMultiplayerWarning : Z
private static final CHAT_TOOLTIP_HIDE_MATCHED_NAMES : Lnet/minecraft/network/chat/Component;
private final hideMatchedNames : Lnet/minecraft/client/OptionInstance;
private final showAutosaveIndicator : Lnet/minecraft/client/OptionInstance;
private static final CHAT_TOOLTIP_ONLY_SHOW_SECURE : Lnet/minecraft/network/chat/Component;
private final onlyShowSecureChat : Lnet/minecraft/client/OptionInstance;
private static final CHAT_TOOLTIP_SAVE_DRAFTS : Lnet/minecraft/network/chat/Component;
private final saveChatDrafts : Lnet/minecraft/client/OptionInstance;
public final keyUp : Lnet/minecraft/client/KeyMapping;
public final keyLeft : Lnet/minecraft/client/KeyMapping;
public final keyDown : Lnet/minecraft/client/KeyMapping;
public final keyRight : Lnet/minecraft/client/KeyMapping;
public final keyJump : Lnet/minecraft/client/KeyMapping;
public final keyShift : Lnet/minecraft/client/KeyMapping;
public final keySprint : Lnet/minecraft/client/KeyMapping;
public final keyInventory : Lnet/minecraft/client/KeyMapping;
public final keySwapOffhand : Lnet/minecraft/client/KeyMapping;
public final keyDrop : Lnet/minecraft/client/KeyMapping;
public final keyUse : Lnet/minecraft/client/KeyMapping;
public final keyAttack : Lnet/minecraft/client/KeyMapping;
public final keyPickItem : Lnet/minecraft/client/KeyMapping;
public final keyChat : Lnet/minecraft/client/KeyMapping;
public final keyPlayerList : Lnet/minecraft/client/KeyMapping;
public final keyCommand : Lnet/minecraft/client/KeyMapping;
public final keyFriends : Lnet/minecraft/client/KeyMapping;
public final keySocialInteractions : Lnet/minecraft/client/KeyMapping;
public final keyScreenshot : Lnet/minecraft/client/KeyMapping;
public final keyTogglePerspective : Lnet/minecraft/client/KeyMapping;
public final keySmoothCamera : Lnet/minecraft/client/KeyMapping;
public final keyFullscreen : Lnet/minecraft/client/KeyMapping;
public final keyAdvancements : Lnet/minecraft/client/KeyMapping;
public final keyQuickActions : Lnet/minecraft/client/KeyMapping;
public final keyToggleGui : Lnet/minecraft/client/KeyMapping;
public final keyToggleSpectatorShaderEffects : Lnet/minecraft/client/KeyMapping;
public final keyHotbarSlots : [Lnet/minecraft/client/KeyMapping;
public final keySaveHotbarActivator : Lnet/minecraft/client/KeyMapping;
public final keyLoadHotbarActivator : Lnet/minecraft/client/KeyMapping;
public final keySpectatorOutlines : Lnet/minecraft/client/KeyMapping;
public final keySpectatorHotbar : Lnet/minecraft/client/KeyMapping;
public final keyDebugOverlay : Lnet/minecraft/client/KeyMapping;
public final keyDebugModifier : Lnet/minecraft/client/KeyMapping;
public final keyDebugCrash : Lnet/minecraft/client/KeyMapping;
public final keyDebugReloadChunk : Lnet/minecraft/client/KeyMapping;
public final keyDebugShowHitboxes : Lnet/minecraft/client/KeyMapping;
public final keyDebugClearChat : Lnet/minecraft/client/KeyMapping;
public final keyDebugShowChunkBorders : Lnet/minecraft/client/KeyMapping;
public final keyDebugShowAdvancedTooltips : Lnet/minecraft/client/KeyMapping;
public final keyDebugCopyRecreateCommand : Lnet/minecraft/client/KeyMapping;
public final keyDebugSpectate : Lnet/minecraft/client/KeyMapping;
public final keyDebugSwitchGameMode : Lnet/minecraft/client/KeyMapping;
public final keyDebugDebugOptions : Lnet/minecraft/client/KeyMapping;
public final keyDebugFocusPause : Lnet/minecraft/client/KeyMapping;
public final keyDebugDumpDynamicTextures : Lnet/minecraft/client/KeyMapping;
public final keyDebugReloadResourcePacks : Lnet/minecraft/client/KeyMapping;
public final keyDebugProfiling : Lnet/minecraft/client/KeyMapping;
public final keyDebugCopyLocation : Lnet/minecraft/client/KeyMapping;
public final keyDebugDumpVersion : Lnet/minecraft/client/KeyMapping;
public final keyDebugPofilingChart : Lnet/minecraft/client/KeyMapping;
public final keyDebugFpsCharts : Lnet/minecraft/client/KeyMapping;
public final keyDebugNetworkCharts : Lnet/minecraft/client/KeyMapping;
public final keyDebugLightmapTexture : Lnet/minecraft/client/KeyMapping;
public final keyDebugSwitchTranslucencyMode : Lnet/minecraft/client/KeyMapping;
public final debugKeys : [Lnet/minecraft/client/KeyMapping;
public final keyMappings : [Lnet/minecraft/client/KeyMapping;
protected minecraft : Lnet/minecraft/client/Minecraft;
private final optionsFile : Ljava/io/File;
private cameraType : Lnet/minecraft/client/CameraType;
public lastMpIp : Ljava/lang/String;
public smoothCamera : Z
private final fov : Lnet/minecraft/client/OptionInstance;
private static final TELEMETRY_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final telemetryOptInExtra : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_SCREEN_EFFECT : Lnet/minecraft/network/chat/Component;
private final screenEffectScale : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_FOV_EFFECT : Lnet/minecraft/network/chat/Component;
private final fovEffectScale : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_DARKNESS_EFFECT : Lnet/minecraft/network/chat/Component;
private final darknessEffectScale : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_GLINT_SPEED : Lnet/minecraft/network/chat/Component;
private final glintSpeed : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_GLINT_STRENGTH : Lnet/minecraft/network/chat/Component;
private final glintStrength : Lnet/minecraft/client/OptionInstance;
private static final ACCESSIBILITY_TOOLTIP_DAMAGE_TILT_STRENGTH : Lnet/minecraft/network/chat/Component;
private final damageTiltStrength : Lnet/minecraft/client/OptionInstance;
private final gamma : Lnet/minecraft/client/OptionInstance;
public static final AUTO_GUI_SCALE : I
private static final MAX_GUI_SCALE_INCLUSIVE : I
private final guiScale : Lnet/minecraft/client/OptionInstance;
public static final DEBUG_GUI_SCALE_UNCHANGED : I
private static final DEBUG_GUI_SCALE_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final debugGuiScale : Lnet/minecraft/client/OptionInstance;
private final particles : Lnet/minecraft/client/OptionInstance;
private final narrator : Lnet/minecraft/client/OptionInstance;
public languageCode : Ljava/lang/String;
private final soundDevice : Lnet/minecraft/client/OptionInstance;
public onboardAccessibility : Z
private static final MUSIC_FREQUENCY_TOOLTIP : Lnet/minecraft/network/chat/Component;
private final musicFrequency : Lnet/minecraft/client/OptionInstance;
private final musicToast : Lnet/minecraft/client/OptionInstance;
public syncWrites : Z
public startedCleanly : Z
public static isSoundDeviceDefault(Ljava/lang/String;)Z
private static operateOnLevelExtractor(Ljava/util/function/Consumer;)V
public darkMojangStudiosBackground()Lnet/minecraft/client/OptionInstance;
public hideLightningFlash()Lnet/minecraft/client/OptionInstance;
public hideSplashTexts()Lnet/minecraft/client/OptionInstance;
public sensitivity()Lnet/minecraft/client/OptionInstance;
public renderDistance()Lnet/minecraft/client/OptionInstance;
public simulationDistance()Lnet/minecraft/client/OptionInstance;
public entityDistanceScaling()Lnet/minecraft/client/OptionInstance;
public framerateLimit()Lnet/minecraft/client/OptionInstance;
public preferredGraphicsBackend()Lnet/minecraft/client/OptionInstance;
public isRestartRequiredToApplyVideoSettings()Z
public applyGraphicsPreset(Lnet/minecraft/client/GraphicsPreset;)V
public graphicsPreset()Lnet/minecraft/client/OptionInstance;
public inactivityFpsLimit()Lnet/minecraft/client/OptionInstance;
public cloudStatus()Lnet/minecraft/client/OptionInstance;
public cloudRange()Lnet/minecraft/client/OptionInstance;
public weatherRadius()Lnet/minecraft/client/OptionInstance;
public cutoutLeaves()Lnet/minecraft/client/OptionInstance;
public vignette()Lnet/minecraft/client/OptionInstance;
public improvedTransparency()Lnet/minecraft/client/OptionInstance;
public ambientOcclusion()Lnet/minecraft/client/OptionInstance;
public chunkSectionFadeInTime()Lnet/minecraft/client/OptionInstance;
public prioritizeChunkUpdates()Lnet/minecraft/client/OptionInstance;
public updateResourcePacks(Lnet/minecraft/server/packs/repository/PackRepository;)V
public chatVisibility()Lnet/minecraft/client/OptionInstance;
public chatOpacity()Lnet/minecraft/client/OptionInstance;
public chatLineSpacing()Lnet/minecraft/client/OptionInstance;
public menuBackgroundBlurriness()Lnet/minecraft/client/OptionInstance;
public getMenuBackgroundBlurriness()I
public textBackgroundOpacity()Lnet/minecraft/client/OptionInstance;
public panoramaSpeed()Lnet/minecraft/client/OptionInstance;
public highContrast()Lnet/minecraft/client/OptionInstance;
public highContrastBlockOutline()Lnet/minecraft/client/OptionInstance;
public narratorHotkey()Lnet/minecraft/client/OptionInstance;
public mainHand()Lnet/minecraft/client/OptionInstance;
public chatScale()Lnet/minecraft/client/OptionInstance;
public chatWidth()Lnet/minecraft/client/OptionInstance;
public chatHeightUnfocused()Lnet/minecraft/client/OptionInstance;
public chatHeightFocused()Lnet/minecraft/client/OptionInstance;
public chatDelay()Lnet/minecraft/client/OptionInstance;
public notificationDisplayTime()Lnet/minecraft/client/OptionInstance;
public mipmapLevels()Lnet/minecraft/client/OptionInstance;
public maxAnisotropyBit()Lnet/minecraft/client/OptionInstance;
public maxAnisotropyValue()I
public textureFiltering()Lnet/minecraft/client/OptionInstance;
public attackIndicator()Lnet/minecraft/client/OptionInstance;
public biomeBlendRadius()Lnet/minecraft/client/OptionInstance;
private static logMouse(I)D
private static unlogMouse(D)I
public mouseWheelSensitivity()Lnet/minecraft/client/OptionInstance;
public allowCursorChanges()Lnet/minecraft/client/OptionInstance;
public quitShortcuts()Lnet/minecraft/client/OptionInstance;
public ctrlClickEmulatesRightClick()Lnet/minecraft/client/OptionInstance;
public autoJump()Lnet/minecraft/client/OptionInstance;
public rotateWithMinecart()Lnet/minecraft/client/OptionInstance;
public operatorItemsTab()Lnet/minecraft/client/OptionInstance;
public autoSuggestions()Lnet/minecraft/client/OptionInstance;
public chatColors()Lnet/minecraft/client/OptionInstance;
public chatLinks()Lnet/minecraft/client/OptionInstance;
public chatLinksPrompt()Lnet/minecraft/client/OptionInstance;
public enableVsync()Lnet/minecraft/client/OptionInstance;
public entityShadows()Lnet/minecraft/client/OptionInstance;
private static updateFontOptions()V
public forceUnicodeFont()Lnet/minecraft/client/OptionInstance;
private static japaneseGlyphVariantsDefault()Z
public japaneseGlyphVariants()Lnet/minecraft/client/OptionInstance;
public invertMouseX()Lnet/minecraft/client/OptionInstance;
public invertMouseY()Lnet/minecraft/client/OptionInstance;
public discreteMouseScroll()Lnet/minecraft/client/OptionInstance;
public realmsNotifications()Lnet/minecraft/client/OptionInstance;
public allowServerListing()Lnet/minecraft/client/OptionInstance;
public reducedDebugInfo()Lnet/minecraft/client/OptionInstance;
public inGameNotification()Lnet/minecraft/client/OptionInstance;
public sharePresence()Lnet/minecraft/client/OptionInstance;
public final getFinalSoundSourceVolume(Lnet/minecraft/sounds/SoundSource;)F
public final getSoundSourceVolume(Lnet/minecraft/sounds/SoundSource;)F
public final getSoundSourceOptionInstance(Lnet/minecraft/sounds/SoundSource;)Lnet/minecraft/client/OptionInstance;
private createSoundSliderOptionInstance(Ljava/lang/String;Lnet/minecraft/sounds/SoundSource;)Lnet/minecraft/client/OptionInstance;
public showSubtitles()Lnet/minecraft/client/OptionInstance;
public directionalAudio()Lnet/minecraft/client/OptionInstance;
public backgroundForChatOnly()Lnet/minecraft/client/OptionInstance;
public fullscreen()Lnet/minecraft/client/OptionInstance;
public exclusiveFullscreen()Lnet/minecraft/client/OptionInstance;
public macFullscreenMenuVisibility()Lnet/minecraft/client/OptionInstance;
public bobView()Lnet/minecraft/client/OptionInstance;
public toggleCrouch()Lnet/minecraft/client/OptionInstance;
public toggleSprint()Lnet/minecraft/client/OptionInstance;
public toggleAttack()Lnet/minecraft/client/OptionInstance;
public toggleUse()Lnet/minecraft/client/OptionInstance;
public sprintWindow()Lnet/minecraft/client/OptionInstance;
public hideMatchedNames()Lnet/minecraft/client/OptionInstance;
public showAutosaveIndicator()Lnet/minecraft/client/OptionInstance;
public onlyShowSecureChat()Lnet/minecraft/client/OptionInstance;
public saveChatDrafts()Lnet/minecraft/client/OptionInstance;
private setGraphicsPresetToCustom()V
public fov()Lnet/minecraft/client/OptionInstance;
public telemetryOptInExtra()Lnet/minecraft/client/OptionInstance;
public screenEffectScale()Lnet/minecraft/client/OptionInstance;
public fovEffectScale()Lnet/minecraft/client/OptionInstance;
public darknessEffectScale()Lnet/minecraft/client/OptionInstance;
public glintSpeed()Lnet/minecraft/client/OptionInstance;
public glintStrength()Lnet/minecraft/client/OptionInstance;
public damageTiltStrength()Lnet/minecraft/client/OptionInstance;
public gamma()Lnet/minecraft/client/OptionInstance;
public guiScale()Lnet/minecraft/client/OptionInstance;
public debugGuiScale()Lnet/minecraft/client/OptionInstance;
public particles()Lnet/minecraft/client/OptionInstance;
public narrator()Lnet/minecraft/client/OptionInstance;
public soundDevice()Lnet/minecraft/client/OptionInstance;
public onboardingAccessibilityFinished()V
public musicFrequency()Lnet/minecraft/client/OptionInstance;
public musicToast()Lnet/minecraft/client/OptionInstance;
public <init>(Lnet/minecraft/client/Minecraft;Ljava/io/File;)V
public getBackgroundOpacity(F)F
public getBackgroundColor(F)I
public getBackgroundColor(I)I
private processDumpedOptions(Lnet/minecraft/client/Options$OptionAccess;)V
private processOptions(Lnet/minecraft/client/Options$FieldAccess;)V
public load()V
private static isTrue(Ljava/lang/String;)Z
private static isFalse(Ljava/lang/String;)Z
private dataFix(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/CompoundTag;
public save()V
private getFullscreenVideoModeString()Ljava/lang/String;
public buildPlayerInformation()Lnet/minecraft/server/level/ClientInformation;
public broadcastOptions()V
public setModelPart(Lnet/minecraft/world/entity/player/PlayerModelPart;Z)V
public isModelPartEnabled(Lnet/minecraft/world/entity/player/PlayerModelPart;)Z
public getCloudStatus()Lnet/minecraft/client/CloudStatus;
public useNativeTransport()Z
public loadSelectedResourcePacks(Lnet/minecraft/server/packs/repository/PackRepository;)V
public getCameraType()Lnet/minecraft/client/CameraType;
public setCameraType(Lnet/minecraft/client/CameraType;)V
private static readListOfStrings(Ljava/lang/String;)Ljava/util/List;
public getFile()Ljava/io/File;
public dumpOptionsForReport()Ljava/lang/String;
public setServerRenderDistance(I)V
public getEffectiveRenderDistance()I
private static pixelValueLabel(Lnet/minecraft/network/chat/Component;I)Lnet/minecraft/network/chat/Component;
private static percentValueLabel(Lnet/minecraft/network/chat/Component;D)Lnet/minecraft/network/chat/Component;
public static genericValueLabel(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public static genericValueLabel(Lnet/minecraft/network/chat/Component;I)Lnet/minecraft/network/chat/Component;
public static genericValueOrOffLabel(Lnet/minecraft/network/chat/Component;I)Lnet/minecraft/network/chat/Component;
private static percentValueOrOffLabel(Lnet/minecraft/network/chat/Component;D)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$dumpOptionsForReport$0(Lcom/mojang/datafixers/util/Pair;)Ljava/lang/String;
private synthetic lambda$load$1(Ljava/lang/String;)V
private static synthetic lambda$load$0(Lnet/minecraft/nbt/CompoundTag;Ljava/lang/String;)V
private synthetic lambda$new$112(Ljava/lang/Integer;)V
private static synthetic lambda$new$111(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$110(Ljava/lang/Integer;)V
private static synthetic lambda$new$109(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$108(Lnet/minecraft/client/MusicToastDisplayState;)V
private static synthetic lambda$new$107(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/MusicToastDisplayState;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$106(Lnet/minecraft/client/MusicToastDisplayState;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$105(Lnet/minecraft/client/sounds/MusicManager$MusicFrequency;)V
private static synthetic lambda$new$104(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/sounds/MusicManager$MusicFrequency;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$103(Ljava/lang/String;)V
private static synthetic lambda$new$102(Ljava/lang/String;)Ljava/util/Optional;
private static synthetic lambda$new$101()Ljava/util/List;
private static synthetic lambda$new$100(Lnet/minecraft/network/chat/Component;Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$99(Lnet/minecraft/client/NarratorStatus;)V
private synthetic lambda$new$98(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/NarratorStatus;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$97(Lnet/minecraft/server/level/ParticleStatus;)V
private static synthetic lambda$new$96(Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/level/ParticleStatus;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$95(Ljava/lang/Integer;)V
private static synthetic lambda$new$94()I
private static synthetic lambda$new$93(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$92(Ljava/lang/Integer;)V
private static synthetic lambda$new$91()I
private static synthetic lambda$new$90(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$89(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$88(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$87(Ljava/lang/Integer;)Ljava/lang/Double;
private static synthetic lambda$new$86(Ljava/lang/Double;)Ljava/lang/Integer;
private static synthetic lambda$new$85(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$keyMappings$0(I)[Lnet/minecraft/client/KeyMapping;
private static synthetic lambda$new$84(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$83(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$82(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$81(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$80(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$79(Ljava/lang/Boolean;)V
private static synthetic lambda$new$78(Ljava/lang/Boolean;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$77(Ljava/lang/Boolean;)V
private static synthetic lambda$new$76(Lnet/minecraft/network/chat/Component;Ljava/lang/Boolean;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$75(Ljava/lang/Boolean;)V
private static synthetic lambda$new$74(Ljava/lang/Boolean;)Lnet/minecraft/client/gui/components/Tooltip;
private synthetic lambda$createSoundSliderOptionInstance$0(Lnet/minecraft/sounds/SoundSource;Ljava/lang/Double;)V
private synthetic lambda$new$73(Lnet/minecraft/sounds/SoundSource;)Lnet/minecraft/client/OptionInstance;
private static synthetic lambda$new$72(Lnet/minecraft/client/PresenceSharing;)V
private static synthetic lambda$new$71(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/PresenceSharing;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$70(Lnet/minecraft/client/PresenceSharing;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$69(Ljava/lang/Boolean;)V
private static synthetic lambda$new$68(Ljava/lang/Boolean;)V
private static synthetic lambda$new$67(Ljava/lang/Boolean;)V
private synthetic lambda$new$66(Ljava/lang/Boolean;)V
private static synthetic lambda$new$65(Ljava/lang/Boolean;)V
private static synthetic lambda$new$64(Ljava/lang/Boolean;)V
private static synthetic lambda$new$63(Ljava/lang/Boolean;)V
private static synthetic lambda$new$62(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$61(Ljava/lang/Integer;)V
private static synthetic lambda$new$60(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$59(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/AttackIndicatorStatus;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$58(Lnet/minecraft/client/TextureFilteringMethod;)V
private static synthetic lambda$new$57(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/TextureFilteringMethod;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$56(Lnet/minecraft/client/TextureFilteringMethod;)Lnet/minecraft/client/gui/components/Tooltip;
private synthetic lambda$new$55(Ljava/lang/Integer;)V
private static synthetic lambda$new$54(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$53(Ljava/lang/Integer;)V
private static synthetic lambda$new$52(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$51(Ljava/lang/Double;)I
private static synthetic lambda$new$50(I)Ljava/lang/Double;
private static synthetic lambda$new$49(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$48(Ljava/lang/Double;)V
private static synthetic lambda$new$47(Ljava/lang/Double;)I
private static synthetic lambda$new$46(I)Ljava/lang/Double;
private static synthetic lambda$new$45(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$44(Ljava/lang/Double;)V
private static synthetic lambda$new$43(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$42(Ljava/lang/Double;)V
private static synthetic lambda$new$41(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$40(Ljava/lang/Double;)V
private static synthetic lambda$new$39(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$38(Ljava/lang/Double;)V
private static synthetic lambda$new$37(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$36(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/entity/HumanoidArm;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$35(Ljava/lang/Boolean;)V
private static synthetic lambda$new$34(Ljava/lang/Double;)V
private synthetic lambda$new$33(Ljava/lang/Integer;)V
private static synthetic lambda$new$32(Ljava/lang/Double;)V
private static synthetic lambda$new$31(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$30(Lnet/minecraft/world/entity/player/ChatVisiblity;)V
private static synthetic lambda$new$29(Lnet/minecraft/network/chat/Component;Lnet/minecraft/world/entity/player/ChatVisiblity;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$28(Lnet/minecraft/client/PrioritizeChunkUpdates;)V
private static synthetic lambda$new$27(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/PrioritizeChunkUpdates;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$26(Lnet/minecraft/client/PrioritizeChunkUpdates;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$25(Ljava/lang/Double;)I
private static synthetic lambda$new$24(I)Ljava/lang/Double;
private static synthetic lambda$new$23(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$22(Ljava/lang/Boolean;)V
private synthetic lambda$new$21(Ljava/lang/Boolean;)V
private synthetic lambda$new$20(Ljava/lang/Boolean;)V
private synthetic lambda$new$19(Ljava/lang/Integer;)V
private static synthetic lambda$new$18(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$17(Ljava/lang/Integer;)V
private static synthetic lambda$new$16(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$15(Lnet/minecraft/client/CloudStatus;)V
private static synthetic lambda$new$14(Ljava/lang/Boolean;)Lnet/minecraft/client/CloudStatus;
private static synthetic lambda$new$13(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/CloudStatus;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$12(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/InactivityFpsLimit;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$11(Lnet/minecraft/client/InactivityFpsLimit;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$10(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/GraphicsPreset;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$new$9(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/PreferredGraphicsApi;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$8(Lnet/minecraft/client/PreferredGraphicsApi;)Lnet/minecraft/client/gui/components/Tooltip;
private static synthetic lambda$new$7(Ljava/lang/Integer;)V
private static synthetic lambda$new$6(Ljava/lang/Integer;)I
private static synthetic lambda$new$5(I)Ljava/lang/Integer;
private static synthetic lambda$new$4(Lnet/minecraft/network/chat/Component;Ljava/lang/Integer;)Lnet/minecraft/network/chat/Component;
private synthetic lambda$new$3(Ljava/lang/Double;)V
private static synthetic lambda$new$2(Ljava/lang/Double;)I
private static synthetic lambda$new$1(I)Ljava/lang/Double;
private static synthetic lambda$new$0(Lnet/minecraft/network/chat/Component;Ljava/lang/Double;)Lnet/minecraft/network/chat/Component;
static <clinit>()V
```
