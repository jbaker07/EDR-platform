---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldOpenFlows"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldOpenFlows

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `openWorld` | `(Ljava/lang/String;Ljava/lang/Runnable;)V` | exact | invokevirtual@23 in `TestWorldSaveImpl.lambda$open$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `openWorldCheckWorldStemCompatibility` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `askForBackup` | `(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAcc` | name_only | @WrapOperation at ['NEW'] | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 52 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final WORLD_PACK_ID : Ljava/util/UUID;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final levelSource : Lnet/minecraft/world/level/storage/LevelStorageSource;
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/world/level/storage/LevelStorageSource;)V
public createFreshLevel(Ljava/lang/String;Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/world/level/levelgen/WorldOptions;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/Screen;)V
private createWorldAccess(Ljava/lang/String;)Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;
public createLevelFromExistingSettings(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;)V
public loadWorldStem(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;ZLnet/minecraft/server/packs/repository/PackRepository;)Lnet/minecraft/server/WorldStem;
public recreateWorldData(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;)Lcom/mojang/datafixers/util/Pair;
private loadWorldDataBlocking(Lnet/minecraft/server/WorldLoader$PackConfig;Lnet/minecraft/server/WorldLoader$WorldDataSupplier;Lnet/minecraft/server/WorldLoader$ResultFactory;)Ljava/lang/Object;
private askForBackup(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;ZLjava/lang/Runnable;Ljava/lang/Runnable;)V
public static confirmWorldCreation(Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lcom/mojang/serialization/Lifecycle;Ljava/lang/Runnable;Z)V
public openWorld(Ljava/lang/String;Ljava/lang/Runnable;)V
private openWorldLoadLevelData(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;)V
private openWorldCheckVersionCompatibility(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/world/level/storage/LevelSummary;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;)V
private createBackupAndOpenWorld(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;Z)V
private upgradeAndOpenWorld(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;)V
private tryFileFixAndReportErrors(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/util/worldupdate/UpgradeProgress;Ljava/lang/Runnable;)Lcom/mojang/serialization/Dynamic;
private openWorldLoadLevelStem(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;ZLjava/lang/Runnable;)V
private openWorldCheckWorldStemCompatibility(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;)V
private openWorldLoadBundledResourcePack(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;)V
private openWorldCheckDiskSpace(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/client/resources/server/DownloadedPackSource;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;)V
private openWorldDoLoad(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;)V
private loadBundledResourcePack(Lnet/minecraft/client/resources/server/DownloadedPackSource;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;)Ljava/util/concurrent/CompletableFuture;
private promptBundledPackLoadFailure()Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$openWorldCheckDiskSpace$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/client/resources/server/DownloadedPackSource;Ljava/lang/Runnable;Z)V
private synthetic lambda$openWorldLoadBundledResourcePack$3(Ljava/lang/Throwable;)Ljava/lang/Void;
private synthetic lambda$openWorldLoadBundledResourcePack$2(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/client/resources/server/DownloadedPackSource;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;Ljava/lang/Boolean;)V
private synthetic lambda$openWorldLoadBundledResourcePack$1(Ljava/lang/Throwable;)Ljava/util/concurrent/CompletionStage;
private static synthetic lambda$openWorldLoadBundledResourcePack$0(Ljava/lang/Void;)Ljava/lang/Boolean;
private static synthetic lambda$openWorldCheckWorldStemCompatibility$1(Lnet/minecraft/server/WorldStem;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;)V
private synthetic lambda$openWorldCheckWorldStemCompatibility$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/WorldStem;Lnet/minecraft/server/packs/repository/PackRepository;Ljava/lang/Runnable;)V
private synthetic lambda$openWorldLoadLevelStem$1(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;)V
private static synthetic lambda$openWorldLoadLevelStem$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;)V
private synthetic lambda$tryFileFixAndReportErrors$2(Ljava/lang/Runnable;Lnet/minecraft/util/filefix/FailedCleanupFileFixException;)V
private synthetic lambda$tryFileFixAndReportErrors$1(Lnet/minecraft/util/filefix/AbortedFileFixException;Ljava/lang/Runnable;)V
private synthetic lambda$tryFileFixAndReportErrors$0(Ljava/lang/Runnable;)V
private synthetic lambda$upgradeAndOpenWorld$1(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/util/worldupdate/UpgradeProgress;Ljava/lang/Runnable;ZLjava/lang/Runnable;)V
private synthetic lambda$upgradeAndOpenWorld$2(ZLnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;Ljava/lang/Runnable;)V
private synthetic lambda$upgradeAndOpenWorld$3(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;Ljava/lang/Runnable;Z)V
private static synthetic lambda$upgradeAndOpenWorld$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;)V
private synthetic lambda$createBackupAndOpenWorld$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;Ljava/lang/Boolean;)V
private synthetic lambda$openWorldCheckVersionCompatibility$1(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Ljava/lang/Runnable;ZZ)V
private static synthetic lambda$openWorldCheckVersionCompatibility$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;)V
private synthetic lambda$openWorldLoadLevelData$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;Z)V
private static synthetic lambda$confirmWorldCreation$0(Ljava/lang/Runnable;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Z)V
private synthetic lambda$askForBackup$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Ljava/lang/Runnable;ZZ)V
private static synthetic lambda$askForBackup$1(Ljava/lang/Runnable;Ljava/lang/Boolean;)V
private static synthetic lambda$recreateWorldData$1(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlows$1Data;)Lcom/mojang/datafixers/util/Pair;
private static synthetic lambda$recreateWorldData$2(Lcom/mojang/serialization/DataResult$Error;)V
private static synthetic lambda$recreateWorldData$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
private static synthetic lambda$loadWorldStem$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
private synthetic lambda$createWorldAccess$0()V
private static synthetic lambda$createFreshLevel$0(Ljava/util/function/Function;Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/world/level/levelgen/WorldOptions;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
static <clinit>()V
```
