---
type: "interface"
fqcn: "net.minecraft.client.Minecraft"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Minecraft

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createWorldOpenFlows()Lnet/minecraft/client/gui/screens/worldselection/WorldOpen` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnectWithSavingScreen()V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `doWorldLoad(Lnet/minecraft/world/level/storage/LevelStorageSource$Level` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `execute(Ljava/lang/Runnable;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `execute(Ljava/lang/Runnable;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getAtlasManager()Lnet/minecraft/client/resources/model/sprite/AtlasManager;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getAtlasManager()Lnet/minecraft/client/resources/model/sprite/AtlasManager;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getConnection()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection()Lnet/minecraft/client/multiplayer/ClientPacketListener;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker()Lnet/minecraft/client/DeltaTracker;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getDeltaTracker()Lnet/minecraft/client/DeltaTracker;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getEntityRenderDispatcher()Lnet/minecraft/client/renderer/entity/EntityRenderDispatch` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getGameProfile()Lcom/mojang/authlib/GameProfile;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-menu-api-v1|fabric-menu-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getInstance()Lnet/minecraft/client/Minecraft;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `getLevelSource()Lnet/minecraft/world/level/storage/LevelStorageSource;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getNarrator()Lnet/minecraft/client/GameNarrator;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `getResourcePackRepository()Lnet/minecraft/server/packs/repository/PackRepository;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `getRunningThread()Ljava/lang/Thread;` | `` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| calls | `getSingleplayerServer()Lnet/minecraft/client/server/IntegratedServer;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow()Lcom/mojang/blaze3d/platform/Window;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow()Lcom/mojang/blaze3d/platform/Window;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow()Lcom/mojang/blaze3d/platform/Window;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow()Lcom/mojang/blaze3d/platform/Window;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWindow()Lcom/mojang/blaze3d/platform/Window;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isLocalServer()Z` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `isSameThread()Z` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread()Z` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `packetProcessor()Lnet/minecraft/network/PacketProcessor;` | `` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `packetProcessor()Lnet/minecraft/network/PacketProcessor;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `setScreenAndShow(Lnet/minecraft/client/gui/screens/Screen;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `stop()V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `stop()V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at INVOKE Lcom/mojang/blaze3d/systems/RenderSystem;getBackendDescription` | client | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at INVOKE Ljava/lang/Thread;currentThread()Ljava/lang/Thread;` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `continueAttack` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `disconnect(Lnet/minecraft/client/gui/screens/Screen;Z)V` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | `@Inject at INVOKE Lnet/minecraft/client/Minecraft;renderFrame(Z)V` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `disconnect(Lnet/minecraft/client/gui/screens/Screen;ZZ)V` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `doWorldLoad` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `@Inject at INVOKE Lnet/minecraft/client/Minecraft;managedBlock(Ljava/util/functi` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `@Inject at INVOKE Lnet/minecraft/client/gui/screens/LevelLoadingScreen;tick()V` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `doWorldLoad` | `@Inject at INVOKE Lnet/minecraft/client/Minecraft;renderFrame(Z)V` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `emergencySave` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `exitWorldAndClose` | `@Inject at INVOKE Lorg/slf4j/Logger;info(Ljava/lang/String;)V` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `exitWorldAndClose` | `@Inject at INVOKE Lnet/minecraft/client/gui/screens/Screen;removed()V` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |
| injects_into | `getInstance` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `handleKeybinds` | `@Inject at INVOKE Lnet/minecraft/client/player/LocalPlayer;isUsingItem()Z` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `run` | `@Inject at FIELD Lnet/minecraft/client/Minecraft;gameThread:Ljava/lang/Thread;` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `runTick` | `@Inject at INVOKE Lnet/minecraft/client/Minecraft;runAllTasks()V` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `startAttack` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `startUseItem` | `@Inject at INVOKE Lnet/minecraft/client/multiplayer/MultiPlayerGameMode;interact` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `tick` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `tick` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tick` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `updateLevelInEngines(Lnet/minecraft/client/multiplayer/ClientLevel;Z)V` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `fontLnet/minecraft/client/gui/Font;` | `` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `gameRendererLnet/minecraft/client/renderer/GameRenderer;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `guiLnet/minecraft/client/gui/Gui;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `hitResultLnet/minecraft/world/phys/HitResult;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `levelLnet/minecraft/client/multiplayer/ClientLevel;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `levelLnet/minecraft/client/multiplayer/ClientLevel;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `levelExtractorLnet/minecraft/client/renderer/extract/LevelExtractor;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `levelRendererLnet/minecraft/client/renderer/LevelRenderer;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `optionsLnet/minecraft/client/Options;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `playerLnet/minecraft/client/player/LocalPlayer;` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| reads | `playerLnet/minecraft/client/player/LocalPlayer;` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `playerLnet/minecraft/client/player/LocalPlayer;` | `` | client | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (356, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.Minecraft extends net.minecraft.util.thread.ReentrantBlockableEventLoop<java.lang.Runnable> implements com.mojang.blaze3d.platform.WindowEventHandler {
    private static net.minecraft.client.Minecraft instance;
    private static final org.slf4j.Logger LOGGER;
    private static final com.mojang.jtracy.SectionCategory TRACY_CURRENT_LEVEL;
    private static final int MAX_TICKS_PER_UPDATE;
    public static final net.minecraft.resources.Identifier DEFAULT_FONT;
    private static final net.minecraft.resources.Identifier REGIONAL_COMPLIANCIES;
    private static final java.util.concurrent.CompletableFuture<net.minecraft.util.Unit> RESOURCE_RELOAD_INITIAL_TASK;
    public static final java.lang.String UPDATE_DRIVERS_ADVICE;
    private final long canary;
    private final java.nio.file.Path resourcePackDirectory;
    private final java.util.concurrent.CompletableFuture<com.mojang.authlib.services.ProfileResult> profileFuture;
    private final net.minecraft.client.renderer.texture.TextureManager textureManager;
    private final net.minecraft.client.renderer.ShaderManager shaderManager;
    private final com.mojang.datafixers.DataFixer fixerUpper;
    private final com.mojang.blaze3d.platform.MonitorManager monitorManager;
    private final com.mojang.blaze3d.platform.Window window;
    private final com.mojang.renderpearl.api.device.GpuSurface windowSurface;
    private final com.mojang.blaze3d.platform.SDLEventHandler sdlEventHandler;
    private final com.mojang.blaze3d.platform.TextInputManager textInputManager;
    private final net.minecraft.client.DeltaTracker$Timer deltaTracker;
    public final net.minecraft.client.renderer.extract.LevelExtractor levelExtractor;
    public final net.minecraft.client.renderer.LevelRenderer levelRenderer;
    private final net.minecraft.client.renderer.entity.EntityRenderDispatcher entityRenderDispatcher;
    private final net.minecraft.client.renderer.item.ItemModelResolver itemModelResolver;
    private final net.minecraft.client.renderer.MapRenderer mapRenderer;
    public final net.minecraft.client.particle.ParticleEngine particleEngine;
    private final net.minecraft.client.User user;
    public final net.minecraft.client.gui.Font font;
    public final net.minecraft.client.gui.Font fontFilterFishy;
    public final net.minecraft.client.renderer.GameRenderer gameRenderer;
    public final net.minecraft.client.gui.Gui gui;
    public final net.minecraft.client.Options options;
    public final net.minecraft.client.gui.components.debug.DebugScreenEntryList debugEntries;
    private final net.minecraft.client.HotbarManager hotbarManager;
    public final net.minecraft.client.MouseHandler mouseHandler;
    public final net.minecraft.client.KeyboardHandler keyboardHandler;
    private net.minecraft.client.InputType lastInputType;
    public final java.io.File gameDirectory;
    private final java.lang.String launchedVersion;
    private final java.net.Proxy proxy;
    private final boolean offlineDeveloperMode;
    private final net.minecraft.world.level.storage.LevelStorageSource levelSource;
    private final boolean demo;
    private final boolean allowsMultiplayer;
    private final boolean allowsChat;
    private final net.minecraft.server.packs.resources.ReloadableResourceManager resourceManager;
    private final net.minecraft.server.packs.VanillaPackResources vanillaPackResources;
    private final net.minecraft.client.resources.server.DownloadedPackSource downloadedPackSource;
    private final net.minecraft.server.packs.repository.PackRepository resourcePackRepository;
    private final net.minecraft.client.resources.language.LanguageManager languageManager;
    private final net.minecraft.client.color.block.BlockColors blockColors;
    private final com.mojang.blaze3d.TracyFrameCapture tracyFrameCapture;
    private final net.minecraft.client.sounds.SoundManager soundManager;
    private final net.minecraft.client.sounds.MusicManager musicManager;
    private final net.minecraft.client.gui.font.FontManager fontManager;
    private final net.minecraft.client.renderer.GpuWarnlistManager gpuWarnlistManager;
    private final net.minecraft.client.PeriodicNotificationManager regionalCompliancies;
    private final com.mojang.authlib.minecraft.UserApiService userApiService;
    private final java.util.concurrent.CompletableFuture<com.mojang.authlib.minecraft.UserApiService$UserProperties> userPropertiesFuture;
    private final net.minecraft.client.resources.SkinManager skinManager;
    private final net.minecraft.client.resources.model.sprite.AtlasManager atlasManager;
    private final net.minecraft.client.resources.model.ModelManager modelManager;
    private final net.minecraft.client.resources.palette.PalettedTextureManager palettedTextureManager;
    private final net.minecraft.client.resources.MapTextureManager mapTextureManager;
    private final net.minecraft.client.tutorial.Tutorial tutorial;
    private final net.minecraft.client.gui.screens.social.PlayerSocialManager playerSocialManager;
    private final net.minecraft.client.gui.screens.social.RemoteFriendListUpdateHandler remoteFriendListUpdateHandler;
    private final net.minecraft.client.renderer.blockentity.BlockEntityRenderDispatcher blockEntityRenderDispatcher;
    private final net.minecraft.client.telemetry.ClientTelemetryManager telemetryManager;
    private final net.minecraft.client.multiplayer.ProfileKeyPairManager profileKeyPairManager;
    private final com.mojang.realmsclient.gui.RealmsDataFetcher realmsDataFetcher;
    private final net.minecraft.client.quickplay.QuickPlayLog quickPlayLog;
    private final net.minecraft.server.Services services;
    private final net.minecraft.client.renderer.PlayerSkinRenderCache playerSkinRenderCache;
    private final com.mojang.blaze3d.systems.TimerQuery timerQuery;
    public net.minecraft.client.multiplayer.MultiPlayerGameMode gameMode;
    public net.minecraft.client.multiplayer.ClientLevel level;
    public net.minecraft.client.player.LocalPlayer player;
    private net.minecraft.client.server.IntegratedServer singleplayerServer;
    private net.minecraft.network.Connection pendingConnection;
    private boolean isLocalServer;
    private com.mojang.jtracy.Section tracyLevelSection;
    public net.minecraft.world.entity.Entity crosshairPickEntity;
    public net.minecraft.world.phys.HitResult hitResult;
    private int rightClickDelay;
    protected int missTime;
    private volatile boolean pause;
    private long lastNanoTime;
    private long lastTime;
    private int frames;
    private java.lang.Thread gameThread;
    private volatile boolean running;
    private static int fps;
    private long frameTimeNs;
    private final com.mojang.blaze3d.platform.FramerateLimitTracker framerateLimitTracker;
    public boolean multiDrawIndirect;
    public boolean wireframe;
    public boolean smartCull;
    private long lastActiveTime;
    private java.util.concurrent.CompletableFuture<java.lang.Void> pendingReload;
    private int fpsPieRenderTicks;
    private final net.minecraft.util.profiling.ContinuousProfiler fpsPieProfiler;
    private net.minecraft.util.profiling.metrics.profiling.MetricsRecorder metricsRecorder;
    private final net.minecraft.client.ResourceLoadStateTracker reloadStateTracker;
    private long savedCpuDuration;
    private double gpuUtilization;
    private final net.minecraft.client.GameNarrator narrator;
    private net.minecraft.client.multiplayer.chat.report.ReportingContext reportingContext;
    private final net.minecraft.world.level.validation.DirectoryValidator directoryValidator;
    private boolean gameLoadFinished;
    private final long clientStartTimeMs;
    private long clientTickCount;
    private final net.minecraft.network.PacketProcessor packetProcessor;
    private final net.minecraft.gizmos.SimpleGizmoCollector perTickGizmos;
    private java.util.List<net.minecraft.gizmos.SimpleGizmoCollector$GizmoInstance> drainedLatestTickGizmos;
    private boolean windowSurfaceNeedsReconfiguring;
    private boolean surfaceIsInvalid;
    private com.mojang.renderpearl.api.device.BackendCreationException backendCreationException;
    public net.minecraft.client.Minecraft(net.minecraft.client.main.GameConfig);
    public boolean hasShiftDown();
    public boolean hasControlDown();
    public boolean hasAltDown();
    private void onResourceLoadFinished(net.minecraft.client.GameLoadCookie);
    private void onGameLoadFinished(net.minecraft.client.GameLoadCookie);
    public boolean isGameLoadFinished();
    private static boolean countryEqualsISO3(java.lang.Object);
    public void updateTitle();
    private java.lang.String createTitle();
    private static com.mojang.authlib.minecraft.UserApiService createUserApiService(com.mojang.authlib.services.MinecraftServicesDiscoveryService, net.minecraft.client.main.GameConfig);
    public boolean isOfflineDeveloperMode();
    public static net.minecraft.util.ModCheck checkModStatus();
    private void loadCriticalShaders();
    private void rollbackResourcePacks(java.lang.Throwable, net.minecraft.client.GameLoadCookie);
    public void clearResourcePacksOnError(java.lang.Throwable, net.minecraft.network.chat.Component, net.minecraft.client.GameLoadCookie);
    private void abortResourcePackRecovery();
    private void addResourcePackLoadFailToast(net.minecraft.network.chat.Component);
    public void triggerResourcePackRecovery(java.lang.Exception);
    public void run();
    void updateFontOptions();
    public java.lang.String getLaunchedVersion();
    public void delayCrash(net.minecraft.CrashReport);
    public void emergencySaveAndCrash(net.minecraft.CrashReport);
    public static void saveReport(java.io.File, net.minecraft.CrashReport);
    public static int saveReport(java.io.File, net.minecraft.CrashReport, int);
    public static void crash(net.minecraft.client.Minecraft, java.io.File, net.minecraft.CrashReport, int);
    public static int saveReportAndShutdownSoundManager(net.minecraft.client.Minecraft, java.io.File, net.minecraft.CrashReport, int);
    public boolean isEnforceUnicode();
    public java.util.concurrent.CompletableFuture<java.lang.Void> reloadResourcePacks();
    private java.util.concurrent.CompletableFuture<java.lang.Void> reloadResourcePacks(boolean, net.minecraft.client.GameLoadCookie);
    private void selfTest();
    public net.minecraft.world.level.storage.LevelStorageSource getLevelSource();
    public void exitWorldAndClose();
    public void close();
    private void runTick(boolean);
    public void renderFrame(boolean);
    private void pauseIfInactive();
    private net.minecraft.util.profiling.ProfilerFiller constructProfiler(boolean, net.minecraft.util.profiling.SingleTickProfiler);
    private void finishProfilers(boolean, net.minecraft.util.profiling.SingleTickProfiler);
    public void framebufferSizeChanged();
    public void resizeGui();
    public void cursorEntered();
    public void fullscreenStateChanged(boolean);
    public int getFps();
    public long getFrameTimeNs();
    public void sendLowDiskSpaceWarning();
    private void emergencySave();
    public boolean debugClientMetricsStart(java.util.function.Consumer<net.minecraft.network.chat.Component>);
    private void debugClientMetricsStop();
    private void debugClientMetricsCancel();
    private java.nio.file.Path archiveProfilingReport(net.minecraft.SystemReport, java.util.List<java.nio.file.Path>);
    public void stop();
    public boolean isRunning();
    public void pauseGame(boolean);
    private void continueAttack(boolean);
    private boolean startAttack();
    private void startUseItem();
    public net.minecraft.client.sounds.MusicManager getMusicManager();
    public void tick();
    private boolean isLevelRunningNormally();
    public boolean isMultiplayerServer();
    private void handleKeybinds();
    private java.util.Optional<net.minecraft.core.Holder<net.minecraft.server.dialog.Dialog>> getQuickActionsDialog();
    public net.minecraft.client.telemetry.ClientTelemetryManager getTelemetryManager();
    public net.minecraft.util.profiling.metrics.profiling.MetricsRecorder getMetricsRecorder();
    public double getGpuUtilization();
    public net.minecraft.client.multiplayer.ProfileKeyPairManager getProfileKeyPairManager();
    public net.minecraft.client.gui.screens.worldselection.WorldOpenFlows createWorldOpenFlows();
    public void doWorldLoad(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, net.minecraft.server.WorldStem, java.util.Optional<net.minecraft.world.level.gamerules.GameRules>, boolean);
    public void setLevel(net.minecraft.client.multiplayer.ClientLevel);
    public void disconnectFromWorld(net.minecraft.network.chat.Component);
    public void disconnectWithSavingScreen();
    public void disconnectWithProgressScreen();
    public void disconnectWithProgressScreen(boolean);
    public void disconnect(net.minecraft.client.gui.screens.Screen, boolean);
    public void disconnect(net.minecraft.client.gui.screens.Screen, boolean, boolean);
    public void clearDownloadedResourcePacks();
    public void clearClientLevel(net.minecraft.client.gui.screens.Screen);
    public void setScreenAndShow(net.minecraft.client.gui.screens.Screen);
    private void updateLevelInEngines(net.minecraft.client.multiplayer.ClientLevel);
    private void updateLevelInEngines(net.minecraft.client.multiplayer.ClientLevel, boolean);
    private com.mojang.authlib.minecraft.UserApiService$UserProperties userProperties();
    public boolean telemetryOptInExtra();
    public boolean extraTelemetryAvailable();
    public boolean allowsTelemetry();
    public boolean allowsMultiplayer();
    public boolean allowsRealms();
    public boolean friendsEnabled();
    public boolean handleGlobalKeyPress(com.mojang.blaze3d.platform.InputConstants$Key, boolean);
    private void toggleFullscreen();
    private boolean toggleFriendsScreen();
    public boolean allowFriendRequests();
    public boolean allowChatOnlyWithFriend();
    public com.mojang.authlib.minecraft.BanDetails multiplayerBan();
    public boolean isNameBanned();
    public boolean isBlocked(java.util.UUID);
    public boolean isFriendOnlyRestricted(java.util.UUID);
    private boolean isLocalOrUnknownPlayer(java.util.UUID);
    public net.minecraft.client.multiplayer.chat.ChatAbilities computeChatAbilities();
    public final boolean isDemo();
    public final boolean canSwitchGameMode();
    public net.minecraft.client.multiplayer.ClientPacketListener getConnection();
    private void pickBlockOrEntity();
    public net.minecraft.CrashReport fillReport(net.minecraft.CrashReport);
    public static void fillReport(net.minecraft.client.Minecraft, net.minecraft.client.resources.language.LanguageManager, java.lang.String, net.minecraft.client.Options, net.minecraft.CrashReport);
    private static java.lang.String formatSeconds(double);
    private void fillUptime(net.minecraft.CrashReportCategory);
    private static net.minecraft.SystemReport fillSystemReport(net.minecraft.SystemReport, net.minecraft.client.Minecraft, net.minecraft.client.resources.language.LanguageManager, java.lang.String, net.minecraft.client.Options);
    public static net.minecraft.client.Minecraft getInstance();
    public java.util.concurrent.CompletableFuture<java.lang.Void> delayTextureReload();
    public void updateReportEnvironment(net.minecraft.client.multiplayer.chat.report.ReportEnvironment);
    public net.minecraft.client.multiplayer.ServerData getCurrentServer();
    public boolean isLocalServer();
    public boolean hasSingleplayerServer();
    public net.minecraft.client.server.IntegratedServer getSingleplayerServer();
    public boolean isLocalPlayer(java.util.UUID);
    public net.minecraft.client.User getUser();
    public com.mojang.authlib.services.ProfileResult getProfileResult();
    public com.mojang.authlib.GameProfile getGameProfile();
    public java.net.Proxy getProxy();
    public net.minecraft.client.renderer.texture.TextureManager getTextureManager();
    public net.minecraft.client.renderer.ShaderManager getShaderManager();
    public net.minecraft.server.packs.resources.ResourceManager getResourceManager();
    public net.minecraft.server.packs.repository.PackRepository getResourcePackRepository();
    public net.minecraft.server.packs.VanillaPackResources getVanillaPackResources();
    public net.minecraft.client.resources.server.DownloadedPackSource getDownloadedPackSource();
    public java.nio.file.Path getResourcePackDirectory();
    public net.minecraft.client.resources.language.LanguageManager getLanguageManager();
    public boolean isPaused();
    public net.minecraft.client.renderer.GpuWarnlistManager getGpuWarnlistManager();
    public net.minecraft.client.sounds.SoundManager getSoundManager();
    public net.minecraft.sounds.Music getSituationalMusic();
    public float getMusicVolume();
    public net.minecraft.server.Services services();
    public net.minecraft.client.resources.SkinManager getSkinManager();
    public net.minecraft.world.entity.Entity getCameraEntity();
    public void setCameraEntity(net.minecraft.world.entity.Entity);
    public boolean shouldEntityAppearGlowing(net.minecraft.world.entity.Entity);
    public java.lang.Thread getRunningThread();
    public java.lang.Runnable wrapRunnable(java.lang.Runnable);
    protected boolean shouldRun(java.lang.Runnable);
    public net.minecraft.client.renderer.entity.EntityRenderDispatcher getEntityRenderDispatcher();
    public net.minecraft.client.renderer.blockentity.BlockEntityRenderDispatcher getBlockEntityRenderDispatcher();
    public net.minecraft.client.renderer.MapRenderer getMapRenderer();
    public com.mojang.datafixers.DataFixer getFixerUpper();
    public net.minecraft.client.DeltaTracker getDeltaTracker();
    public net.minecraft.client.color.block.BlockColors getBlockColors();
    public boolean showOnlyReducedInfo();
    public net.minecraft.client.tutorial.Tutorial getTutorial();
    public boolean isWindowActive();
    public net.minecraft.client.HotbarManager getHotbarManager();
    public net.minecraft.client.resources.model.ModelManager getModelManager();
    public net.minecraft.client.resources.model.sprite.AtlasManager getAtlasManager();
    public net.minecraft.client.resources.palette.PalettedTextureManager getPalettedTextureManager();
    public net.minecraft.client.resources.MapTextureManager getMapTextureManager();
    public net.minecraft.network.chat.Component grabPanoramixScreenshot(java.io.File);
    public net.minecraft.client.gui.screens.social.PlayerSocialManager getPlayerSocialManager();
    public com.mojang.blaze3d.platform.Window getWindow();
    public com.mojang.blaze3d.platform.TextInputManager textInputManager();
    public void onTextInputFocusChange(net.minecraft.client.gui.components.events.GuiEventListener, boolean);
    public com.mojang.renderpearl.api.device.GpuSurface windowSurface();
    public com.mojang.blaze3d.platform.FramerateLimitTracker getFramerateLimitTracker();
    public net.minecraft.client.gui.components.DebugScreenOverlay getDebugOverlay();
    public void updateMaxMipLevel(int);
    public net.minecraft.client.model.geom.EntityModelSet getEntityModels();
    public boolean isTextFilteringEnabled();
    public void prepareForMultiplayer();
    public net.minecraft.client.InputType getLastInputType();
    public void setLastInputType(net.minecraft.client.InputType);
    public net.minecraft.client.GameNarrator getNarrator();
    public net.minecraft.client.multiplayer.chat.report.ReportingContext getReportingContext();
    public com.mojang.realmsclient.gui.RealmsDataFetcher realmsDataFetcher();
    public net.minecraft.client.quickplay.QuickPlayLog quickPlayLog();
    public net.minecraft.world.level.validation.DirectoryValidator directoryValidator();
    public net.minecraft.client.renderer.PlayerSkinRenderCache playerSkinRenderCache();
    private float getTickTargetMillis(float);
    public net.minecraft.client.renderer.item.ItemModelResolver getItemModelResolver();
    public boolean canInterruptScreen();
    public void invalidateSurfaceConfiguration();
    public static java.lang.String getLauncherBrand();
    public net.minecraft.network.PacketProcessor packetProcessor();
    public net.minecraft.gizmos.Gizmos$TemporaryCollection collectPerTickGizmos();
    public java.util.Collection<net.minecraft.gizmos.SimpleGizmoCollector$GizmoInstance> getPerTickGizmos();
    private void pick(float);
    public void showDebugChat(net.minecraft.network.chat.Component);
    private static net.minecraft.network.chat.Style lambda$grabPanoramixScreenshot$1(java.io.File, net.minecraft.network.chat.Style);
    private static void lambda$grabPanoramixScreenshot$0(net.minecraft.network.chat.Component);
    private static java.util.concurrent.CompletionStage lambda$delayTextureReload$0(java.util.concurrent.CompletableFuture);
    private static java.lang.Object lambda$fillSystemReport$11() throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$10() throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$9(net.minecraft.client.resources.language.LanguageManager) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$8(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$7(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$6(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$5(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$4() throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$3(com.mojang.renderpearl.api.device.GpuDevice) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$2(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$1(net.minecraft.client.Minecraft) throws java.lang.Exception;
    private static java.lang.Object lambda$fillSystemReport$0(java.lang.String) throws java.lang.Exception;
    private java.lang.String lambda$fillUptime$3() throws java.lang.Exception;
    private static java.lang.String lambda$fillUptime$2() throws java.lang.Exception;
    private java.lang.String lambda$fillUptime$1() throws java.lang.Exception;
    private static java.lang.String lambda$fillUptime$0() throws java.lang.Exception;
    private void lambda$toggleFriendsScreen$0(net.minecraft.client.gui.screens.Screen);
    private static void lambda$doWorldLoad$3(net.minecraft.network.chat.Component);
    private static boolean lambda$doWorldLoad$2(long);
    private static java.lang.String lambda$doWorldLoad$1(net.minecraft.server.WorldStem) throws java.lang.Exception;
    private net.minecraft.client.server.IntegratedServer lambda$doWorldLoad$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, net.minecraft.server.WorldStem, java.util.Optional, net.minecraft.server.level.progress.LevelLoadListener, java.lang.Thread);
    private static java.util.Optional lambda$getQuickActionsDialog$0(net.minecraft.core.Registry, net.minecraft.core.HolderSet$Named);
    private void lambda$handleKeybinds$0(net.minecraft.core.Holder);
    private static boolean lambda$tick$0();
    private void lambda$debugClientMetricsStart$9(java.util.function.Consumer, net.minecraft.util.profiling.ProfileResults);
    private static void lambda$debugClientMetricsStart$8(net.minecraft.util.profiling.ProfileResults);
    private static void lambda$debugClientMetricsStart$7(java.util.function.Consumer, java.util.concurrent.CompletableFuture, java.util.concurrent.CompletableFuture);
    private static void lambda$debugClientMetricsStart$6(java.util.function.Consumer, java.nio.file.Path);
    private void lambda$debugClientMetricsStart$5(net.minecraft.SystemReport, java.util.function.Consumer, java.util.List);
    private void lambda$debugClientMetricsStart$2(java.util.function.Consumer, java.nio.file.Path);
    private static void lambda$debugClientMetricsStart$4(java.util.function.Consumer, net.minecraft.network.chat.Component);
    private static net.minecraft.network.chat.Style lambda$debugClientMetricsStart$3(java.nio.file.Path, net.minecraft.network.chat.Style);
    private void lambda$debugClientMetricsStart$0(java.util.function.Consumer, net.minecraft.util.profiling.ProfileResults);
    private static void lambda$debugClientMetricsStart$1(java.util.function.Consumer, double, int);
    private void lambda$sendLowDiskSpaceWarning$0();
    private static void lambda$runTick$0(java.util.concurrent.CompletableFuture);
    private void lambda$reloadResourcePacks$0(boolean, net.minecraft.client.GameLoadCookie, java.util.concurrent.CompletableFuture, java.util.Optional);
    private void lambda$reloadResourcePacks$2(java.util.concurrent.CompletableFuture, net.minecraft.client.GameLoadCookie);
    private void lambda$reloadResourcePacks$1(boolean, net.minecraft.client.GameLoadCookie, java.lang.Throwable);
    private void lambda$clearResourcePacksOnError$0(net.minecraft.network.chat.Component);
    private void lambda$new$7(net.minecraft.client.telemetry.TelemetryPropertyMap$Builder);
    private int lambda$new$6();
    private void lambda$new$3(net.minecraft.client.GameLoadCookie, java.util.Optional);
    private void lambda$new$5(net.minecraft.client.GameLoadCookie);
    private void lambda$new$4(net.minecraft.client.GameLoadCookie, java.lang.Throwable);
    private void lambda$new$2(net.minecraft.client.resources.language.ClientLanguage);
    private com.mojang.authlib.minecraft.UserApiService$UserProperties lambda$new$1();
    private com.mojang.authlib.services.ProfileResult lambda$new$0();
    static {};
}
```
