---
type: "interface"
fqcn: "net.minecraft.gametest.framework.GameTestServer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.GameTestServer

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

`class` public; extends `net/minecraft/server/MinecraftServer`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | exact | invokestatic@8 in `FabricGameTestRunner.lambda$runHeadlessServer$0` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `isDedicatedServer` | `()Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| wraps | `create` | `(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSour` | name_only | @Redirect at ['NEW'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (13 fields, 36 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final PROGRESS_REPORT_INTERVAL : I
private static final TEST_POSITION_RANGE : I
private static final NO_SERVICES : Lnet/minecraft/server/Services;
private static final ENABLED_FEATURES : Lnet/minecraft/world/flag/FeatureFlagSet;
private final sampleLogger : Lnet/minecraft/util/debugchart/LocalSampleLogger;
private final testSelection : Ljava/util/Optional;
private final verify : Z
private final repeatCount : I
private testBatches : Ljava/util/List;
private final stopwatch : Lcom/google/common/base/Stopwatch;
private static final WORLD_OPTIONS : Lnet/minecraft/world/level/levelgen/WorldOptions;
private testTracker : Lnet/minecraft/gametest/framework/MultipleTestTracker;
public static create(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/util/Optional;ZI)Lnet/minecraft/gametest/framework/GameTestServer;
private <init>(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/server/WorldStem;Ljava/util/Optional;ZI)V
protected initServer()Z
private evaluateTestsToRun(Lnet/minecraft/server/MinecraftServer;)Ljava/util/List;
private static rotateAndMultiply(Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/server/level/ServerLevel;)Ljava/util/stream/Stream;
public static getTestsForSelection(Lnet/minecraft/core/RegistryAccess;Ljava/lang/String;)Ljava/util/stream/Stream;
private multiplyTest(Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/server/level/ServerLevel;)Ljava/util/stream/Stream;
protected tickServer(Ljava/util/function/BooleanSupplier;)V
private static logFailedTest(Lnet/minecraft/gametest/framework/GameTestInfo;)V
protected getTickTimeLogger()Lnet/minecraft/util/debugchart/SampleLogger;
public isTickTimeLoggingEnabled()Z
protected waitUntilNextTick()V
public fillServerSystemReport(Lnet/minecraft/SystemReport;)Lnet/minecraft/SystemReport;
protected onServerExit()V
protected onServerCrash(Lnet/minecraft/CrashReport;)V
private startTests(Lnet/minecraft/server/level/ServerLevel;)V
private haveTestsStarted()Z
public isHardcore()Z
public operatorUserPermissions()Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public getFunctionCompilationPermissions()Lnet/minecraft/server/permissions/PermissionSet;
public shouldRconBroadcast()Z
public isDedicatedServer()Z
public getRateLimitPacketsPerSecond()I
public getCommandSpamThresholdSeconds()I
public getChatSpamThresholdSeconds()I
public useNativeTransport()Z
public isPublished()Z
public shouldInformAdmins()Z
public isSingleplayerOwner(Lnet/minecraft/server/players/NameAndId;)Z
public getMaxPlayers()I
private static synthetic lambda$startTests$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/BlockPos;
private static synthetic lambda$evaluateTestsToRun$1(Lnet/minecraft/core/Holder$Reference;)Z
private static synthetic lambda$evaluateTestsToRun$0(Lnet/minecraft/core/Holder$Reference;)Z
private static synthetic lambda$create$0(Lnet/minecraft/server/WorldLoader$InitConfig;Lnet/minecraft/world/level/LevelSettings;Ljava/util/concurrent/Executor;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$create$1(Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
static <clinit>()V
```
