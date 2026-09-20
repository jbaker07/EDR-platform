---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedServer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedServer

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

`class` public; extends `net/minecraft/server/MinecraftServer`; implements `net/minecraft/server/ServerInterface`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPlayerList` | `()Lnet/minecraft/server/dedicated/DedicatedPlayerList;` | exact | invokevirtual@1 in `DedicatedServerImplUtil.lambda$whitelistClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `initServer` | `()Z` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| wraps | `initServer` | `()Z` | name_only | @Redirect at ['INVOKE'] | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| wraps | `stopServer` | `()V` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (15 fields, 118 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final CONVERSION_RETRY_DELAY_MS : I
private static final CONVERSION_RETRIES : I
private final consoleInput : Ljava/util/List;
private queryThreadGs4 : Lnet/minecraft/server/rcon/thread/QueryThreadGs4;
private final rconConsoleSource : Lnet/minecraft/server/rcon/RconConsoleSource;
private rconThread : Lnet/minecraft/server/rcon/thread/RconThread;
private final settings : Lnet/minecraft/server/dedicated/DedicatedServerSettings;
private gui : Lnet/minecraft/server/gui/MinecraftServerGui;
private final serverTextFilter : Lnet/minecraft/server/network/ServerTextFilter;
private tickTimeLogger : Lnet/minecraft/util/debugchart/RemoteSampleLogger;
private isTickTimeLoggingEnabled : Z
private final serverLinks : Lnet/minecraft/server/ServerLinks;
private final codeOfConductTexts : Ljava/util/Map;
private final jsonRpcServer : Lnet/minecraft/server/jsonrpc/ManagementServer;
public <init>(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;Lnet/minecraft/server/dedicated/DedicatedServerSettings;Lcom/mojang/datafixers/DataFixer;Lnet/minecraft/server/Services;Lnet/minecraft/server/jsonrpc/ManagementServer;Lnet/minecraft/server/notifications/NotificationManager;)V
private static readCodeOfConducts()Ljava/util/Map;
protected initServer()Z
public isEnforceWhitelist()Z
public setEnforceWhitelist(Z)V
public isUsingWhitelist()Z
public setUsingWhitelist(Z)V
protected tickServer(Ljava/util/function/BooleanSupplier;)V
public saveAllChunks(ZZZ)Z
public sendLowDiskSpaceWarning()V
public allowFlight()Z
public setAllowFlight(Z)V
public getProperties()Lnet/minecraft/server/dedicated/DedicatedServerProperties;
public setDifficulty(Lnet/minecraft/world/Difficulty;)V
protected forceDifficulty()V
public viewDistance()I
public setViewDistance(I)V
public simulationDistance()I
public setSimulationDistance(I)V
public fillServerSystemReport(Lnet/minecraft/SystemReport;)Lnet/minecraft/SystemReport;
public dumpServerProperties(Ljava/nio/file/Path;)V
protected onServerExit()V
protected tickConnection()V
public handleConsoleInput(Ljava/lang/String;Lnet/minecraft/commands/CommandSourceStack;)V
public handleConsoleInputs()V
public isDedicatedServer()Z
public getRateLimitPacketsPerSecond()I
public getCommandSpamThresholdSeconds()I
public getChatSpamThresholdSeconds()I
public useNativeTransport()Z
public getPlayerList()Lnet/minecraft/server/dedicated/DedicatedPlayerList;
public getMaxPlayers()I
public setMaxPlayers(I)V
public isPublished()Z
public getServerIp()Ljava/lang/String;
public getServerPort()I
public getServerName()Ljava/lang/String;
public showGui()V
public spawnProtectionRadius()I
public setSpawnProtectionRadius(I)V
public isUnderSpawnProtection(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)Z
public repliesToStatus()Z
public setRepliesToStatus(Z)V
public hidesOnlinePlayers()Z
public setHidesOnlinePlayers(Z)V
public operatorUserPermissions()Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public setOperatorUserPermissions(Lnet/minecraft/server/permissions/LevelBasedPermissionSet;)V
public getFunctionCompilationPermissions()Lnet/minecraft/server/permissions/PermissionSet;
public playerIdleTimeout()I
public setPlayerIdleTimeout(I)V
public statusHeartbeatInterval()I
public setStatusHeartbeatInterval(I)Z
public getMotd()Ljava/lang/String;
public setMotd(Ljava/lang/String;)V
public shouldRconBroadcast()Z
public shouldInformAdmins()Z
public getAbsoluteMaxWorldSize()I
public getCompressionThreshold()I
public enforceSecureProfile()Z
public logIPs()Z
protected convertOldUsers()Z
private waitForRetry()V
public getMaxTickLength()J
public getMaxChainedNeighborUpdates()I
public getPluginNames()Ljava/lang/String;
public runCommand(Ljava/lang/String;)Ljava/lang/String;
protected stopServer()V
public isSingleplayerOwner(Lnet/minecraft/server/players/NameAndId;)Z
public getScaledTrackingDistance(I)I
public entityBroadcastRangePercentage()I
public setEntityBroadcastRangePercentage(I)V
public getLevelIdName()Ljava/lang/String;
public forceSynchronousWrites()Z
public createTextFilterForPlayer(Lnet/minecraft/server/level/ServerPlayer;)Lnet/minecraft/server/network/TextFilter;
public getForcedGameType()Lnet/minecraft/world/level/GameType;
public forceGameMode()Z
public setForceGameMode(Z)V
public gameMode()Lnet/minecraft/world/level/GameType;
public setGameMode(Lnet/minecraft/world/level/GameType;)V
public getServerResourcePack()Ljava/util/Optional;
protected endMetricsRecordingTick()V
protected getTickTimeLogger()Lnet/minecraft/util/debugchart/SampleLogger;
public isTickTimeLoggingEnabled()Z
public acceptsTransfers()Z
public setAcceptsTransfers(Z)V
public serverLinks()Lnet/minecraft/server/ServerLinks;
public pauseWhenEmptySeconds()I
public setPauseWhenEmptySeconds(I)V
private static createServerLinks(Lnet/minecraft/server/dedicated/DedicatedServerSettings;)Lnet/minecraft/server/ServerLinks;
private static parseBugReportLink(Lnet/minecraft/server/dedicated/DedicatedServerProperties;)Ljava/util/Optional;
public getCodeOfConducts()Ljava/util/Map;
public synthetic getPlayerList()Lnet/minecraft/server/players/PlayerList;
private static synthetic lambda$createServerLinks$0(Ljava/net/URI;)Lnet/minecraft/server/ServerLinks;
private synthetic lambda$setPauseWhenEmptySeconds$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setAcceptsTransfers$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setGameMode$0(Lnet/minecraft/world/level/GameType;Lnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setForceGameMode$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setEntityBroadcastRangePercentage$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$runCommand$0(Ljava/lang/String;)V
private synthetic lambda$setMotd$0(Ljava/lang/String;Lnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setStatusHeartbeatInterval$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setPlayerIdleTimeout$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setOperatorUserPermissions$0(Lnet/minecraft/server/permissions/LevelBasedPermissionSet;Lnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setHidesOnlinePlayers$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setRepliesToStatus$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setSpawnProtectionRadius$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setMaxPlayers$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private static synthetic lambda$fillServerSystemReport$1()Ljava/lang/Object;
private synthetic lambda$fillServerSystemReport$0()Ljava/lang/Object;
private synthetic lambda$setSimulationDistance$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setViewDistance$0(ILnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setDifficulty$0(Lnet/minecraft/world/Difficulty;Lnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setAllowFlight$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private static synthetic lambda$sendLowDiskSpaceWarning$1(Lnet/minecraft/server/level/ServerPlayer;)V
private static synthetic lambda$sendLowDiskSpaceWarning$0(Lnet/minecraft/server/permissions/Permission$HasCommandLevel;Lnet/minecraft/server/level/ServerPlayer;)Z
private synthetic lambda$setUsingWhitelist$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private synthetic lambda$setEnforceWhitelist$0(ZLnet/minecraft/server/dedicated/DedicatedServerProperties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
static <clinit>()V
```
