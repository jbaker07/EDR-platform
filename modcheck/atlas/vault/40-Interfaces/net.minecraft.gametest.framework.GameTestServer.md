---
type: "interface"
fqcn: "net.minecraft.gametest.framework.GameTestServer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.GameTestServer

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `create(Ljava/lang/Thread;Lnet/minecraft/world/level/storage/LevelS` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| injects_into | `isDedicatedServer` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| wraps | `create` | `@Redirect at NEW (Ljava/util/List;Ljava/util/List;)Lnet/minecraft/world/level/Da` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (49, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.gametest.framework.GameTestServer extends net.minecraft.server.MinecraftServer {
    private static final org.slf4j.Logger LOGGER;
    private static final int PROGRESS_REPORT_INTERVAL;
    private static final int TEST_POSITION_RANGE;
    private static final net.minecraft.server.Services NO_SERVICES;
    private static final net.minecraft.world.flag.FeatureFlagSet ENABLED_FEATURES;
    private final net.minecraft.util.debugchart.LocalSampleLogger sampleLogger;
    private final java.util.Optional<java.lang.String> testSelection;
    private final boolean verify;
    private final int repeatCount;
    private java.util.List<net.minecraft.gametest.framework.GameTestBatch> testBatches;
    private final com.google.common.base.Stopwatch stopwatch;
    private static final net.minecraft.world.level.levelgen.WorldOptions WORLD_OPTIONS;
    private net.minecraft.gametest.framework.MultipleTestTracker testTracker;
    public static net.minecraft.gametest.framework.GameTestServer create(java.lang.Thread, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, java.util.Optional<java.lang.String>, boolean, int);
    private net.minecraft.gametest.framework.GameTestServer(java.lang.Thread, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.repository.PackRepository, net.minecraft.server.WorldStem, java.util.Optional<java.lang.String>, boolean, int);
    protected boolean initServer();
    private java.util.List<net.minecraft.gametest.framework.GameTestBatch> evaluateTestsToRun(net.minecraft.server.MinecraftServer);
    private static java.util.stream.Stream<net.minecraft.gametest.framework.GameTestInfo> rotateAndMultiply(net.minecraft.core.Holder$Reference<net.minecraft.gametest.framework.GameTestInstance>, net.minecraft.server.level.ServerLevel);
    public static java.util.stream.Stream<net.minecraft.core.Holder$Reference<net.minecraft.gametest.framework.GameTestInstance>> getTestsForSelection(net.minecraft.core.RegistryAccess, java.lang.String);
    private java.util.stream.Stream<net.minecraft.gametest.framework.GameTestInfo> multiplyTest(net.minecraft.core.Holder$Reference<net.minecraft.gametest.framework.GameTestInstance>, net.minecraft.server.level.ServerLevel);
    protected void tickServer(java.util.function.BooleanSupplier);
    private static void logFailedTest(net.minecraft.gametest.framework.GameTestInfo);
    protected net.minecraft.util.debugchart.SampleLogger getTickTimeLogger();
    public boolean isTickTimeLoggingEnabled();
    protected void waitUntilNextTick();
    public net.minecraft.SystemReport fillServerSystemReport(net.minecraft.SystemReport);
    protected void onServerExit();
    protected void onServerCrash(net.minecraft.CrashReport);
    private void startTests(net.minecraft.server.level.ServerLevel);
    private boolean haveTestsStarted();
    public boolean isHardcore();
    public net.minecraft.server.permissions.LevelBasedPermissionSet operatorUserPermissions();
    public net.minecraft.server.permissions.PermissionSet getFunctionCompilationPermissions();
    public boolean shouldRconBroadcast();
    public boolean isDedicatedServer();
    public int getRateLimitPacketsPerSecond();
    public int getCommandSpamThresholdSeconds();
    public int getChatSpamThresholdSeconds();
    public boolean useNativeTransport();
    public boolean isPublished();
    public boolean shouldInformAdmins();
    public boolean isSingleplayerOwner(net.minecraft.server.players.NameAndId);
    public int getMaxPlayers();
    private static net.minecraft.core.BlockPos lambda$startTests$0(net.minecraft.core.BlockPos, net.minecraft.resources.ResourceKey);
    private static boolean lambda$evaluateTestsToRun$1(net.minecraft.core.Holder$Reference);
    private static boolean lambda$evaluateTestsToRun$0(net.minecraft.core.Holder$Reference);
    private static java.util.concurrent.CompletableFuture lambda$create$0(net.minecraft.server.WorldLoader$InitConfig, net.minecraft.world.level.LevelSettings, java.util.concurrent.Executor);
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$create$1(net.minecraft.world.level.LevelSettings, net.minecraft.server.WorldLoader$DataLoadContext);
    static {};
}
```
