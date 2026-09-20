---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelStorageSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelStorageSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBaseDir` | `()Ljava/nio/file/Path;` | exact | invokevirtual@89 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (10 fields, 38 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final TAG_DATA : Ljava/lang/String;
private static final NO_SYMLINKS_ALLOWED : Ljava/nio/file/PathMatcher;
public static final ALLOWED_SYMLINKS_CONFIG_NAME : Ljava/lang/String;
private static final DISK_SPACE_WARNING_THRESHOLD : I
private static final LOAD_FOLDER_ACCESS_MESSAGE : Lnet/minecraft/network/chat/Component;
private final baseDir : Ljava/nio/file/Path;
private final backupDir : Ljava/nio/file/Path;
private final fixerUpper : Lcom/mojang/datafixers/DataFixer;
private final worldDirValidator : Lnet/minecraft/world/level/validation/DirectoryValidator;
public <init>(Ljava/nio/file/Path;Ljava/nio/file/Path;Lnet/minecraft/world/level/validation/DirectoryValidator;Lcom/mojang/datafixers/DataFixer;)V
public static parseValidator(Ljava/nio/file/Path;)Lnet/minecraft/world/level/validation/DirectoryValidator;
public static createDefault(Ljava/nio/file/Path;)Lnet/minecraft/world/level/storage/LevelStorageSource;
public static readDataConfig(Lcom/mojang/serialization/Dynamic;)Lnet/minecraft/world/level/WorldDataConfiguration;
public static getPackConfig(Lcom/mojang/serialization/Dynamic;Lnet/minecraft/server/packs/repository/PackRepository;Z)Lnet/minecraft/server/WorldLoader$PackConfig;
public static getLevelDataAndDimensions(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lcom/mojang/serialization/Dynamic;Lnet/minecraft/world/level/WorldDataConfiguration;Lnet/minecraft/core/Registry;Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/storage/LevelDataAndDimensions;
public static readExistingSavedData(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/level/saveddata/SavedDataType;)Lcom/mojang/serialization/DataResult;
public static writeGameRules(Lnet/minecraft/world/level/storage/WorldData;Ljava/nio/file/Path;Lnet/minecraft/world/level/gamerules/GameRules;)V
public static writeWorldGenSettings(Lnet/minecraft/core/RegistryAccess;Ljava/nio/file/Path;Lnet/minecraft/world/level/levelgen/WorldGenSettings;)V
private static writeSavedData(Ljava/nio/file/Path;Lcom/mojang/serialization/DynamicOps;Lnet/minecraft/world/level/saveddata/SavedDataType;Lcom/mojang/serialization/Codec;Ljava/lang/Object;)V
public getName()Ljava/lang/String;
public findLevelCandidates()Lnet/minecraft/world/level/storage/LevelStorageSource$LevelCandidates;
public loadLevelSummaries(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelCandidates;)Ljava/util/concurrent/CompletableFuture;
private getStorageVersion()I
private static readLevelDataTagRaw(Ljava/nio/file/Path;)Lnet/minecraft/nbt/CompoundTag;
private readLevelSummary(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;Z)Lnet/minecraft/world/level/storage/LevelSummary;
private static getFileModificationTime(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;)J
private static getFileModificationTime(Ljava/nio/file/Path;)Ljava/time/Instant;
private makeLevelSummary(Lcom/mojang/serialization/Dynamic;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;ZI)Lnet/minecraft/world/level/storage/LevelSummary;
private static parseFeatureFlagsFromSummary(Lcom/mojang/serialization/Dynamic;)Lnet/minecraft/world/flag/FeatureFlagSet;
private static readLightweightData(Ljava/nio/file/Path;)Lnet/minecraft/nbt/Tag;
public isNewLevelIdAcceptable(Ljava/lang/String;)Z
public levelExists(Ljava/lang/String;)Z
public getLevelPath(Ljava/lang/String;)Ljava/nio/file/Path;
public getBaseDir()Ljava/nio/file/Path;
public getBackupPath()Ljava/nio/file/Path;
public validateAndCreateAccess(Ljava/lang/String;)Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;
public createAccess(Ljava/lang/String;)Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;
public getWorldDirValidator()Lnet/minecraft/world/level/validation/DirectoryValidator;
private static synthetic lambda$parseFeatureFlagsFromSummary$1(Lnet/minecraft/resources/Identifier;)V
private static synthetic lambda$parseFeatureFlagsFromSummary$0(Lcom/mojang/serialization/Dynamic;)Ljava/util/stream/Stream;
private static synthetic lambda$loadLevelSummaries$1(Ljava/util/List;)Ljava/util/List;
private synthetic lambda$loadLevelSummaries$0(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;)Lnet/minecraft/world/level/storage/LevelSummary;
private static synthetic lambda$findLevelCandidates$1(Lnet/minecraft/world/level/storage/LevelStorageSource$LevelDirectory;)Z
private static synthetic lambda$findLevelCandidates$0(Ljava/nio/file/Path;)Z
private static synthetic lambda$getLevelDataAndDimensions$0(Lnet/minecraft/core/Registry;Lcom/mojang/serialization/DataResult$Error;)Lnet/minecraft/world/level/levelgen/WorldGenSettings;
private static synthetic lambda$static$0(Ljava/nio/file/Path;)Z
static <clinit>()V
```
