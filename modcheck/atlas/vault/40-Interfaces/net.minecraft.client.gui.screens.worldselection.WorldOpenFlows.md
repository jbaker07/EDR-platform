---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldOpenFlows"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldOpenFlows

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `openWorld(Ljava/lang/String;Ljava/lang/Runnable;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (56, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.worldselection.WorldOpenFlows {
    private static final org.slf4j.Logger LOGGER;
    private static final java.util.UUID WORLD_PACK_ID;
    private final net.minecraft.client.Minecraft minecraft;
    private final net.minecraft.world.level.storage.LevelStorageSource levelSource;
    public net.minecraft.client.gui.screens.worldselection.WorldOpenFlows(net.minecraft.client.Minecraft, net.minecraft.world.level.storage.LevelStorageSource);
    public void createFreshLevel(java.lang.String, net.minecraft.world.level.LevelSettings, net.minecraft.world.level.levelgen.WorldOptions, java.util.function.Function<net.minecraft.core.HolderLookup$Provider, net.minecraft.world.level.levelgen.WorldDimensions>, net.minecraft.client.gui.screens.Screen);
    private net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess createWorldAccess(java.lang.String);
    public void createLevelFromExistingSettings(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional<net.minecraft.world.level.gamerules.GameRules>);
    public net.minecraft.server.WorldStem loadWorldStem(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, boolean, net.minecraft.server.packs.repository.PackRepository) throws java.lang.Exception;
    public com.mojang.datafixers.util.Pair<net.minecraft.world.level.LevelSettings, net.minecraft.client.gui.screens.worldselection.WorldCreationContext> recreateWorldData(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess) throws java.lang.Exception;
    private <D, R> R loadWorldDataBlocking(net.minecraft.server.WorldLoader$PackConfig, net.minecraft.server.WorldLoader$WorldDataSupplier<D>, net.minecraft.server.WorldLoader$ResultFactory<D, R>) throws java.lang.Exception;
    private void askForBackup(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, boolean, java.lang.Runnable, java.lang.Runnable);
    public static void confirmWorldCreation(net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, com.mojang.serialization.Lifecycle, java.lang.Runnable, boolean);
    public void openWorld(java.lang.String, java.lang.Runnable);
    private void openWorldLoadLevelData(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable);
    private void openWorldCheckVersionCompatibility(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.world.level.storage.LevelSummary, com.mojang.serialization.Dynamic<?>, java.lang.Runnable);
    private void createBackupAndOpenWorld(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, java.lang.Runnable, boolean);
    private void upgradeAndOpenWorld(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, java.lang.Runnable);
    private com.mojang.serialization.Dynamic<?> tryFileFixAndReportErrors(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, net.minecraft.util.worldupdate.UpgradeProgress, java.lang.Runnable);
    private void openWorldLoadLevelStem(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, boolean, java.lang.Runnable);
    private void openWorldCheckWorldStemCompatibility(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.server.packs.repository.PackRepository, java.lang.Runnable);
    private void openWorldLoadBundledResourcePack(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.server.packs.repository.PackRepository, java.lang.Runnable);
    private void openWorldCheckDiskSpace(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.client.resources.server.DownloadedPackSource, net.minecraft.server.packs.repository.PackRepository, java.lang.Runnable);
    private void openWorldDoLoad(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.server.packs.repository.PackRepository);
    private java.util.concurrent.CompletableFuture<java.lang.Void> loadBundledResourcePack(net.minecraft.client.resources.server.DownloadedPackSource, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess);
    private java.util.concurrent.CompletableFuture<java.lang.Boolean> promptBundledPackLoadFailure();
    private void lambda$openWorldCheckDiskSpace$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.server.packs.repository.PackRepository, net.minecraft.client.resources.server.DownloadedPackSource, java.lang.Runnable, boolean);
    private java.lang.Void lambda$openWorldLoadBundledResourcePack$3(java.lang.Throwable);
    private void lambda$openWorldLoadBundledResourcePack$2(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.client.resources.server.DownloadedPackSource, net.minecraft.server.packs.repository.PackRepository, java.lang.Runnable, java.lang.Boolean);
    private java.util.concurrent.CompletionStage lambda$openWorldLoadBundledResourcePack$1(java.lang.Throwable);
    private static java.lang.Boolean lambda$openWorldLoadBundledResourcePack$0(java.lang.Void);
    private static void lambda$openWorldCheckWorldStemCompatibility$1(net.minecraft.server.WorldStem, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable);
    private void lambda$openWorldCheckWorldStemCompatibility$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.WorldStem, net.minecraft.server.packs.repository.PackRepository, java.lang.Runnable);
    private void lambda$openWorldLoadLevelStem$1(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, java.lang.Runnable);
    private static void lambda$openWorldLoadLevelStem$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable);
    private void lambda$tryFileFixAndReportErrors$2(java.lang.Runnable, net.minecraft.util.filefix.FailedCleanupFileFixException);
    private void lambda$tryFileFixAndReportErrors$1(net.minecraft.util.filefix.AbortedFileFixException, java.lang.Runnable);
    private void lambda$tryFileFixAndReportErrors$0(java.lang.Runnable);
    private void lambda$upgradeAndOpenWorld$1(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, net.minecraft.util.worldupdate.UpgradeProgress, java.lang.Runnable, boolean, java.lang.Runnable);
    private void lambda$upgradeAndOpenWorld$2(boolean, net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, java.lang.Runnable, java.lang.Runnable);
    private void lambda$upgradeAndOpenWorld$3(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, java.lang.Runnable, java.lang.Runnable, boolean);
    private static void lambda$upgradeAndOpenWorld$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable);
    private void lambda$createBackupAndOpenWorld$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, java.lang.Runnable, java.lang.Boolean);
    private void lambda$openWorldCheckVersionCompatibility$1(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, java.lang.Runnable, boolean, boolean);
    private static void lambda$openWorldCheckVersionCompatibility$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable);
    private void lambda$openWorldLoadLevelData$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable, boolean);
    private static void lambda$confirmWorldCreation$0(java.lang.Runnable, net.minecraft.client.Minecraft, net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, boolean);
    private void lambda$askForBackup$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, java.lang.Runnable, boolean, boolean);
    private static void lambda$askForBackup$1(java.lang.Runnable, java.lang.Boolean);
    private static com.mojang.datafixers.util.Pair lambda$recreateWorldData$1(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess, net.minecraft.client.gui.screens.worldselection.WorldOpenFlows$1Data);
    private static void lambda$recreateWorldData$2(com.mojang.serialization.DataResult$Error);
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$recreateWorldData$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, net.minecraft.server.WorldLoader$DataLoadContext);
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$loadWorldStem$0(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic, net.minecraft.server.WorldLoader$DataLoadContext);
    private void lambda$createWorldAccess$0();
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$createFreshLevel$0(java.util.function.Function, net.minecraft.world.level.LevelSettings, net.minecraft.world.level.levelgen.WorldOptions, net.minecraft.server.WorldLoader$DataLoadContext);
    static {};
}
```
