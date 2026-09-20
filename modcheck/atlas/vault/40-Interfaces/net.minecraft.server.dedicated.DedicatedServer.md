---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedServer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedServer

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getPlayerList()Lnet/minecraft/server/dedicated/DedicatedPlayerList;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `initServer` | `@Inject at INVOKE Lnet/minecraft/server/dedicated/DedicatedServer;loadLevel()V` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| wraps | `initServer` | `@Redirect at INVOKE Lnet/minecraft/server/notifications/NotificationManager;serv` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (133, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.dedicated.DedicatedServer extends net.minecraft.server.MinecraftServer implements net.minecraft.server.ServerInterface {
    private static final org.slf4j.Logger LOGGER;
    private static final int CONVERSION_RETRY_DELAY_MS;
    private static final int CONVERSION_RETRIES;
    private final java.util.List<net.minecraft.server.ConsoleInput> consoleInput;
    private net.minecraft.server.rcon.thread.QueryThreadGs4 queryThreadGs4;
    private final net.minecraft.server.rcon.RconConsoleSource rconConsoleSource;
    private net.minecraft.server.rcon.thread.RconThread rconThread;
    private final net.minecraft.server.dedicated.DedicatedServerSettings settings;
    private net.minecraft.server.gui.MinecraftServerGui gui;
    private final net.minecraft.server.network.ServerTextFilter serverTextFilter;
    private net.minecraft.util.debugchart.RemoteSampleLogger tickTimeLogger;
    private boolean isTickTimeLoggingEnabled;
    private final net.minecraft.server.ServerLinks serverLinks;
    private final java.util.Map<java.lang.String, java.lang.String> codeOfConductTexts;
    private final net.minecraft.server.jsonrpc.ManagementServer jsonRpcServer;
    public net.minecraft.server.dedicated.DedicatedServer(java.lang.Thread, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, net.minecraft.server.WorldStem, java.util.Optional<net.minecraft.world.level.gamerules.GameRules>, net.minecraft.server.dedicated.DedicatedServerSettings, com.mojang.datafixers.DataFixer, net.minecraft.server.Services, net.minecraft.server.jsonrpc.ManagementServer, net.minecraft.server.notifications.NotificationManager);
    private static java.util.Map<java.lang.String, java.lang.String> readCodeOfConducts();
    protected boolean initServer() throws java.io.IOException;
    public boolean isEnforceWhitelist();
    public void setEnforceWhitelist(boolean);
    public boolean isUsingWhitelist();
    public void setUsingWhitelist(boolean);
    protected void tickServer(java.util.function.BooleanSupplier);
    public boolean saveAllChunks(boolean, boolean, boolean);
    public void sendLowDiskSpaceWarning();
    public boolean allowFlight();
    public void setAllowFlight(boolean);
    public net.minecraft.server.dedicated.DedicatedServerProperties getProperties();
    public void setDifficulty(net.minecraft.world.Difficulty);
    protected void forceDifficulty();
    public int viewDistance();
    public void setViewDistance(int);
    public int simulationDistance();
    public void setSimulationDistance(int);
    public net.minecraft.SystemReport fillServerSystemReport(net.minecraft.SystemReport);
    public void dumpServerProperties(java.nio.file.Path) throws java.io.IOException;
    protected void onServerExit();
    protected void tickConnection();
    public void handleConsoleInput(java.lang.String, net.minecraft.commands.CommandSourceStack);
    public void handleConsoleInputs();
    public boolean isDedicatedServer();
    public int getRateLimitPacketsPerSecond();
    public int getCommandSpamThresholdSeconds();
    public int getChatSpamThresholdSeconds();
    public boolean useNativeTransport();
    public net.minecraft.server.dedicated.DedicatedPlayerList getPlayerList();
    public int getMaxPlayers();
    public void setMaxPlayers(int);
    public boolean isPublished();
    public java.lang.String getServerIp();
    public int getServerPort();
    public java.lang.String getServerName();
    public void showGui();
    public int spawnProtectionRadius();
    public void setSpawnProtectionRadius(int);
    public boolean isUnderSpawnProtection(net.minecraft.server.level.ServerLevel, net.minecraft.core.BlockPos, net.minecraft.world.entity.player.Player);
    public boolean repliesToStatus();
    public void setRepliesToStatus(boolean);
    public boolean hidesOnlinePlayers();
    public void setHidesOnlinePlayers(boolean);
    public net.minecraft.server.permissions.LevelBasedPermissionSet operatorUserPermissions();
    public void setOperatorUserPermissions(net.minecraft.server.permissions.LevelBasedPermissionSet);
    public net.minecraft.server.permissions.PermissionSet getFunctionCompilationPermissions();
    public int playerIdleTimeout();
    public void setPlayerIdleTimeout(int);
    public int statusHeartbeatInterval();
    public boolean setStatusHeartbeatInterval(int);
    public java.lang.String getMotd();
    public void setMotd(java.lang.String);
    public boolean shouldRconBroadcast();
    public boolean shouldInformAdmins();
    public int getAbsoluteMaxWorldSize();
    public int getCompressionThreshold();
    public boolean enforceSecureProfile();
    public boolean logIPs();
    protected boolean convertOldUsers();
    private void waitForRetry();
    public long getMaxTickLength();
    public int getMaxChainedNeighborUpdates();
    public java.lang.String getPluginNames();
    public java.lang.String runCommand(java.lang.String);
    protected void stopServer();
    public boolean isSingleplayerOwner(net.minecraft.server.players.NameAndId);
    public int getScaledTrackingDistance(int);
    public int entityBroadcastRangePercentage();
    public void setEntityBroadcastRangePercentage(int);
    public java.lang.String getLevelIdName();
    public boolean forceSynchronousWrites();
    public net.minecraft.server.network.TextFilter createTextFilterForPlayer(net.minecraft.server.level.ServerPlayer);
    public net.minecraft.world.level.GameType getForcedGameType();
    public boolean forceGameMode();
    public void setForceGameMode(boolean);
    public net.minecraft.world.level.GameType gameMode();
    public void setGameMode(net.minecraft.world.level.GameType);
    public java.util.Optional<net.minecraft.server.MinecraftServer$ServerResourcePackInfo> getServerResourcePack();
    protected void endMetricsRecordingTick();
    protected net.minecraft.util.debugchart.SampleLogger getTickTimeLogger();
    public boolean isTickTimeLoggingEnabled();
    public boolean acceptsTransfers();
    public void setAcceptsTransfers(boolean);
    public net.minecraft.server.ServerLinks serverLinks();
    public int pauseWhenEmptySeconds();
    public void setPauseWhenEmptySeconds(int);
    private static net.minecraft.server.ServerLinks createServerLinks(net.minecraft.server.dedicated.DedicatedServerSettings);
    private static java.util.Optional<java.net.URI> parseBugReportLink(net.minecraft.server.dedicated.DedicatedServerProperties);
    public java.util.Map<java.lang.String, java.lang.String> getCodeOfConducts();
    public net.minecraft.server.players.PlayerList getPlayerList();
    private static net.minecraft.server.ServerLinks lambda$createServerLinks$0(java.net.URI);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setPauseWhenEmptySeconds$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setAcceptsTransfers$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setGameMode$0(net.minecraft.world.level.GameType, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setForceGameMode$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setEntityBroadcastRangePercentage$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private void lambda$runCommand$0(java.lang.String);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setMotd$0(java.lang.String, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setStatusHeartbeatInterval$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setPlayerIdleTimeout$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setOperatorUserPermissions$0(net.minecraft.server.permissions.LevelBasedPermissionSet, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setHidesOnlinePlayers$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setRepliesToStatus$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setSpawnProtectionRadius$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setMaxPlayers$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private static java.lang.Object lambda$fillServerSystemReport$1() throws java.lang.Exception;
    private java.lang.Object lambda$fillServerSystemReport$0() throws java.lang.Exception;
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setSimulationDistance$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setViewDistance$0(int, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setDifficulty$0(net.minecraft.world.Difficulty, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setAllowFlight$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private static void lambda$sendLowDiskSpaceWarning$1(net.minecraft.server.level.ServerPlayer);
    private static boolean lambda$sendLowDiskSpaceWarning$0(net.minecraft.server.permissions.Permission$HasCommandLevel, net.minecraft.server.level.ServerPlayer);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setUsingWhitelist$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    private net.minecraft.server.dedicated.DedicatedServerProperties lambda$setEnforceWhitelist$0(boolean, net.minecraft.server.dedicated.DedicatedServerProperties);
    static {};
}
```
