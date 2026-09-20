---
type: "interface"
fqcn: "net.minecraft.world.level.storage.PrimaryLevelData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.PrimaryLevelData

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createTag(Ljava/util/UUID;)Lnet/minecraft/nbt/CompoundTag;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (62, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.PrimaryLevelData implements net.minecraft.world.level.storage.ServerLevelData,net.minecraft.world.level.storage.WorldData {
    private static final org.slf4j.Logger LOGGER;
    public static final java.lang.String LEVEL_NAME;
    protected static final java.lang.String OLD_PLAYER;
    protected static final java.lang.String SINGLEPLAYER_UUID;
    protected static final java.lang.String OLD_WORLD_GEN_SETTINGS;
    private net.minecraft.world.level.LevelSettings settings;
    private final net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty specialWorldProperty;
    private final com.mojang.serialization.Lifecycle worldGenSettingsLifecycle;
    private net.minecraft.world.level.storage.LevelData$RespawnData respawnData;
    private long gameTime;
    private final java.util.UUID singlePlayerUUID;
    private final int version;
    private boolean initialized;
    private final java.util.Set<java.lang.String> knownServerBrands;
    private boolean wasModded;
    private final java.util.Set<java.lang.String> removedFeatureFlags;
    private java.util.List<java.lang.Integer> versionHistory;
    private net.minecraft.world.level.storage.PrimaryLevelData(java.util.UUID, boolean, net.minecraft.world.level.storage.LevelData$RespawnData, long, int, boolean, java.util.Set<java.lang.String>, java.util.Set<java.lang.String>, net.minecraft.world.level.LevelSettings, net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty, com.mojang.serialization.Lifecycle, java.util.List<java.lang.Integer>);
    public net.minecraft.world.level.storage.PrimaryLevelData(net.minecraft.world.level.LevelSettings, net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty, com.mojang.serialization.Lifecycle);
    public static <T> net.minecraft.world.level.storage.PrimaryLevelData parse(com.mojang.serialization.Dynamic<T>, net.minecraft.world.level.LevelSettings, net.minecraft.world.level.storage.PrimaryLevelData$SpecialWorldProperty, com.mojang.serialization.Lifecycle);
    public net.minecraft.nbt.CompoundTag createTag(java.util.UUID);
    private void setTagData(net.minecraft.nbt.CompoundTag, java.util.UUID);
    public static void writeLastPlayed(net.minecraft.nbt.CompoundTag);
    public static com.mojang.serialization.Dynamic<?> writeLastPlayed(com.mojang.serialization.Dynamic<?>);
    public static void writeVersionTag(net.minecraft.nbt.CompoundTag);
    public static com.mojang.serialization.Dynamic<?> writeVersionTag(com.mojang.serialization.Dynamic<?>);
    public void writeVersionHistory(net.minecraft.nbt.CompoundTag);
    private static net.minecraft.nbt.ListTag stringCollectionToTag(java.util.Set<java.lang.String>);
    public net.minecraft.world.level.storage.LevelData$RespawnData getRespawnData();
    public long getGameTime();
    public java.util.UUID getSinglePlayerUUID();
    public void setGameTime(long);
    public void setSpawn(net.minecraft.world.level.storage.LevelData$RespawnData);
    public java.lang.String getLevelName();
    public int getVersion();
    public net.minecraft.world.level.GameType getGameType();
    public void setGameType(net.minecraft.world.level.GameType);
    public boolean isHardcore();
    public boolean isAllowCommands();
    public void setAllowCommands(boolean);
    public boolean isInitialized();
    public void setInitialized(boolean);
    public net.minecraft.world.Difficulty getDifficulty();
    public void setDifficulty(net.minecraft.world.Difficulty);
    public boolean isDifficultyLocked();
    public void setDifficultyLocked(boolean);
    public void fillCrashReportCategory(net.minecraft.CrashReportCategory, net.minecraft.world.level.LevelHeightAccessor);
    public boolean isFlatWorld();
    public boolean isDebugWorld();
    public com.mojang.serialization.Lifecycle worldGenSettingsLifecycle();
    public net.minecraft.world.level.WorldDataConfiguration getDataConfiguration();
    public void setDataConfiguration(net.minecraft.world.level.WorldDataConfiguration);
    public void setModdedInfo(java.lang.String, boolean);
    public boolean wasModded();
    public java.util.Set<java.lang.String> getKnownServerBrands();
    public java.util.Set<java.lang.String> getRemovedFeatureFlags();
    public net.minecraft.world.level.storage.ServerLevelData overworldData();
    public net.minecraft.world.level.LevelSettings getLevelSettings();
    private static java.lang.Integer lambda$parse$2(com.mojang.serialization.Dynamic);
    private static java.util.stream.Stream lambda$parse$1(com.mojang.serialization.Dynamic);
    private static java.util.stream.Stream lambda$parse$0(com.mojang.serialization.Dynamic);
    static {};
}
```
