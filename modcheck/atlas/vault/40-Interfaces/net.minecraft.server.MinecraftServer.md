---
type: "interface"
fqcn: "net.minecraft.server.MinecraftServer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.MinecraftServer

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`abstract_class` public abstract; extends `net/minecraft/util/thread/ReentrantBlockableEventLoop`; implements `net/minecraft/commands/CommandSource`, `net/minecraft/server/ServerInfo`, `net/minecraft/world/level/chunk/storage/ChunkIOErrorReporter`, `net/fabricmc/fabric/api/attachment/v1/GlobalAttachmentsProvider`, `net/fabricmc/fabric/api/resource/v1/DataResourceStore`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createCommandSourceStack` | `()Lnet/minecraft/commands/CommandSourceStack;` | exact | invokevirtual@5 in `TestServerContextImpl.lambda$runCommand$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `execute` | `(Ljava/lang/Runnable;)V` | inherited_exact | invokevirtual@5 in `ServerConfigurationNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `execute` | `(Ljava/lang/Runnable;)V` | inherited_exact | invokevirtual@14 in `ServerPlayNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getCommands` | `()Lnet/minecraft/commands/Commands;` | exact | invokevirtual@1 in `TestServerContextImpl.lambda$runCommand$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getCompressionThreshold` | `()I` | exact | invokevirtual@4 in `ServerLoginNetworkAddon.sendCompressionPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getCompressionThreshold` | `()I` | exact | invokevirtual@32 in `ServerLoginNetworkAddon.sendCompressionPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getCompressionThreshold` | `()I` | exact | invokevirtual@8 in `ServerLoginNetworkAddon.lambda$sendCompressionPacket$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getConnection` | `()Lnet/minecraft/server/network/ServerConnectionListener;` | exact | invokevirtual@11 in `GlobalAttachmentsImpl.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getLevel` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/server/level/Ser` | exact | invokevirtual@38 in `TestServerConnectionImpl.getServerLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getPlayerList` | `()Lnet/minecraft/server/players/PlayerList;` | exact | invokevirtual@29 in `TestServerConnectionImpl.getServerPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getPlayerList` | `()Lnet/minecraft/server/players/PlayerList;` | exact | invokevirtual@8 in `PlayerLookup.all` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPlayerList` | `()Lnet/minecraft/server/players/PlayerList;` | exact | invokevirtual@15 in `PlayerLookup.all` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPort` | `()I` | exact | invokevirtual@4 in `TestDedicatedServerContextImpl.getConnectionAddress` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getProfilePermissions` | `(Lnet/minecraft/server/players/NameAndId;)Lnet/minecraft/server/permis` | exact | invokevirtual@25 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getProfilePermissions` | `(Lnet/minecraft/server/players/NameAndId;)Lnet/minecraft/server/permis` | exact | invokevirtual@16 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `getRunningThread` | `()Ljava/lang/Thread;` | exact | invokevirtual@15 in `TestDedicatedServerContextImpl.close` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getRunningThread` | `()Ljava/lang/Thread;` | exact | invokevirtual@10 in `TestDedicatedServerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWorldPath` | `(Lnet/minecraft/world/level/storage/LevelResource;)Ljava/nio/file/Path` | exact | invokevirtual@4 in `RegistryCustomContentState.getPath` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `globalAttachments` | `()Lnet/fabricmc/fabric/api/attachment/v1/GlobalAttachments;` | inherited_exact | invokevirtual@1 in `AttachmentSavedData.codec` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `globalAttachments` | `()Lnet/fabricmc/fabric/api/attachment/v1/GlobalAttachments;` | inherited_exact | invokevirtual@4 in `ServerLevelMixin.globalAttachments` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `halt` | `(Z)V` | exact | invokevirtual@39 in `TestDedicatedServerContextImpl.close` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isDedicatedServer` | `()Z` | exact | invokevirtual@12 in `ServerHandshakePacketListenerImplMixin.rejectConnectionsDuringStartup | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@20 in `TestServerContextImpl.runOnServer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@20 in `TestServerContextImpl.computeOnServer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@1 in `ThreadingImpl.checkOnServerThread` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSameThread` | `()Z` | inherited_exact | invokevirtual@10 in `ThreadingImpl.checkOnGametestOrServerThread` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `isSingleplayerOwner` | `(Lnet/minecraft/server/players/NameAndId;)Z` | exact | invokevirtual@18 in `RegistrySyncManager.configureClient` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `notificationManager` | `()Lnet/minecraft/server/notifications/NotificationManager;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| calls | `overworld` | `()Lnet/minecraft/server/level/ServerLevel;` | exact | invokevirtual@30 in `EntityApiLookupImpl.lambda$checkSelfImplementingTypes$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `packetProcessor` | `()Lnet/minecraft/network/PacketProcessor;` | exact | invokevirtual@4 in `ServerPlayNetworkAddon.isOnReceiveThread` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `packetProcessor` | `()Lnet/minecraft/network/PacketProcessor;` | exact | invokevirtual@65 in `ServerCommonPacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `packetProcessor` | `()Lnet/minecraft/network/PacketProcessor;` | exact | invokevirtual@26 in `ServerGamePacketListenerImplMixin.handleCustomPayloadReceivedAsync` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | declared |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@5 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWarnin | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@11 in `GlobalAttachmentsImpl.fabric_getRegistryAccess` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@1 in `DimensionModificationImpl.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess$Frozen;` | exact | invokevirtual@21 in `ServerLoginPacketListenerImplMixin.initAddon` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `reloadableRegistries` | `()Lnet/minecraft/server/ReloadableServerRegistries$Holder;` | exact | invokevirtual@4 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `schedule` | `(Ljava/lang/Runnable;)V` | inherited_exact | invokevirtual@89 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `spin` | `(Ljava/util/function/Function;)Lnet/minecraft/server/MinecraftServer;` | exact | invokestatic@82 in `FabricGameTestRunner.runHeadlessServer` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `wrapRunnable` | `(Ljava/lang/Runnable;)Lnet/minecraft/server/TickTask;` | exact | invokevirtual@86 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| injects_into | `<init>` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `getChatDecorator` | `()Lnet/minecraft/network/chat/ChatDecorator;` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `onGameRuleChanged` | `(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| injects_into | `reloadResources` | `(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `reloadResources` | `(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `runServer` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `runServer` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `runServer` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `runServer` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `saveAllChunks` | `(ZZZ)Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `saveAllChunks` | `(ZZZ)Z` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `stopServer` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `stopServer` | `()V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `stopServer` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tickServer` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tickServer` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `resources` | `Lnet/minecraft/server/MinecraftServer$ReloadableResources;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |
| reads | `resources` | `Lnet/minecraft/server/MinecraftServer$ReloadableResources;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| reads | `savedDataStorage` | `Lnet/minecraft/world/level/storage/SavedDataStorage;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |
| wraps | `configurePackRepository` | `(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `createLevels` | `()V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| wraps | `runServer` | `()V` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (98 fields, 269 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final VANILLA_BRAND : Ljava/lang/String;
private static final AVERAGE_TICK_TIME_SMOOTHING : F
private static final TICK_STATS_SPAN : I
private static final OVERLOADED_THRESHOLD_NANOS : J
private static final OVERLOADED_TICKS_THRESHOLD : I
private static final OVERLOADED_WARNING_INTERVAL_NANOS : J
private static final OVERLOADED_TICKS_WARNING_INTERVAL : I
private static final STATUS_EXPIRE_TIME_NANOS : J
private static final PREPARE_LEVELS_DEFAULT_DELAY_NANOS : J
private static final MAX_STATUS_PLAYER_SAMPLE : I
public static final SPAWN_POSITION_SEARCH_RADIUS : I
private static final SERVER_ACTIVITY_MONITOR_SECONDS_BETWEEN_NOTIFICATIONS : I
private static final AUTOSAVE_INTERVAL : I
private static final MIMINUM_AUTOSAVE_TICKS : I
private static final MAX_TICK_LATENCY : I
public static final ABSOLUTE_MAX_WORLD_SIZE : I
public static final DEMO_SETTINGS : Lnet/minecraft/world/level/LevelSettings;
public static final DEFAULT_GAME_RULES : Ljava/util/function/Supplier;
public static final ANONYMOUS_PLAYER_PROFILE : Lnet/minecraft/server/players/NameAndId;
public static final SERVER_THREAD_NAME : Ljava/lang/String;
protected final storageSource : Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;
protected final playerDataStorage : Lnet/minecraft/world/level/storage/PlayerDataStorage;
private final savedDataStorage : Lnet/minecraft/world/level/storage/SavedDataStorage;
private final tickables : Ljava/util/List;
private final gameRules : Lnet/minecraft/world/level/gamerules/GameRules;
private metricsRecorder : Lnet/minecraft/util/profiling/metrics/profiling/MetricsRecorder;
private onMetricsRecordingStopped : Ljava/util/function/Consumer;
private onMetricsRecordingFinished : Ljava/util/function/Consumer;
private willStartRecordingMetrics : Z
private debugCommandProfiler : Lnet/minecraft/server/MinecraftServer$TimeProfiler;
private debugCommandProfilerDelayStart : Z
private final connection : Lnet/minecraft/server/network/ServerConnectionListener;
private final levelLoadListener : Lnet/minecraft/server/level/progress/LevelLoadListener;
private status : Lnet/minecraft/network/protocol/status/ServerStatus;
private statusIcon : Lnet/minecraft/network/protocol/status/ServerStatus$Favicon;
private final random : Lnet/minecraft/util/RandomSource;
private final fixerUpper : Lcom/mojang/datafixers/DataFixer;
private localIp : Ljava/lang/String;
private port : I
private final registries : Lnet/minecraft/core/LayeredRegistryAccess;
private final levels : Ljava/util/Map;
private playerList : Lnet/minecraft/server/players/PlayerList;
private running : Z
private stopped : Z
private tickCount : I
private ticksUntilAutosave : I
protected final proxy : Ljava/net/Proxy;
private onlineMode : Z
private preventProxyConnections : Z
private motd : Ljava/lang/String;
private playerIdleTimeout : I
private final tickTimesNanos : [J
private aggregatedTickTimesNanos : J
private keyPair : Ljava/security/KeyPair;
private singleplayerProfile : Lcom/mojang/authlib/GameProfile;
private isDemo : Z
private isReady : Z
private lastOverloadWarningNanos : J
protected final services : Lnet/minecraft/server/Services;
private final notificationManager : Lnet/minecraft/server/notifications/NotificationManager;
private final serverActivityMonitor : Lnet/minecraft/server/notifications/ServerActivityMonitor;
private lastServerStatus : J
private final serverThread : Ljava/lang/Thread;
private lastTickNanos : J
private taskExecutionStartNanos : J
private idleTimeNanos : J
private nextTickTimeNanos : J
private waitingForNextTick : Z
private delayedTasksMaxNextTickTimeNanos : J
private mayHaveDelayedTasks : Z
private final packRepository : Lnet/minecraft/server/packs/repository/PackRepository;
private final worldGenSettings : Lnet/minecraft/world/level/levelgen/WorldGenSettings;
private final scoreboard : Lnet/minecraft/server/ServerScoreboard;
private stopwatches : Lnet/minecraft/world/Stopwatches;
private commandStorage : Lnet/minecraft/world/level/storage/CommandStorage;
private final customBossEvents : Lnet/minecraft/server/bossevents/CustomBossEvents;
private final randomSequences : Lnet/minecraft/world/RandomSequences;
private final weatherData : Lnet/minecraft/world/level/saveddata/WeatherData;
private final functionManager : Lnet/minecraft/server/ServerFunctionManager;
private enforceWhitelist : Z
private usingWhitelist : Z
private smoothedTickTimeMillis : F
private final executor : Ljava/util/concurrent/Executor;
private serverId : Ljava/lang/String;
private resources : Lnet/minecraft/server/MinecraftServer$ReloadableResources;
private final structureTemplateManager : Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;
private final tickRateManager : Lnet/minecraft/server/ServerTickRateManager;
private final debugSubscribers : Lnet/minecraft/util/debug/ServerDebugSubscribers;
protected final worldData : Lnet/minecraft/world/level/storage/WorldData;
private effectiveRespawnData : Lnet/minecraft/world/level/storage/LevelData$RespawnData;
private emptyTicks : I
private isSaving : Z
private final suppressedExceptions : Lnet/minecraft/server/SuppressedExceptionCollector;
private final tickFrame : Lcom/mojang/jtracy/DiscontinuousFrame;
private final packetProcessor : Lnet/minecraft/network/PacketProcessor;
private final scheduledEvents : Lnet/minecraft/world/level/timers/TimerQueue;
private final clockManager : Lnet/minecraft/world/clock/ServerClockManager;
public static spin(Ljava/util/function/Function;)Lnet/minecraft/server/MinecraftServer;
public <init>(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Ljava/net/Proxy;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/level/progress/LevelLoadListener;ZLnet/minecraft/server/notifications/NotificationManager;)V
protected abstract initServer()Z
public createChunkLoadStatusView(I)Lnet/minecraft/server/level/progress/ChunkLoadStatusView;
protected loadLevel()V
protected forceDifficulty()V
protected createLevels()V
private static setInitialSpawn(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/level/storage/ServerLevelData;ZZLnet/minecraft/server/level/progress/LevelLoadListener;)V
private setupDebugLevel(Lnet/minecraft/world/level/storage/WorldData;)V
private prepareLevels()V
protected selectLevelLoadFocusPos()Lnet/minecraft/core/GlobalPos;
public getDefaultGameType()Lnet/minecraft/world/level/GameType;
public isHardcore()Z
public abstract operatorUserPermissions()Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public abstract getFunctionCompilationPermissions()Lnet/minecraft/server/permissions/PermissionSet;
public abstract shouldRconBroadcast()Z
public saveAllChunks(ZZZ)Z
public saveEverything(ZZZ)Z
public close()V
protected stopServer()V
public getLocalIp()Ljava/lang/String;
public setLocalIp(Ljava/lang/String;)V
public isRunning()Z
public halt(Z)V
protected runServer()V
private logFullTickTime()V
private startMeasuringTaskExecutionTime()V
private finishMeasuringTaskExecutionTime()V
private static constructOrExtractCrashReport(Ljava/lang/Throwable;)Lnet/minecraft/CrashReport;
private haveTime()Z
public notificationManager()Lnet/minecraft/server/notifications/NotificationManager;
protected waitUntilNextTick()V
protected waitForTasks()V
public wrapRunnable(Ljava/lang/Runnable;)Lnet/minecraft/server/TickTask;
protected shouldRun(Lnet/minecraft/server/TickTask;)Z
protected pollTask()Z
private pollTaskInternal()Z
protected doRunTask(Lnet/minecraft/server/TickTask;)V
private loadStatusIcon()Ljava/util/Optional;
public getWorldScreenshotFile()Ljava/util/Optional;
public getServerDirectory()Ljava/nio/file/Path;
public getServerActivityMonitor()Lnet/minecraft/server/notifications/ServerActivityMonitor;
protected onServerCrash(Lnet/minecraft/CrashReport;)V
protected onServerExit()V
public isPaused()Z
protected tickServer(Ljava/util/function/BooleanSupplier;)V
protected processPacketsAndTick(Z)V
private autoSave()V
private logTickMethodTime(J)V
private computeNextAutosaveInterval()I
public onTickRateChanged()V
protected abstract getTickTimeLogger()Lnet/minecraft/util/debugchart/SampleLogger;
public abstract isTickTimeLoggingEnabled()Z
private buildServerStatus()Lnet/minecraft/network/protocol/status/ServerStatus;
private buildPlayerStatus()Lnet/minecraft/network/protocol/status/ServerStatus$Players;
protected tickChildren(Ljava/util/function/BooleanSupplier;)V
private updateEffectiveRespawnData()V
protected tickConnection()V
public forceGameTimeSynchronization()V
public addTickable(Ljava/lang/Runnable;)V
protected setId(Ljava/lang/String;)V
public isShutdown()Z
public getFile(Ljava/lang/String;)Ljava/nio/file/Path;
public final overworld()Lnet/minecraft/server/level/ServerLevel;
public getLevel(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/server/level/ServerLevel;
public levelKeys()Ljava/util/Set;
public getAllLevels()Ljava/lang/Iterable;
public getServerVersion()Ljava/lang/String;
public getPlayerCount()I
public getPlayerNames()[Ljava/lang/String;
public getServerModName()Ljava/lang/String;
public clockManager()Lnet/minecraft/world/clock/ServerClockManager;
public fillSystemReport(Lnet/minecraft/SystemReport;)Lnet/minecraft/SystemReport;
public abstract fillServerSystemReport(Lnet/minecraft/SystemReport;)Lnet/minecraft/SystemReport;
public getModdedStatus()Lnet/minecraft/util/ModCheck;
public sendSystemMessage(Lnet/minecraft/network/chat/Component;)V
public getKeyPair()Ljava/security/KeyPair;
public getPort()I
public setPort(I)V
public getSingleplayerProfile()Lcom/mojang/authlib/GameProfile;
public setSingleplayerProfile(Lcom/mojang/authlib/GameProfile;)V
public isSingleplayer()Z
protected initializeKeyPair()V
public setDifficulty(Lnet/minecraft/world/Difficulty;Z)V
public getScaledTrackingDistance(I)I
public updateMobSpawningFlags()V
public setDifficultyLocked(Z)V
private sendDifficultyUpdate(Lnet/minecraft/server/level/ServerPlayer;)V
public isDemo()Z
public setDemo(Z)V
public getCodeOfConducts()Ljava/util/Map;
public getServerResourcePack()Ljava/util/Optional;
public isResourcePackRequired()Z
public abstract isDedicatedServer()Z
public abstract getRateLimitPacketsPerSecond()I
public abstract getCommandSpamThresholdSeconds()I
public abstract getChatSpamThresholdSeconds()I
public usesAuthentication()Z
public setUsesAuthentication(Z)V
public getPreventProxyConnections()Z
public setPreventProxyConnections(Z)V
public abstract useNativeTransport()Z
public allowFlight()Z
public getMotd()Ljava/lang/String;
public setMotd(Ljava/lang/String;)V
public isStopped()Z
public getPlayerList()Lnet/minecraft/server/players/PlayerList;
public setPlayerList(Lnet/minecraft/server/players/PlayerList;)V
public abstract isPublished()Z
public setDefaultGameType(Lnet/minecraft/world/level/GameType;)V
public enforceGameTypeForPlayers(Lnet/minecraft/world/level/GameType;)I
public getConnection()Lnet/minecraft/server/network/ServerConnectionListener;
public isReady()Z
public publishServer(Lnet/minecraft/server/MinecraftServer$MultiplayerScope;ZI)Z
public unpublishServer()Z
public getTickCount()I
public isUnderSpawnProtection(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)Z
public repliesToStatus()Z
public hidesOnlinePlayers()Z
public getProxy()Ljava/net/Proxy;
public playerIdleTimeout()I
public setPlayerIdleTimeout(I)V
public services()Lnet/minecraft/server/Services;
public getStatus()Lnet/minecraft/network/protocol/status/ServerStatus;
public invalidateStatus()V
public getAbsoluteMaxWorldSize()I
public scheduleExecutables()Z
public executeIfPossible(Ljava/lang/Runnable;)V
public getRunningThread()Ljava/lang/Thread;
public getCompressionThreshold()I
public enforceSecureProfile()Z
public getNextTickTime()J
public getFixerUpper()Lcom/mojang/datafixers/DataFixer;
public getAdvancements()Lnet/minecraft/server/ServerAdvancementManager;
public getFunctions()Lnet/minecraft/server/ServerFunctionManager;
public reloadResources(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;
public static configurePackRepository(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;ZZ)Lnet/minecraft/world/level/WorldDataConfiguration;
private static configureRepositoryWithSelection(Lnet/minecraft/server/packs/repository/PackRepository;Ljava/util/Collection;Lnet/minecraft/world/flag/FeatureFlagSet;Z)Lnet/minecraft/world/level/WorldDataConfiguration;
private static enableForcedFeaturePacks(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/flag/FeatureFlagSet;)V
private static getSelectedPacks(Lnet/minecraft/server/packs/repository/PackRepository;Z)Lnet/minecraft/world/level/DataPackConfig;
public kickUnlistedPlayers()V
public getPackRepository()Lnet/minecraft/server/packs/repository/PackRepository;
public getCommands()Lnet/minecraft/commands/Commands;
public createCommandSourceStack()Lnet/minecraft/commands/CommandSourceStack;
public findRespawnDimension()Lnet/minecraft/server/level/ServerLevel;
public setRespawnData(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)V
public getRespawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public acceptsSuccess()Z
public acceptsFailure()Z
public abstract shouldInformAdmins()Z
public getWorldGenSettings()Lnet/minecraft/world/level/levelgen/WorldGenSettings;
public getRecipeManager()Lnet/minecraft/world/item/crafting/RecipeManager;
public getScoreboard()Lnet/minecraft/server/ServerScoreboard;
public getCommandStorage()Lnet/minecraft/world/level/storage/CommandStorage;
public getStopwatches()Lnet/minecraft/world/Stopwatches;
public getCustomBossEvents()Lnet/minecraft/server/bossevents/CustomBossEvents;
public getRandomSequence(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/util/RandomSource;
public getRandomSequences()Lnet/minecraft/world/RandomSequences;
public setWeatherParameters(IIZZ)V
public getWeatherData()Lnet/minecraft/world/level/saveddata/WeatherData;
public isEnforceWhitelist()Z
public setEnforceWhitelist(Z)V
public isUsingWhitelist()Z
public setUsingWhitelist(Z)V
public getCurrentSmoothedTickTime()F
public tickRateManager()Lnet/minecraft/server/ServerTickRateManager;
public getAverageTickTimeNanos()J
public getTickTimesNanos()[J
public getProfilePermissions(Lnet/minecraft/server/players/NameAndId;)Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public abstract isSingleplayerOwner(Lnet/minecraft/server/players/NameAndId;)Z
public dumpServerProperties(Ljava/nio/file/Path;)V
private saveDebugReport(Ljava/nio/file/Path;)V
private dumpMiscStats(Ljava/nio/file/Path;)V
private dumpGameRules(Ljava/nio/file/Path;)V
private dumpClasspath(Ljava/nio/file/Path;)V
private dumpThreads(Ljava/nio/file/Path;)V
private dumpNativeModules(Ljava/nio/file/Path;)V
private createProfiler()Lnet/minecraft/util/profiling/ProfilerFiller;
protected endMetricsRecordingTick()V
public isRecordingMetrics()Z
public startRecordingMetrics(Ljava/util/function/Consumer;Ljava/util/function/Consumer;)V
public stopRecordingMetrics()V
public finishRecordingMetrics()V
public cancelRecordingMetrics()V
public getWorldPath(Lnet/minecraft/world/level/storage/LevelResource;)Ljava/nio/file/Path;
public forceSynchronousWrites()Z
public getStructureTemplateManager()Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;
public getWorldData()Lnet/minecraft/world/level/storage/WorldData;
public registryAccess()Lnet/minecraft/core/RegistryAccess$Frozen;
public registries()Lnet/minecraft/core/LayeredRegistryAccess;
public reloadableRegistries()Lnet/minecraft/server/ReloadableServerRegistries$Holder;
public createTextFilterForPlayer(Lnet/minecraft/server/level/ServerPlayer;)Lnet/minecraft/server/network/TextFilter;
public createGameModeForPlayer(Lnet/minecraft/server/level/ServerPlayer;)Lnet/minecraft/server/level/ServerPlayerGameMode;
public getForcedGameType()Lnet/minecraft/world/level/GameType;
public forceGameMode()Z
public setForceGameMode(Z)V
public getResourceManager()Lnet/minecraft/server/packs/resources/ResourceManager;
public isCurrentlySaving()Z
public isTimeProfilerRunning()Z
public startTimeProfiler()V
public stopTimeProfiler()Lnet/minecraft/util/profiling/ProfileResults;
public getMaxChainedNeighborUpdates()I
public logChatMessage(Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/ChatType$Bound;Ljava/lang/String;)V
public getChatDecorator()Lnet/minecraft/network/chat/ChatDecorator;
public logIPs()Z
public handleCustomClickAction(Lnet/minecraft/resources/Identifier;Ljava/util/Optional;)V
public getLevelLoadListener()Lnet/minecraft/server/level/progress/LevelLoadListener;
public setAutoSave(Z)Z
public isAutoSave()Z
public onGameRuleChanged(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
public getGlobalGameRules()Lnet/minecraft/world/level/gamerules/GameRules;
public getDataStorage()Lnet/minecraft/world/level/storage/SavedDataStorage;
public getScheduledEvents()Lnet/minecraft/world/level/timers/TimerQueue;
public getGameRules()Lnet/minecraft/world/level/gamerules/GameRules;
public acceptsTransfers()Z
private storeChunkIoError(Lnet/minecraft/CrashReport;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;)V
public reportChunkLoadFailure(Ljava/lang/Throwable;Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;Lnet/minecraft/world/level/ChunkPos;)V
public reportChunkSaveFailure(Ljava/lang/Throwable;Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;Lnet/minecraft/world/level/ChunkPos;)V
protected warnOnLowDiskSpace()V
public sendLowDiskSpaceWarning()V
public reportPacketHandlingException(Ljava/lang/Throwable;Lnet/minecraft/network/protocol/PacketType;)V
public serverLinks()Lnet/minecraft/server/ServerLinks;
protected pauseWhenEmptySeconds()I
public packetProcessor()Lnet/minecraft/network/PacketProcessor;
public debugSubscribers()Lnet/minecraft/util/debug/ServerDebugSubscribers;
protected synthetic doRunTask(Ljava/lang/Runnable;)V
protected synthetic shouldRun(Ljava/lang/Runnable;)Z
public synthetic wrapRunnable(Ljava/lang/Runnable;)Ljava/lang/Runnable;
private synthetic lambda$storeChunkIoError$0(Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;Lnet/minecraft/CrashReport;Lnet/minecraft/world/level/ChunkPos;)V
private static synthetic lambda$storeChunkIoError$1(Lnet/minecraft/world/level/chunk/storage/RegionStorageInfo;)Ljava/lang/String;
private static synthetic lambda$onGameRuleChanged$1(Ljava/lang/Object;Lnet/minecraft/server/level/ServerLevel;)V
private static synthetic lambda$onGameRuleChanged$0(Lnet/minecraft/network/protocol/game/ClientboundGameEventPacket;Lnet/minecraft/server/level/ServerPlayer;)V
private synthetic lambda$startRecordingMetrics$0(Ljava/util/function/Consumer;Lnet/minecraft/util/profiling/ProfileResults;)V
private synthetic lambda$createProfiler$0(Ljava/nio/file/Path;)V
private synthetic lambda$createProfiler$1(Ljava/nio/file/Path;)V
private static synthetic lambda$getSelectedPacks$0(Ljava/util/Collection;Ljava/lang/String;)Z
private synthetic lambda$reloadResources$4(Ljava/util/Collection;Lnet/minecraft/server/MinecraftServer$ReloadableResources;)V
private synthetic lambda$reloadResources$1(Lcom/google/common/collect/ImmutableList;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$reloadResources$3(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;)Lnet/minecraft/server/MinecraftServer$ReloadableResources;
private static synthetic lambda$reloadResources$2(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Ljava/lang/Throwable;)V
private synthetic lambda$reloadResources$0(Ljava/util/Collection;)Lcom/google/common/collect/ImmutableList;
private synthetic lambda$fillSystemReport$7()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$6()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$5()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$4()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$3()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$2()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$1()Ljava/lang/Object;
private synthetic lambda$fillSystemReport$0()Ljava/lang/Object;
private static synthetic lambda$tickChildren$1(Lnet/minecraft/server/level/ServerLevel;)Ljava/lang/String;
private static synthetic lambda$tickChildren$0(Lnet/minecraft/server/level/ServerPlayer;)V
private static synthetic lambda$processPacketsAndTick$0()Z
private static synthetic lambda$loadStatusIcon$3(Ljava/nio/file/Path;)Ljava/util/Optional;
private synthetic lambda$loadStatusIcon$1()Ljava/util/Optional;
private static synthetic lambda$loadStatusIcon$2(Ljava/nio/file/Path;)Z
private static synthetic lambda$loadStatusIcon$0(Ljava/nio/file/Path;)Z
private synthetic lambda$waitUntilNextTick$0()Z
private static synthetic lambda$stopServer$1()Z
private static synthetic lambda$stopServer$0(Lnet/minecraft/server/level/ServerLevel;)Z
private static synthetic lambda$prepareLevels$0(Lnet/minecraft/server/level/ServerLevel;)V
private static synthetic lambda$setInitialSpawn$1(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerChunkCache;Lnet/minecraft/world/level/storage/ServerLevelData;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$setInitialSpawn$0(Lnet/minecraft/core/Registry;)Ljava/util/Optional;
private synthetic lambda$new$2(Lnet/minecraft/world/level/gamerules/GameRules;)V
private static synthetic lambda$spin$1(Ljava/lang/Thread;Ljava/lang/Throwable;)V
private static synthetic lambda$spin$0(Ljava/util/concurrent/atomic/AtomicReference;)V
private static synthetic lambda$new$1(Ljava/nio/file/Path;)V
private synthetic lambda$new$0(Lnet/minecraft/util/profiling/ProfileResults;)V
private static synthetic lambda$static$0()Lnet/minecraft/world/level/gamerules/GameRules;
static <clinit>()V
```
