---
type: "interface"
fqcn: "net.minecraft.server.level.ServerLevel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerLevel

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `net/minecraft/world/level/Level`; implements `net/minecraft/world/level/WorldGenLevel`, `net/minecraft/server/level/ServerEntityGetter`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `dimension` | `()Lnet/minecraft/resources/ResourceKey;` | inherited_exact | invokevirtual@1 in `AttachmentSavedData.lambda$codec$1` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getAllEntities` | `()Ljava/lang/Iterable;` | exact | invokevirtual@96 in `LifecycleEventsImpl.lambda$onInitialize$3` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | inherited_exact | invokevirtual@16 in `BlockApiCacheImpl.getBlockEntity` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | inherited_exact | invokevirtual@86 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | inherited_exact | invokevirtual@2 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokevirtual@35 in `BlockApiCacheImpl.find` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokevirtual@72 in `ServerPlayerGameModeMixin.startBlockBreak` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getChunkSource` | `()Lnet/minecraft/server/level/ServerChunkCache;` | exact | invokevirtual@18 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getChunkSource` | `()Lnet/minecraft/server/level/ServerChunkCache;` | exact | invokevirtual@21 in `BlockEntityMixin.fabric_syncChange` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getChunkSource` | `()Lnet/minecraft/server/level/ServerChunkCache;` | exact | invokevirtual@15 in `PlayerLookup.tracking` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getDataStorage` | `()Lnet/minecraft/world/level/storage/SavedDataStorage;` | exact | invokevirtual@28 in `ServerLevelMixin.createAttachmentsPersistentState` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getGameRules` | `()Lnet/minecraft/world/level/gamerules/GameRules;` | exact | invokevirtual@12 in `EnumRuleCommand.executeAndSetEnum` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getRandom` | `()Lnet/minecraft/util/RandomSource;` | inherited_exact | invokevirtual@64 in `DropperBlockMixin.hookDispense` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@70 in `BlockEntityMixin.fabric_markChanged` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@2 in `FakePlayer.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@5 in `FakePlayerPacketListener.<init>` | unknown | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@1 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `getServer` | `()Lnet/minecraft/server/MinecraftServer;` | exact | invokevirtual@10 in `ServerPlayNetworkAddon.schedule` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `globalAttachments` | `()Lnet/fabricmc/fabric/api/attachment/v1/GlobalAttachments;` | inherited_exact | invokevirtual@12 in `AttachmentSync.lambda$onInitialize$2` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `players` | `()Ljava/util/List;` | exact | invokevirtual@8 in `PlayerLookup.level` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `recipeAccess` | `()Lnet/minecraft/world/item/crafting/RecipeManager;` | exact | invokevirtual@36 in `RecipeSyncImpl.sendRecipes` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | inherited_exact | invokevirtual@58 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | inherited_exact | invokevirtual@13 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Ljava/util/concurrent/Executor;` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| injects_into | `tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject at ['FIELD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `server` | `Lnet/minecraft/server/MinecraftServer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |

## Declared members (39 fields, 191 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final END_SPAWN_POINT : Lnet/minecraft/core/BlockPos;
public static final RAIN_DELAY : Lnet/minecraft/util/valueproviders/IntProvider;
public static final RAIN_DURATION : Lnet/minecraft/util/valueproviders/IntProvider;
private static final THUNDER_DELAY : Lnet/minecraft/util/valueproviders/IntProvider;
public static final THUNDER_DURATION : Lnet/minecraft/util/valueproviders/IntProvider;
private static final LOGGER : Lorg/slf4j/Logger;
private static final EMPTY_TIME_NO_TICK : I
private static final MAX_SCHEDULED_TICKS_PER_TICK : I
private static final ENTITY_COUNTER : Ljava/util/concurrent/atomic/AtomicInteger;
private final players : Ljava/util/List;
private final chunkSource : Lnet/minecraft/server/level/ServerChunkCache;
private final server : Lnet/minecraft/server/MinecraftServer;
private final serverLevelData : Lnet/minecraft/world/level/storage/ServerLevelData;
private final entityTickList : Lnet/minecraft/world/level/entity/EntityTickList;
private final waypointManager : Lnet/minecraft/server/waypoints/ServerWaypointManager;
private environmentAttributes : Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
private final entityManager : Lnet/minecraft/world/level/entity/PersistentEntitySectionManager;
private final gameEventDispatcher : Lnet/minecraft/world/level/gameevent/GameEventDispatcher;
public noSave : Z
private final sleepStatus : Lnet/minecraft/server/players/SleepStatus;
private emptyTime : I
private final portalForcer : Lnet/minecraft/world/level/portal/PortalForcer;
private final blockTicks : Lnet/minecraft/world/ticks/LevelTicks;
private final fluidTicks : Lnet/minecraft/world/ticks/LevelTicks;
private final pathTypesByPosCache : Lnet/minecraft/world/level/pathfinder/PathTypeCache;
private final navigatingMobs : Ljava/util/Set;
private isUpdatingNavigations : Z
protected final raids : Lnet/minecraft/world/entity/raid/Raids;
private final blockEvents : Lit/unimi/dsi/fastutil/objects/ObjectLinkedOpenHashSet;
private final blockEventsToReschedule : Ljava/util/List;
private handlingTick : Z
private final customSpawners : Ljava/util/List;
private dragonFight : Lnet/minecraft/world/level/dimension/end/EnderDragonFight;
private final dragonParts : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private final uncachedBiomeResolver : Lnet/minecraft/world/level/biome/BiomeResolver;
private final structureManager : Lnet/minecraft/world/level/StructureManager;
private final structureCheck : Lnet/minecraft/world/level/levelgen/structure/StructureCheck;
private final tickTime : Z
private final debugSynchronizers : Lnet/minecraft/util/debug/LevelDebugSynchronizers;
public <init>(Lnet/minecraft/server/MinecraftServer;Ljava/util/concurrent/Executor;Lnet/minecraft/world/level/storage/LevelStorageSource$LevelStorageAccess;Lnet/minecraft/world/level/storage/ServerLevelData;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/dimension/LevelStem;ZJLjava/util/List;Z)V
public getNextEntityId()I
public setDragonFight(Lnet/minecraft/world/level/dimension/end/EnderDragonFight;)V
public getUncachedNoiseBiome(III)Lnet/minecraft/core/Holder;
public uncachedBiomeResolver()Lnet/minecraft/world/level/biome/BiomeResolver;
public structureManager()Lnet/minecraft/world/level/StructureManager;
public clockManager()Lnet/minecraft/world/clock/ServerClockManager;
public environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
public setEnvironmentAttributes(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;)Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
public tick(Ljava/util/function/BooleanSupplier;)V
public shouldTickBlocksAt(J)Z
protected tickTime()V
public tickCustomSpawners(Z)V
private wakeUpAllPlayers()V
public tickChunk(Lnet/minecraft/world/level/chunk/LevelChunk;I)V
public tickThunder(Lnet/minecraft/world/level/chunk/LevelChunk;)V
public tickPrecipitation(Lnet/minecraft/core/BlockPos;)V
private findLightningRod(Lnet/minecraft/core/BlockPos;)Ljava/util/Optional;
protected findLightningTargetAround(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public isHandlingTick()Z
public canSleepThroughNights()Z
private announceSleepStatus()V
public updateSleepingPlayerList()V
public getScoreboard()Lnet/minecraft/server/ServerScoreboard;
public getWaypointManager()Lnet/minecraft/server/waypoints/ServerWaypointManager;
public getCurrentDifficultyAt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/DifficultyInstance;
public getMoonBrightness(Lnet/minecraft/core/BlockPos;)F
private prepareWeather(Lnet/minecraft/world/level/saveddata/WeatherData;)V
private advanceWeatherCycle()V
public resetWeatherCycle()V
public resetEmptyTime()V
private tickFluid(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/Fluid;)V
private tickBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V
public tickNonPassenger(Lnet/minecraft/world/entity/Entity;)V
private tickPassenger(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)V
public updateNeighboursOnBlockSet(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public mayInteract(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;)Z
public save(Lnet/minecraft/util/ProgressListener;ZZ)V
private saveLevelData(Z)V
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Ljava/util/function/Predicate;)Ljava/util/List;
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Ljava/util/function/Predicate;Ljava/util/List;)V
public getEntities(Lnet/minecraft/world/level/entity/EntityTypeTest;Ljava/util/function/Predicate;Ljava/util/List;I)V
public getDragons()Ljava/util/List;
public getPlayers(Ljava/util/function/Predicate;)Ljava/util/List;
public getPlayers(Ljava/util/function/Predicate;I)Ljava/util/List;
public getRandomPlayer()Lnet/minecraft/server/level/ServerPlayer;
public addFreshEntity(Lnet/minecraft/world/entity/Entity;)Z
public addWithUUID(Lnet/minecraft/world/entity/Entity;)Z
public addDuringTeleport(Lnet/minecraft/world/entity/Entity;)V
public addNewPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
public addRespawnedPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
private addPlayer(Lnet/minecraft/server/level/ServerPlayer;)V
private addEntity(Lnet/minecraft/world/entity/Entity;)Z
public tryAddFreshEntityWithPassengers(Lnet/minecraft/world/entity/Entity;)Z
public unload(Lnet/minecraft/world/level/chunk/LevelChunk;)V
public removePlayerImmediately(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public destroyBlockProgress(ILnet/minecraft/core/BlockPos;I)V
public playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public globalLevelEvent(ILnet/minecraft/core/BlockPos;I)V
public levelEvent(Lnet/minecraft/world/entity/Entity;ILnet/minecraft/core/BlockPos;I)V
public getLogicalHeight()I
public gameEvent(Lnet/minecraft/core/Holder;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/gameevent/GameEvent$Context;)V
public sendBlockUpdated(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V
public updateNeighborsAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;)V
public updateNeighborsAt(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;)V
public updateNeighborsAtExceptFromFacing(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/redstone/Orientation;)V
public neighborChanged(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;)V
public neighborChanged(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;Z)V
public broadcastEntityEvent(Lnet/minecraft/world/entity/Entity;B)V
public broadcastDamageEvent(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;)V
public getChunkSource()Lnet/minecraft/server/level/ServerChunkCache;
public explode(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/level/ExplosionDamageCalculator;DDDFZLnet/minecraft/world/level/Level$ExplosionInteraction;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/random/WeightedList;Lnet/minecraft/core/Holder;)V
private getDestroyType(Lnet/minecraft/world/level/gamerules/GameRule;)Lnet/minecraft/world/level/Explosion$BlockInteraction;
public blockEvent(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;II)V
private runBlockEvents()V
private doBlockEvent(Lnet/minecraft/world/level/BlockEventData;)Z
public getBlockTicks()Lnet/minecraft/world/ticks/LevelTicks;
public getFluidTicks()Lnet/minecraft/world/ticks/LevelTicks;
public getServer()Lnet/minecraft/server/MinecraftServer;
public getPortalForcer()Lnet/minecraft/world/level/portal/PortalForcer;
public getStructureTemplateManager()Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;DDDIDDDD)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;DDDIDDDDLnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket$RandomizationType;)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;DDDIDDDDDD)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;DDDIDDDDDDLnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket$RandomizationType;)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDIDDDD)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDIDDDDLnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket$RandomizationType;)I
public sendParticles(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDIDDDDDDLnet/minecraft/network/protocol/game/ClientboundLevelParticlesPacket$RandomizationType;)I
public sendParticles(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/core/particles/ParticleOptions;ZZDDDIDDDD)Z
public final sendParticles(Lnet/minecraft/server/level/ServerPlayer;ZDDDLnet/minecraft/network/protocol/Packet;)Z
public getEntity(I)Lnet/minecraft/world/entity/Entity;
public getEntityInAnyDimension(Ljava/util/UUID;)Lnet/minecraft/world/entity/Entity;
public getPlayerInAnyDimension(Ljava/util/UUID;)Lnet/minecraft/world/entity/player/Player;
public getEntityOrPart(I)Lnet/minecraft/world/entity/Entity;
public dragonParts()Ljava/util/Collection;
public findNearestMapStructure(Lnet/minecraft/tags/TagKey;Lnet/minecraft/core/BlockPos;IZ)Lnet/minecraft/core/BlockPos;
public findNearestMapStructure(Lnet/minecraft/core/HolderSet;Lnet/minecraft/core/BlockPos;IZ)Lnet/minecraft/core/BlockPos;
public findClosestBiome3d(Ljava/util/function/Predicate;Lnet/minecraft/core/BlockPos;III)Lcom/mojang/datafixers/util/Pair;
public getWorldBorder()Lnet/minecraft/world/level/border/WorldBorder;
public recipeAccess()Lnet/minecraft/world/item/crafting/RecipeManager;
public tickRateManager()Lnet/minecraft/world/TickRateManager;
public noSave()Z
public getDataStorage()Lnet/minecraft/world/level/storage/SavedDataStorage;
public getMapData(Lnet/minecraft/world/level/saveddata/maps/MapId;)Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData;
public setMapData(Lnet/minecraft/world/level/saveddata/maps/MapId;Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData;)V
public getFreeMapId()Lnet/minecraft/world/level/saveddata/maps/MapId;
public setRespawnData(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)V
public getRespawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public getForceLoadedChunks()Lit/unimi/dsi/fastutil/longs/LongSet;
public setChunkForced(IIZ)Z
public players()Ljava/util/List;
public updatePOIOnBlockStateChange(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
public getPoiManager()Lnet/minecraft/world/entity/ai/village/poi/PoiManager;
public isVillage(Lnet/minecraft/core/BlockPos;)Z
public isVillage(Lnet/minecraft/core/SectionPos;)Z
public isCloseToVillage(Lnet/minecraft/core/BlockPos;I)Z
public sectionsToVillage(Lnet/minecraft/core/SectionPos;)I
public getRaids()Lnet/minecraft/world/entity/raid/Raids;
public getRaidAt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/entity/raid/Raid;
public isRaided(Lnet/minecraft/core/BlockPos;)Z
public onReputationEvent(Lnet/minecraft/world/entity/ai/village/ReputationEventType;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/ReputationEventHandler;)V
public saveDebugReport(Ljava/nio/file/Path;)V
private static dumpEntities(Ljava/io/Writer;Ljava/lang/Iterable;)V
private dumpBlockEntityTickers(Ljava/io/Writer;)V
public clearBlockEvents(Lnet/minecraft/world/level/levelgen/structure/BoundingBox;)V
public getAllEntities()Ljava/lang/Iterable;
public toString()Ljava/lang/String;
public isFlat()Z
public getSeed()J
public getDragonFight()Lnet/minecraft/world/level/dimension/end/EnderDragonFight;
public getWeatherData()Lnet/minecraft/world/level/saveddata/WeatherData;
public getLevel()Lnet/minecraft/server/level/ServerLevel;
public getWatchdogStats()Ljava/lang/String;
private static getTypeCount(Ljava/lang/Iterable;Ljava/util/function/Function;)Ljava/lang/String;
protected getEntities()Lnet/minecraft/world/level/entity/LevelEntityGetter;
public addLegacyChunkEntities(Ljava/util/stream/Stream;)V
public addWorldGenChunkEntities(Ljava/util/stream/Stream;)V
public startTickingChunk(Lnet/minecraft/world/level/chunk/LevelChunk;)V
public onStructureStartsAvailable(Lnet/minecraft/world/level/chunk/ChunkAccess;)V
public getPathTypeCache()Lnet/minecraft/world/level/pathfinder/PathTypeCache;
public waitForEntities(Lnet/minecraft/world/level/ChunkPos;I)V
public isSpawningMonsters()Z
public close()V
public gatherChunkSourceStats()Ljava/lang/String;
public areEntitiesLoaded(J)Z
public isPositionTickingWithEntitiesLoaded(J)Z
public isPositionEntityTicking(Lnet/minecraft/core/BlockPos;)Z
public areEntitiesActuallyLoadedAndTicking(Lnet/minecraft/world/level/ChunkPos;)Z
public anyPlayerCloseEnoughForSpawning(Lnet/minecraft/core/BlockPos;)Z
public anyPlayerCloseEnoughForSpawning(Lnet/minecraft/world/level/ChunkPos;)Z
public canSpreadFireAround(Lnet/minecraft/core/BlockPos;)Z
public canSpawnEntitiesInChunk(Lnet/minecraft/world/level/ChunkPos;)Z
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public getGameRules()Lnet/minecraft/world/level/gamerules/GameRules;
public fillReportDetails(Lnet/minecraft/CrashReport;)Lnet/minecraft/CrashReportCategory;
public getSeaLevel()I
public onBlockEntityAdded(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public debugSynchronizers()Lnet/minecraft/util/debug/LevelDebugSynchronizers;
public isAllowedToEnterPortal(Lnet/minecraft/world/level/Level;)Z
public isPvpAllowed()Z
public isCommandBlockEnabled()Z
public isSpawnerBlockEnabled()Z
public synthetic clockManager()Lnet/minecraft/world/clock/ClockManager;
public synthetic recipeAccess()Lnet/minecraft/world/item/crafting/RecipeAccess;
public synthetic getScoreboard()Lnet/minecraft/world/scores/Scoreboard;
public synthetic getChunkSource()Lnet/minecraft/world/level/chunk/ChunkSource;
public synthetic environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeReader;
public synthetic getFluidTicks()Lnet/minecraft/world/ticks/LevelTickAccess;
public synthetic getBlockTicks()Lnet/minecraft/world/ticks/LevelTickAccess;
private synthetic lambda$fillReportDetails$1(Lnet/minecraft/world/level/saveddata/WeatherData;)Ljava/lang/String;
private synthetic lambda$fillReportDetails$0()Ljava/lang/String;
private synthetic lambda$waitForEntities$0(Ljava/util/List;)Z
private synthetic lambda$onStructureStartsAvailable$0(Lnet/minecraft/world/level/chunk/ChunkAccess;)V
private static synthetic lambda$getTypeCount$0(Lit/unimi/dsi/fastutil/objects/Object2IntMap$Entry;)Ljava/lang/String;
private static synthetic lambda$getWatchdogStats$0(Lnet/minecraft/world/entity/Entity;)Ljava/lang/String;
private static synthetic lambda$clearBlockEvents$0(Lnet/minecraft/world/level/levelgen/structure/BoundingBox;Lnet/minecraft/world/level/BlockEventData;)Z
private synthetic lambda$updatePOIOnBlockStateChange$2(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Holder;)V
private synthetic lambda$updatePOIOnBlockStateChange$3(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Holder;)V
private synthetic lambda$updatePOIOnBlockStateChange$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Holder;)V
private synthetic lambda$updatePOIOnBlockStateChange$1(Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$globalLevelEvent$0(Lnet/minecraft/core/BlockPos;IILnet/minecraft/server/level/ServerPlayer;)V
private static synthetic lambda$getEntities$0(Ljava/util/function/Predicate;Ljava/util/List;ILnet/minecraft/world/entity/Entity;)Lnet/minecraft/util/Continuation;
private synthetic lambda$findLightningTargetAround$0(Lnet/minecraft/world/entity/LivingEntity;)Z
private static synthetic lambda$findLightningRod$2(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
private synthetic lambda$findLightningRod$1(Lnet/minecraft/core/BlockPos;)Z
private static synthetic lambda$findLightningRod$0(Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$wakeUpAllPlayers$0(Lnet/minecraft/server/level/ServerPlayer;)V
private synthetic lambda$tick$1(Lnet/minecraft/core/BlockPos;)V
private synthetic lambda$tick$0(Lnet/minecraft/world/TickRateManager;Lnet/minecraft/util/profiling/ProfilerFiller;Lnet/minecraft/world/entity/Entity;)V
static <clinit>()V
```
