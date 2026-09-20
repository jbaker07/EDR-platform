---
type: "interface"
fqcn: "net.minecraft.world.level.storage.LevelStorageSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.LevelStorageSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getBaseDir()Ljava/nio/file/Path;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (48, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.LevelStorageSource {
    private static final org.slf4j.Logger LOGGER;
    public static final java.lang.String TAG_DATA;
    private static final java.nio.file.PathMatcher NO_SYMLINKS_ALLOWED;
    public static final java.lang.String ALLOWED_SYMLINKS_CONFIG_NAME;
    private static final int DISK_SPACE_WARNING_THRESHOLD;
    private static final net.minecraft.network.chat.Component LOAD_FOLDER_ACCESS_MESSAGE;
    private final java.nio.file.Path baseDir;
    private final java.nio.file.Path backupDir;
    private final com.mojang.datafixers.DataFixer fixerUpper;
    private final net.minecraft.world.level.validation.DirectoryValidator worldDirValidator;
    public net.minecraft.world.level.storage.LevelStorageSource(java.nio.file.Path, java.nio.file.Path, net.minecraft.world.level.validation.DirectoryValidator, com.mojang.datafixers.DataFixer);
    public static net.minecraft.world.level.validation.DirectoryValidator parseValidator(java.nio.file.Path);
    public static net.minecraft.world.level.storage.LevelStorageSource createDefault(java.nio.file.Path);
    public static net.minecraft.world.level.WorldDataConfiguration readDataConfig(com.mojang.serialization.Dynamic<?>);
    public static net.minecraft.server.WorldLoader$PackConfig getPackConfig(com.mojang.serialization.Dynamic<?>, net.minecraft.server.packs.repository.PackRepository, boolean);
    public static net.minecraft.world.level.storage.LevelDataAndDimensions getLevelDataAndDimensions(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, com.mojang.serialization.Dynamic<?>, net.minecraft.world.level.WorldDataConfiguration, net.minecraft.core.Registry<net.minecraft.world.level.dimension.LevelStem>, net.minecraft.core.HolderLookup$Provider);
    public static <T extends net.minecraft.world.level.saveddata.SavedData> com.mojang.serialization.DataResult<T> readExistingSavedData(net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess, net.minecraft.core.HolderLookup$Provider, net.minecraft.world.level.saveddata.SavedDataType<T>);
    public static void writeGameRules(net.minecraft.world.level.storage.WorldData, java.nio.file.Path, net.minecraft.world.level.gamerules.GameRules) throws java.io.IOException;
    public static void writeWorldGenSettings(net.minecraft.core.RegistryAccess, java.nio.file.Path, net.minecraft.world.level.levelgen.WorldGenSettings) throws java.io.IOException;
    private static <T> void writeSavedData(java.nio.file.Path, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, net.minecraft.world.level.saveddata.SavedDataType<?>, com.mojang.serialization.Codec<T>, T) throws java.io.IOException;
    public java.lang.String getName();
    public net.minecraft.world.level.storage.LevelStorageSource$LevelCandidates findLevelCandidates() throws net.minecraft.world.level.storage.LevelStorageException;
    public java.util.concurrent.CompletableFuture<java.util.List<net.minecraft.world.level.storage.LevelSummary>> loadLevelSummaries(net.minecraft.world.level.storage.LevelStorageSource$LevelCandidates);
    private int getStorageVersion();
    private static net.minecraft.nbt.CompoundTag readLevelDataTagRaw(java.nio.file.Path) throws java.io.IOException;
    private net.minecraft.world.level.storage.LevelSummary readLevelSummary(net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory, boolean);
    private static long getFileModificationTime(net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory);
    private static java.time.Instant getFileModificationTime(java.nio.file.Path);
    private net.minecraft.world.level.storage.LevelSummary makeLevelSummary(com.mojang.serialization.Dynamic<?>, net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory, boolean, int);
    private static net.minecraft.world.flag.FeatureFlagSet parseFeatureFlagsFromSummary(com.mojang.serialization.Dynamic<?>);
    private static net.minecraft.nbt.Tag readLightweightData(java.nio.file.Path) throws java.io.IOException;
    public boolean isNewLevelIdAcceptable(java.lang.String);
    public boolean levelExists(java.lang.String);
    public java.nio.file.Path getLevelPath(java.lang.String);
    public java.nio.file.Path getBaseDir();
    public java.nio.file.Path getBackupPath();
    public net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess validateAndCreateAccess(java.lang.String) throws java.io.IOException, net.minecraft.world.level.validation.ContentValidationException;
    public net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess createAccess(java.lang.String) throws java.io.IOException;
    public net.minecraft.world.level.validation.DirectoryValidator getWorldDirValidator();
    private static void lambda$parseFeatureFlagsFromSummary$1(net.minecraft.resources.Identifier);
    private static java.util.stream.Stream lambda$parseFeatureFlagsFromSummary$0(com.mojang.serialization.Dynamic);
    private static java.util.List lambda$loadLevelSummaries$1(java.util.List);
    private net.minecraft.world.level.storage.LevelSummary lambda$loadLevelSummaries$0(net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory);
    private static boolean lambda$findLevelCandidates$1(net.minecraft.world.level.storage.LevelStorageSource$LevelDirectory);
    private static boolean lambda$findLevelCandidates$0(java.nio.file.Path);
    private static net.minecraft.world.level.levelgen.WorldGenSettings lambda$getLevelDataAndDimensions$0(net.minecraft.core.Registry, com.mojang.serialization.DataResult$Error);
    private static boolean lambda$static$0(java.nio.file.Path);
    static {};
}
```
