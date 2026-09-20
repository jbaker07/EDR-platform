---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedServerProperties"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedServerProperties

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `<init>` | `@Redirect at FIELD Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lne` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (85, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.dedicated.DedicatedServerProperties extends net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties> {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.regex.Pattern SHA1;
    private static final com.google.common.base.Splitter COMMA_SPLITTER;
    public static final java.lang.String MANAGEMENT_SERVER_TLS_ENABLED_KEY;
    public static final java.lang.String MANAGEMENT_SERVER_TLS_KEYSTORE_KEY;
    public static final java.lang.String MANAGEMENT_SERVER_TLS_KEYSTORE_PASSWORD_KEY;
    public final boolean onlineMode;
    public final boolean preventProxyConnections;
    public final java.lang.String serverIp;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> allowFlight;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.String> motd;
    public final boolean codeOfConduct;
    public final java.lang.String bugReportLink;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> forceGameMode;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> enforceWhitelist;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<net.minecraft.world.Difficulty> difficulty;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<net.minecraft.world.level.GameType> gameMode;
    public final java.lang.String levelName;
    public final int serverPort;
    public final boolean managementServerEnabled;
    public final java.lang.String managementServerHost;
    public final int managementServerPort;
    public final java.lang.String managementServerSecret;
    public final boolean managementServerTlsEnabled;
    public final java.lang.String managementServerTlsKeystore;
    public final java.lang.String managementServerTlsKeystorePassword;
    public final java.lang.String managementServerAllowedOrigins;
    public final java.lang.Boolean announcePlayerAchievements;
    public final boolean enableQuery;
    public final int queryPort;
    public final boolean enableRcon;
    public final int rconPort;
    public final java.lang.String rconPassword;
    public final boolean hardcore;
    public final boolean useNativeTransport;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> spawnProtection;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<net.minecraft.server.permissions.LevelBasedPermissionSet> opPermissions;
    public final net.minecraft.server.permissions.LevelBasedPermissionSet functionPermissions;
    public final long maxTickTime;
    public final int maxChainedNeighborUpdates;
    public final int rateLimitPacketsPerSecond;
    public final int commandSpamThresholdSeconds;
    public final int chatSpamThresholdSeconds;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> viewDistance;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> simulationDistance;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> maxPlayers;
    public final int networkCompressionThreshold;
    public final boolean broadcastRconToOps;
    public final boolean broadcastConsoleToOps;
    public final int maxWorldSize;
    public final boolean syncChunkWrites;
    public final java.lang.String regionFileComression;
    public final boolean enableJmxMonitoring;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> enableStatus;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> hideOnlinePlayers;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> entityBroadcastRangePercentage;
    public final java.lang.String textFilteringConfig;
    public final int textFilteringVersion;
    public final java.util.Optional<net.minecraft.server.MinecraftServer$ServerResourcePackInfo> serverResourcePackInfo;
    public final net.minecraft.world.level.DataPackConfig initialDataPackConfiguration;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> playerIdleTimeout;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> statusHeartbeatInterval;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> whiteList;
    public final boolean enforceSecureProfile;
    public final boolean logIPs;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Integer> pauseWhenEmptySeconds;
    private final net.minecraft.server.dedicated.DedicatedServerProperties$WorldDimensionData worldDimensionData;
    public final net.minecraft.world.level.levelgen.WorldOptions worldOptions;
    public final net.minecraft.server.dedicated.Settings<net.minecraft.server.dedicated.DedicatedServerProperties>.MutableValue<java.lang.Boolean> acceptsTransfers;
    public net.minecraft.server.dedicated.DedicatedServerProperties(java.util.Properties);
    public static net.minecraft.server.dedicated.DedicatedServerProperties fromFile(java.nio.file.Path);
    protected net.minecraft.server.dedicated.DedicatedServerProperties reload(net.minecraft.core.RegistryAccess, java.util.Properties);
    private static net.minecraft.network.chat.Component parseResourcePackPrompt(java.lang.String);
    private static java.util.Optional<net.minecraft.server.MinecraftServer$ServerResourcePackInfo> getServerPackInfo(java.lang.String, java.lang.String, java.lang.String, java.lang.String, boolean, java.lang.String);
    private static net.minecraft.world.level.DataPackConfig getDatapackConfig(java.lang.String, java.lang.String);
    public static net.minecraft.server.permissions.LevelBasedPermissionSet deserializePermission(java.lang.String);
    public static java.lang.String serializePermission(net.minecraft.server.permissions.LevelBasedPermissionSet);
    public net.minecraft.world.level.levelgen.WorldDimensions createDimensions(net.minecraft.core.HolderLookup$Provider);
    protected net.minecraft.server.dedicated.Settings reload(net.minecraft.core.RegistryAccess, java.util.Properties);
    private static void lambda$parseResourcePackPrompt$0(java.lang.String, java.lang.String);
    private static java.lang.String lambda$new$3(java.lang.String);
    private static com.google.gson.JsonObject lambda$new$2(java.lang.String);
    private static java.lang.Integer lambda$new$1(java.lang.String);
    private static java.lang.Integer lambda$new$0(java.lang.Integer);
    static {};
}
```
