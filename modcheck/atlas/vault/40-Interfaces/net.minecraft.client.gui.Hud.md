---
type: "interface"
fqcn: "net.minecraft.client.gui.Hud"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.Hud

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getChat` | `()Lnet/minecraft/client/gui/components/ChatComponent;` | exact | invokevirtual@10 in `ClientSuggestionProviderMixin.sendFeedback` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| injects_into | `extractAirBubbles` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/e` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `minecraft` | `Lnet/minecraft/client/Minecraft;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractHotbarAndDecorations` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractPlayerHealth` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractPlayerHealth` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractPlayerHealth` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractPlayerHealth` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (90 fields, 82 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final CROSSHAIR_SPRITE : Lnet/minecraft/resources/Identifier;
private static final CROSSHAIR_ATTACK_INDICATOR_FULL_SPRITE : Lnet/minecraft/resources/Identifier;
private static final CROSSHAIR_ATTACK_INDICATOR_BACKGROUND_SPRITE : Lnet/minecraft/resources/Identifier;
private static final CROSSHAIR_ATTACK_INDICATOR_PROGRESS_SPRITE : Lnet/minecraft/resources/Identifier;
private static final EFFECT_BACKGROUND_AMBIENT_SPRITE : Lnet/minecraft/resources/Identifier;
private static final EFFECT_BACKGROUND_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_SELECTION_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_OFFHAND_LEFT_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_OFFHAND_RIGHT_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_ATTACK_INDICATOR_BACKGROUND_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HOTBAR_ATTACK_INDICATOR_PROGRESS_SPRITE : Lnet/minecraft/resources/Identifier;
private static final ARMOR_EMPTY_SPRITE : Lnet/minecraft/resources/Identifier;
private static final ARMOR_HALF_SPRITE : Lnet/minecraft/resources/Identifier;
private static final ARMOR_FULL_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_EMPTY_HUNGER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_HALF_HUNGER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_FULL_HUNGER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_EMPTY_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_HALF_SPRITE : Lnet/minecraft/resources/Identifier;
private static final FOOD_FULL_SPRITE : Lnet/minecraft/resources/Identifier;
private static final AIR_SPRITE : Lnet/minecraft/resources/Identifier;
private static final AIR_POPPING_SPRITE : Lnet/minecraft/resources/Identifier;
private static final AIR_EMPTY_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HEART_VEHICLE_CONTAINER_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HEART_VEHICLE_FULL_SPRITE : Lnet/minecraft/resources/Identifier;
private static final HEART_VEHICLE_HALF_SPRITE : Lnet/minecraft/resources/Identifier;
private static final VIGNETTE_LOCATION : Lnet/minecraft/resources/Identifier;
public static final NAUSEA_LOCATION : Lnet/minecraft/resources/Identifier;
private static final SPYGLASS_SCOPE_LOCATION : Lnet/minecraft/resources/Identifier;
private static final POWDER_SNOW_OUTLINE_LOCATION : Lnet/minecraft/resources/Identifier;
private static final SCORE_DISPLAY_ORDER : Ljava/util/Comparator;
private static final DEMO_EXPIRED_TEXT : Lnet/minecraft/network/chat/Component;
private static final SAVING_TEXT : Lnet/minecraft/network/chat/Component;
private static final MIN_CROSSHAIR_ATTACK_SPEED : F
private static final EXPERIENCE_BAR_DISPLAY_TICKS : I
private static final NUM_HEARTS_PER_ROW : I
private static final LINE_HEIGHT : I
private static final SPACER : Ljava/lang/String;
private static final PORTAL_OVERLAY_ALPHA_MIN : F
private static final HEART_SIZE : I
private static final HEART_SEPARATION : I
private static final NUM_AIR_BUBBLES : I
private static final AIR_BUBBLE_SIZE : I
private static final AIR_BUBBLE_SEPERATION : I
private static final AIR_BUBBLE_POPPING_DURATION : I
private static final EMPTY_AIR_BUBBLE_DELAY_DURATION : I
private static final AIR_BUBBLE_POP_SOUND_VOLUME_BASE : F
private static final AIR_BUBBLE_POP_SOUND_VOLUME_INCREMENT : F
private static final AIR_BUBBLE_POP_SOUND_PITCH_BASE : F
private static final AIR_BUBBLE_POP_SOUND_PITCH_INCREMENT : F
private static final NUM_AIR_BUBBLE_POPPED_BEFORE_SOUND_VOLUME_INCREASE : I
private static final NUM_AIR_BUBBLE_POPPED_BEFORE_SOUND_PITCH_INCREASE : I
private static final AUTOSAVE_FADE_SPEED_FACTOR : F
private static final SAVING_INDICATOR_WIDTH_PADDING_RIGHT : I
private static final SAVING_INDICATOR_HEIGHT_PADDING_BOTTOM : I
private final random : Lnet/minecraft/util/RandomSource;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final chat : Lnet/minecraft/client/gui/components/ChatComponent;
private final waypointStyles : Lnet/minecraft/client/resources/WaypointStyleManager;
private isHidden : Z
private tickCount : I
private overlayMessageString : Lnet/minecraft/network/chat/Component;
private overlayMessageTime : I
private animateOverlayMessageColor : Z
public vignetteBrightness : F
private toolHighlightTimer : I
private lastToolHighlight : Lnet/minecraft/world/item/ItemStack;
private final debugOverlay : Lnet/minecraft/client/gui/components/DebugScreenOverlay;
private final subtitleOverlay : Lnet/minecraft/client/gui/components/SubtitleOverlay;
private final spectatorGui : Lnet/minecraft/client/gui/components/spectator/SpectatorGui;
private final tabList : Lnet/minecraft/client/gui/components/PlayerTabOverlay;
private final bossOverlay : Lnet/minecraft/client/gui/components/BossHealthOverlay;
private titleTime : I
private title : Lnet/minecraft/network/chat/Component;
private subtitle : Lnet/minecraft/network/chat/Component;
private titleFadeInTime : I
private titleStayTime : I
private titleFadeOutTime : I
private lastHealth : I
private displayHealth : I
private lastHealthTime : J
private healthBlinkTime : J
private lastBubblePopSoundPlayed : I
private deferredSubtitles : Ljava/lang/Runnable;
private autosaveIndicatorValue : F
private lastAutosaveIndicatorValue : F
private contextualInfoBar : Lcom/mojang/datafixers/util/Pair;
private final contextualInfoBars : Ljava/util/Map;
private scopeScale : F
public <init>(Lnet/minecraft/client/Minecraft;)V
public registerReloadListeners(Lnet/minecraft/server/packs/resources/ReloadableResourceManager;)V
public toggle()V
public isHidden()Z
public resetTitleTimes()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractBossOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
public extractDebugOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private extractSubtitleOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Z)V
public extractDeferredSubtitles()V
private extractCameraOverlays(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractSleepOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractOverlayMessage(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractTitle(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractChat(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractScoreboardSidebar(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractTabList(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractCrosshair(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private canRenderCrosshairForSpectator(Lnet/minecraft/world/phys/HitResult;)Z
private extractEffects(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
public static getMobEffectSprite(Lnet/minecraft/core/Holder;)Lnet/minecraft/resources/Identifier;
private extractHotbarAndDecorations(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractItemHotbar(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private extractSelectedItemName(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private extractDemoOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private displayScoreboardSidebar(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/scores/Objective;)V
private getCameraPlayer()Lnet/minecraft/world/entity/player/Player;
private getPlayerVehicleWithHealth()Lnet/minecraft/world/entity/LivingEntity;
private getVehicleMaxHearts(Lnet/minecraft/world/entity/LivingEntity;)I
private getVisibleVehicleHeartRows(I)I
private extractPlayerHealth(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private static extractArmor(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/player/Player;IIII)V
private extractHearts(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/player/Player;IIIIFIIIZ)V
private extractHeart(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/gui/Hud$HeartType;IIZZZ)V
private extractAirBubbles(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/player/Player;III)V
private getAirBubbleYLine(II)I
private static getCurrentAirSupplyBubble(III)I
private static getEmptyBubbleDelayDuration(IZ)I
private playAirBubblePoppedSound(ILnet/minecraft/world/entity/player/Player;I)V
private extractFood(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/player/Player;II)V
private extractVehicleHealth(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private extractTextureOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/resources/Identifier;F)V
private extractSpyglassOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V
private updateVignetteBrightness(Lnet/minecraft/world/entity/Entity;)V
private extractVignette(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/world/entity/Entity;)V
private extractPortalOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V
private extractConfusionOverlay(Lnet/minecraft/client/gui/GuiGraphicsExtractor;F)V
private extractSlot(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IILnet/minecraft/client/DeltaTracker;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/item/ItemStack;I)V
public tick(Z)V
private tick()V
private tickAutosaveIndicator()V
public setNowPlaying(Lnet/minecraft/network/chat/Component;)V
public setOverlayMessage(Lnet/minecraft/network/chat/Component;Z)V
public setTimes(III)V
public setSubtitle(Lnet/minecraft/network/chat/Component;)V
public setTitle(Lnet/minecraft/network/chat/Component;)V
public clearTitles()V
public getChat()Lnet/minecraft/client/gui/components/ChatComponent;
public getWaypointStyles()Lnet/minecraft/client/resources/WaypointStyleManager;
public getGuiTicks()I
public getFont()Lnet/minecraft/client/gui/Font;
public getSpectatorGui()Lnet/minecraft/client/gui/components/spectator/SpectatorGui;
public getTabList()Lnet/minecraft/client/gui/components/PlayerTabOverlay;
public onDisconnected()V
public getBossOverlay()Lnet/minecraft/client/gui/components/BossHealthOverlay;
public getDebugOverlay()Lnet/minecraft/client/gui/components/DebugScreenOverlay;
public clearCache()V
public extractSavingIndicator(Lnet/minecraft/client/gui/GuiGraphicsExtractor;Lnet/minecraft/client/DeltaTracker;)V
private willPrioritizeExperienceInfo()Z
private willPrioritizeJumpInfo()Z
private nextContextualInfoState()Lnet/minecraft/client/gui/Hud$ContextualInfo;
private static synthetic lambda$displayScoreboardSidebar$2(I)[Lnet/minecraft/client/gui/Hud$1DisplayEntry;
private synthetic lambda$displayScoreboardSidebar$1(Lnet/minecraft/world/scores/Scoreboard;Lnet/minecraft/network/chat/numbers/NumberFormat;Lnet/minecraft/world/scores/PlayerScoreEntry;)Lnet/minecraft/client/gui/Hud$1DisplayEntry;
private static synthetic lambda$displayScoreboardSidebar$0(Lnet/minecraft/world/scores/PlayerScoreEntry;)Z
private static synthetic lambda$getMobEffectSprite$0(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resources/Identifier;
private static synthetic lambda$extractCameraOverlays$0(Ljava/lang/String;)Ljava/lang/String;
private synthetic lambda$extractSubtitleOverlay$0(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private static synthetic lambda$new$3(Lnet/minecraft/client/Minecraft;)Lnet/minecraft/client/gui/contextualbar/ContextualBar;
private static synthetic lambda$new$2(Lnet/minecraft/client/Minecraft;)Lnet/minecraft/client/gui/contextualbar/ContextualBar;
private static synthetic lambda$new$1(Lnet/minecraft/client/Minecraft;)Lnet/minecraft/client/gui/contextualbar/ContextualBar;
private static synthetic lambda$new$0()Lnet/minecraft/client/gui/contextualbar/ContextualBar;
static <clinit>()V
```
