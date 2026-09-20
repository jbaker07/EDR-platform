---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedServerProperties"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedServerProperties

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

`class` public; extends `net/minecraft/server/dedicated/Settings`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `<init>` | `(Ljava/util/Properties;)V` | name_only | @Redirect at ['FIELD'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (69 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final SHA1 : Ljava/util/regex/Pattern;
private static final COMMA_SPLITTER : Lcom/google/common/base/Splitter;
public static final MANAGEMENT_SERVER_TLS_ENABLED_KEY : Ljava/lang/String;
public static final MANAGEMENT_SERVER_TLS_KEYSTORE_KEY : Ljava/lang/String;
public static final MANAGEMENT_SERVER_TLS_KEYSTORE_PASSWORD_KEY : Ljava/lang/String;
public final onlineMode : Z
public final preventProxyConnections : Z
public final serverIp : Ljava/lang/String;
public final allowFlight : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final motd : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final codeOfConduct : Z
public final bugReportLink : Ljava/lang/String;
public final forceGameMode : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final enforceWhitelist : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final difficulty : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final gameMode : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final levelName : Ljava/lang/String;
public final serverPort : I
public final managementServerEnabled : Z
public final managementServerHost : Ljava/lang/String;
public final managementServerPort : I
public final managementServerSecret : Ljava/lang/String;
public final managementServerTlsEnabled : Z
public final managementServerTlsKeystore : Ljava/lang/String;
public final managementServerTlsKeystorePassword : Ljava/lang/String;
public final managementServerAllowedOrigins : Ljava/lang/String;
public final announcePlayerAchievements : Ljava/lang/Boolean;
public final enableQuery : Z
public final queryPort : I
public final enableRcon : Z
public final rconPort : I
public final rconPassword : Ljava/lang/String;
public final hardcore : Z
public final useNativeTransport : Z
public final spawnProtection : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final opPermissions : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final functionPermissions : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public final maxTickTime : J
public final maxChainedNeighborUpdates : I
public final rateLimitPacketsPerSecond : I
public final commandSpamThresholdSeconds : I
public final chatSpamThresholdSeconds : I
public final viewDistance : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final simulationDistance : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final maxPlayers : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final networkCompressionThreshold : I
public final broadcastRconToOps : Z
public final broadcastConsoleToOps : Z
public final maxWorldSize : I
public final syncChunkWrites : Z
public final regionFileComression : Ljava/lang/String;
public final enableJmxMonitoring : Z
public final enableStatus : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final hideOnlinePlayers : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final entityBroadcastRangePercentage : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final textFilteringConfig : Ljava/lang/String;
public final textFilteringVersion : I
public final serverResourcePackInfo : Ljava/util/Optional;
public final initialDataPackConfiguration : Lnet/minecraft/world/level/DataPackConfig;
public final playerIdleTimeout : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final statusHeartbeatInterval : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final whiteList : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public final enforceSecureProfile : Z
public final logIPs : Z
public final pauseWhenEmptySeconds : Lnet/minecraft/server/dedicated/Settings$MutableValue;
private final worldDimensionData : Lnet/minecraft/server/dedicated/DedicatedServerProperties$WorldDimensionData;
public final worldOptions : Lnet/minecraft/world/level/levelgen/WorldOptions;
public final acceptsTransfers : Lnet/minecraft/server/dedicated/Settings$MutableValue;
public <init>(Ljava/util/Properties;)V
public static fromFile(Ljava/nio/file/Path;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
protected reload(Lnet/minecraft/core/RegistryAccess;Ljava/util/Properties;)Lnet/minecraft/server/dedicated/DedicatedServerProperties;
private static parseResourcePackPrompt(Ljava/lang/String;)Lnet/minecraft/network/chat/Component;
private static getServerPackInfo(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;ZLjava/lang/String;)Ljava/util/Optional;
private static getDatapackConfig(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/world/level/DataPackConfig;
public static deserializePermission(Ljava/lang/String;)Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public static serializePermission(Lnet/minecraft/server/permissions/LevelBasedPermissionSet;)Ljava/lang/String;
public createDimensions(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/levelgen/WorldDimensions;
protected synthetic reload(Lnet/minecraft/core/RegistryAccess;Ljava/util/Properties;)Lnet/minecraft/server/dedicated/Settings;
private static synthetic lambda$parseResourcePackPrompt$0(Ljava/lang/String;Ljava/lang/String;)V
private static synthetic lambda$new$3(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$new$2(Ljava/lang/String;)Lcom/google/gson/JsonObject;
private static synthetic lambda$new$1(Ljava/lang/String;)Ljava/lang/Integer;
private static synthetic lambda$new$0(Ljava/lang/Integer;)Ljava/lang/Integer;
static <clinit>()V
```
