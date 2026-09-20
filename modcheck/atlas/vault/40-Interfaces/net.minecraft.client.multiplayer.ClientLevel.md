---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientLevel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientLevel

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `net/minecraft/world/level/Level`; implements `net/minecraft/client/renderer/block/BlockAndTintGetter`, `net/minecraft/client/multiplayer/CacheSlot$Cleaner`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `calculateBlockTint` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | declared |
| calls | `dimension` | `()Lnet/minecraft/resources/ResourceKey;` | inherited_exact | invokevirtual@35 in `TestServerConnectionImpl.getServerLevel` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@26 in `TestDedicatedServerConnectionImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | exact | invokevirtual@26 in `TestSingleplayerContextImpl.lambda$close$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `entitiesForRendering` | `()Ljava/lang/Iterable;` | exact | invokevirtual@11 in `ClientPacketListenerMixin.onPlayerRespawn` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `entitiesForRendering` | `()Ljava/lang/Iterable;` | exact | invokevirtual@11 in `ClientPacketListenerMixin.onGameJoin` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `entitiesForRendering` | `()Ljava/lang/Iterable;` | exact | invokevirtual@11 in `ClientPacketListenerMixin.onClearLevel` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getBiome` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Holder;` | inherited_exact | invokevirtual@5 in `RenderSectionRegionMixin.getBiomeFabric` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getChunk` | `(II)Lnet/minecraft/world/level/chunk/LevelChunk;` | inherited_exact | invokevirtual@7 in `RenderRegionCacheMixin.copyDataForChunk` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |
| calls | `getChunk` | `(IILnet/minecraft/world/level/chunk/status/ChunkStatus;Z)Lnet/minecraf` | inherited_exact | invokevirtual@91 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getChunkSource` | `()Lnet/minecraft/client/multiplayer/ClientChunkCache;` | exact | invokevirtual@20 in `TestServerConnectionImpl.areChunksLoaded` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | inherited_exact | invokevirtual@26 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `registryAccess` | `()Lnet/minecraft/core/RegistryAccess;` | inherited_exact | invokevirtual@38 in `ClientTagsImpl.getRegistry` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| injects_into | `clearTintCaches` | `()V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `getBlockTint` | `(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver` | exact | @ModifyExpressionValue at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `onChunkLoaded` | `(Lnet/minecraft/world/level/ChunkPos;)V` | exact | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| injects_into | `tick` | `(Ljava/util/function/BooleanSupplier;)V` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `tickEntities` | `()V` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `connection` | `Lnet/minecraft/client/multiplayer/ClientPacketListener;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | declared |

## Declared members (34 fields, 130 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DEFAULT_QUIT_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final FLUID_PARTICLE_SPAWN_OFFSET : D
private static final NORMAL_LIGHT_UPDATES_PER_FRAME : I
private static final LIGHT_UPDATE_QUEUE_SIZE_THRESHOLD : I
private static final RAIN_PARTICLES_PER_BLOCK : F
private static final RAIN_RADIUS : I
private final tickingEntities : Lnet/minecraft/world/level/entity/EntityTickList;
private final entityStorage : Lnet/minecraft/world/level/entity/TransientEntitySectionManager;
private final connection : Lnet/minecraft/client/multiplayer/ClientPacketListener;
private final levelExtractor : Lnet/minecraft/client/renderer/extract/LevelExtractor;
private final levelEventHandler : Lnet/minecraft/client/renderer/LevelEventHandler;
private final clientLevelData : Lnet/minecraft/client/multiplayer/ClientLevel$ClientLevelData;
private final tickRateManager : Lnet/minecraft/world/TickRateManager;
private final endFlashState : Lnet/minecraft/client/renderer/EndFlashState;
private final minecraft : Lnet/minecraft/client/Minecraft;
private final players : Ljava/util/List;
private final dragonParts : Ljava/util/List;
private final mapData : Ljava/util/Map;
private skyFlashTime : I
private rainSoundTime : I
private final tintCaches : Lit/unimi/dsi/fastutil/objects/Object2ObjectArrayMap;
private final chunkSource : Lnet/minecraft/client/multiplayer/ClientChunkCache;
private final lightUpdateQueue : Ljava/util/Deque;
private serverSimulationDistance : I
private final blockStatePredictionHandler : Lnet/minecraft/client/multiplayer/prediction/BlockStatePredictionHandler;
private final globallyRenderedBlockEntities : Ljava/util/Set;
private final explosionTracker : Lnet/minecraft/client/multiplayer/ClientExplosionTracker;
private final worldBorder : Lnet/minecraft/world/level/border/WorldBorder;
private final environmentAttributes : Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
private final destroyingBlocks : Lit/unimi/dsi/fastutil/ints/Int2ObjectMap;
private final destructionProgress : Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
private final seaLevel : I
private static final MARKER_PARTICLE_ITEMS : Ljava/util/Set;
public handleBlockChangedAck(I)V
public onBlockEntityAdded(Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public getGloballyRenderedBlockEntities()Ljava/util/Set;
public setServerVerifiedBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)V
public syncBlockState(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/phys/Vec3;)V
 getBlockStatePredictionHandler()Lnet/minecraft/client/multiplayer/prediction/BlockStatePredictionHandler;
public setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;II)Z
public <init>(Lnet/minecraft/client/multiplayer/ClientPacketListener;Lnet/minecraft/client/multiplayer/ClientLevel$ClientLevelData;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/Holder;IILnet/minecraft/client/renderer/extract/LevelExtractor;ZJI)V
private addEnvironmentAttributeLayers(Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;)Lnet/minecraft/world/attribute/EnvironmentAttributeSystem$Builder;
public queueLightUpdate(Ljava/lang/Runnable;)V
public pollLightUpdates()V
public endFlashState()Lnet/minecraft/client/renderer/EndFlashState;
public tick(Ljava/util/function/BooleanSupplier;)V
public tickWeatherEffects()V
public getPrecipitationAt(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/biome/Biome$Precipitation;
private removeBlockBreakingProgress()V
private removeProgress(Lnet/minecraft/server/level/BlockDestructionProgress;)V
public destructionProgress()Lit/unimi/dsi/fastutil/longs/Long2ObjectMap;
private tickTime()V
public setTimeFromServer(J)V
public entitiesForRendering()Ljava/lang/Iterable;
public tickEntities()V
public isTickingEntity(Lnet/minecraft/world/entity/Entity;)Z
public shouldTickDeath(Lnet/minecraft/world/entity/Entity;)Z
public tickNonPassenger(Lnet/minecraft/world/entity/Entity;)V
private tickPassenger(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)V
public update()V
public unload(Lnet/minecraft/world/level/chunk/LevelChunk;)V
public onChunkLoaded(Lnet/minecraft/world/level/ChunkPos;)V
public clearTintCaches()V
public hasChunk(II)Z
public getEntityCount()I
public addEntity(Lnet/minecraft/world/entity/Entity;)V
public removeEntity(ILnet/minecraft/world/entity/Entity$RemovalReason;)V
public getPushableEntities(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/AABB;)Ljava/util/List;
public getEntity(I)Lnet/minecraft/world/entity/Entity;
public disconnect(Lnet/minecraft/network/chat/Component;)V
public animateTick(III)V
private getMarkerParticleTarget()Lnet/minecraft/world/level/block/Block;
public doAnimateTick(IIIILnet/minecraft/util/RandomSource;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/core/BlockPos$MutableBlockPos;)V
private trySpawnDripParticles(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/particles/ParticleOptions;Z)V
private spawnParticle(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/world/phys/shapes/VoxelShape;D)V
private spawnFluidParticle(DDDDDLnet/minecraft/core/particles/ParticleOptions;)V
public fillReportDetails(Lnet/minecraft/CrashReport;)Lnet/minecraft/CrashReportCategory;
public playSeededSound(Lnet/minecraft/world/entity/Entity;DDDLnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public playSeededSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Holder;Lnet/minecraft/sounds/SoundSource;FFJ)V
public playLocalSound(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public playPlayerSound(Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FF)V
public playLocalSound(DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFZ)V
private playSound(DDDLnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundSource;FFZJ)V
public createFireworks(DDDDDDLjava/util/List;Z)V
public sendPacketToServer(Lnet/minecraft/network/protocol/Packet;)V
public getWorldBorder()Lnet/minecraft/world/level/border/WorldBorder;
public recipeAccess()Lnet/minecraft/world/item/crafting/RecipeAccess;
public tickRateManager()Lnet/minecraft/world/TickRateManager;
public getRelativeTickSpeed()F
public clockManager()Lnet/minecraft/client/ClientClockManager;
public environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeSystem;
public getBlockTicks()Lnet/minecraft/world/ticks/LevelTickAccess;
public getFluidTicks()Lnet/minecraft/world/ticks/LevelTickAccess;
public getChunkSource()Lnet/minecraft/client/multiplayer/ClientChunkCache;
public getMapData(Lnet/minecraft/world/level/saveddata/maps/MapId;)Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData;
public overrideMapData(Lnet/minecraft/world/level/saveddata/maps/MapId;Lnet/minecraft/world/level/saveddata/maps/MapItemSavedData;)V
public getScoreboard()Lnet/minecraft/world/scores/Scoreboard;
public sendBlockUpdated(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;I)V
public setBlocksDirty(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;)V
public setSectionDirtyWithNeighbors(III)V
public setSectionRangeDirty(IIIIII)V
public destroyBlockProgress(ILnet/minecraft/core/BlockPos;I)V
public globalLevelEvent(ILnet/minecraft/core/BlockPos;I)V
public levelEvent(Lnet/minecraft/world/entity/Entity;ILnet/minecraft/core/BlockPos;I)V
public addParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V
public addParticle(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDDDD)V
public addAlwaysVisibleParticle(Lnet/minecraft/core/particles/ParticleOptions;DDDDDD)V
public addAlwaysVisibleParticle(Lnet/minecraft/core/particles/ParticleOptions;ZDDDDDD)V
private doAddParticle(Lnet/minecraft/core/particles/ParticleOptions;ZZDDDDDD)V
private calculateParticleLevel(Z)Lnet/minecraft/server/level/ParticleStatus;
public players()Ljava/util/List;
public dragonParts()Ljava/util/List;
public getUncachedNoiseBiome(III)Lnet/minecraft/core/Holder;
private getSkyFlashTime()I
public setSkyFlashTime(I)V
public cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;
public getBlockTint(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
public calculateBlockTint(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
public setRespawnData(Lnet/minecraft/world/level/storage/LevelData$RespawnData;)V
public getRespawnData()Lnet/minecraft/world/level/storage/LevelData$RespawnData;
public toString()Ljava/lang/String;
public getLevelData()Lnet/minecraft/client/multiplayer/ClientLevel$ClientLevelData;
public gameEvent(Lnet/minecraft/core/Holder;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/level/gameevent/GameEvent$Context;)V
protected getAllMapData()Ljava/util/Map;
protected addMapData(Ljava/util/Map;)V
protected getEntities()Lnet/minecraft/world/level/entity/LevelEntityGetter;
public gatherChunkSourceStats()Ljava/lang/String;
public addDestroyBlockEffect(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public addBreakingBlockEffects(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Z)V
private addBreakingParticles(Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/state/BlockState;)V
private playBreakingSound(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public setServerSimulationDistance(I)V
public getServerSimulationDistance()I
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public explode(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/damagesource/DamageSource;Lnet/minecraft/world/level/ExplosionDamageCalculator;DDDFZLnet/minecraft/world/level/Level$ExplosionInteraction;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/util/random/WeightedList;Lnet/minecraft/core/Holder;)V
public getSeaLevel()I
public getClientLeafTintColor(Lnet/minecraft/core/BlockPos;)I
public registerForCleaning(Lnet/minecraft/client/multiplayer/CacheSlot;)V
public trackExplosionEffects(Lnet/minecraft/world/phys/Vec3;FILnet/minecraft/util/random/WeightedList;)V
public synthetic clockManager()Lnet/minecraft/world/clock/ClockManager;
public synthetic getLevelData()Lnet/minecraft/world/level/storage/LevelData;
public synthetic dragonParts()Ljava/util/Collection;
public synthetic getChunkSource()Lnet/minecraft/world/level/chunk/ChunkSource;
public synthetic environmentAttributes()Lnet/minecraft/world/attribute/EnvironmentAttributeReader;
private synthetic lambda$addDestroyBlockEffect$0(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;DDDDDD)V
private synthetic lambda$doAddParticle$1(DDD)Ljava/lang/String;
private synthetic lambda$doAddParticle$0(Lnet/minecraft/core/particles/ParticleOptions;)Ljava/lang/String;
private static synthetic lambda$destroyBlockProgress$0(J)Ljava/util/SortedSet;
private synthetic lambda$fillReportDetails$3()Ljava/lang/String;
private synthetic lambda$fillReportDetails$2()Ljava/lang/String;
private synthetic lambda$fillReportDetails$1()Ljava/lang/String;
private synthetic lambda$fillReportDetails$0()Ljava/lang/String;
private static synthetic lambda$clearTintCaches$0(Lnet/minecraft/world/level/ColorResolver;Lnet/minecraft/client/color/block/BlockTintCache;)V
private static synthetic lambda$onChunkLoaded$0(Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/world/level/ColorResolver;Lnet/minecraft/client/color/block/BlockTintCache;)V
private synthetic lambda$tickEntities$0(Lnet/minecraft/world/entity/Entity;)V
private synthetic lambda$addEnvironmentAttributeLayers$1(Ljava/lang/Float;I)Ljava/lang/Float;
private synthetic lambda$addEnvironmentAttributeLayers$0(Lorg/joml/Vector3fc;Lorg/joml/Vector3fc;I)Lorg/joml/Vector3fc;
private synthetic lambda$new$0(Lit/unimi/dsi/fastutil/objects/Object2ObjectArrayMap;)V
private synthetic lambda$new$4(Lnet/minecraft/core/BlockPos;)I
private synthetic lambda$new$3(Lnet/minecraft/core/BlockPos;)I
private synthetic lambda$new$2(Lnet/minecraft/core/BlockPos;)I
private synthetic lambda$new$1(Lnet/minecraft/core/BlockPos;)I
static <clinit>()V
```
