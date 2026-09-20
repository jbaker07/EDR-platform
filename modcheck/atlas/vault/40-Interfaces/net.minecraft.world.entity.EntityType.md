---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityType

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/level/entity/EntityTypeTest`, `net/minecraft/world/flag/FeatureElement`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `create` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySp` | exact | invokevirtual@36 in `EntityApiLookupImpl.lambda$checkSelfImplementingTypes$0` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `getCategory` | `()Lnet/minecraft/world/entity/MobCategory;` | exact | invokevirtual@1 in `BiomeModifications.addSpawn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getDefaultLootTable` | `()Ljava/util/Optional;` | exact | invokevirtual@203 in `FabricEntityLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@2 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `updateInterval` | `()I` | exact | invokevirtual@48 in `TestServerConnectionImpl.waitForClientboundEntityUpdates` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `updateInterval` | `()I` | exact | invokevirtual@83 in `TestServerConnectionImpl.waitForClientboundEntityUpdates` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySp` | exact | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `onlyOpCanSetNbt` | `()Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| injects_into | `trackDeltas` | `()Z` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (22 fields, 65 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final builtInRegistryHolder : Lnet/minecraft/core/Holder$Reference;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final NO_UPDATE_INTERVAL : I
private final factory : Lnet/minecraft/world/entity/EntityType$EntityFactory;
private final category : Lnet/minecraft/world/entity/MobCategory;
private final immuneTo : Lnet/minecraft/tags/TagKey;
private final serialize : Z
private final summon : Z
private final fireImmune : Z
private final canSpawnFarFromPlayer : Z
private final clientTrackingRange : I
private final updateInterval : I
private final descriptionId : Ljava/lang/String;
private description : Lnet/minecraft/network/chat/Component;
private final lootTable : Ljava/util/Optional;
private final dimensions : Lnet/minecraft/world/entity/EntityDimensions;
private final spawnDimensionsScale : F
private final requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final allowedInPeaceful : Z
private final trackDeltas : Z
public static getKey(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/resources/Identifier;
public <init>(Lnet/minecraft/world/entity/EntityType$EntityFactory;Lnet/minecraft/world/entity/MobCategory;ZZZZLnet/minecraft/tags/TagKey;Lnet/minecraft/world/entity/EntityDimensions;FIILjava/lang/String;Ljava/util/Optional;Lnet/minecraft/world/flag/FeatureFlagSet;ZZ)V
public spawn(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntitySpawnReason;ZZ)Lnet/minecraft/world/entity/Entity;
public static createDefaultStackConfig(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/entity/PostSpawnProcessor;
public static appendDefaultStackConfig(Lnet/minecraft/world/entity/PostSpawnProcessor;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/entity/PostSpawnProcessor;
public static appendComponentsConfig(Lnet/minecraft/world/entity/PostSpawnProcessor;Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/PostSpawnProcessor;
public static appendCustomEntityStackConfig(Lnet/minecraft/world/entity/PostSpawnProcessor;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/world/entity/PostSpawnProcessor;
public spawn(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntitySpawnReason;)Lnet/minecraft/world/entity/Entity;
public spawn(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/PostSpawnProcessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntitySpawnReason;ZZ)Lnet/minecraft/world/entity/Entity;
public create(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/PostSpawnProcessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntitySpawnReason;ZZ)Lnet/minecraft/world/entity/Entity;
protected static getYOffset(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;ZLnet/minecraft/world/phys/AABB;)D
public static updateCustomEntityTag(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/component/TypedEntityData;)V
public canSerialize()Z
public canSummon()Z
public fireImmune()Z
public canSpawnFarFromPlayer()Z
public getCategory()Lnet/minecraft/world/entity/MobCategory;
public getDescriptionId()Ljava/lang/String;
public getDescription()Lnet/minecraft/network/chat/Component;
public toString()Ljava/lang/String;
public toShortString()Ljava/lang/String;
public getDefaultLootTable()Ljava/util/Optional;
public getWidth()F
public getHeight()F
public requiredFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public canSpawn(Lnet/minecraft/world/level/Level;)Z
public create(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;)Lnet/minecraft/world/entity/Entity;
public create(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;)Lnet/minecraft/world/entity/Entity;
public static create(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;)Ljava/util/Optional;
public static create(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;)Ljava/util/Optional;
public getSpawnAABB(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/phys/AABB;
public getSpawnAABB(DDD)Lnet/minecraft/world/phys/AABB;
public isBlockDangerous(Lnet/minecraft/world/level/block/state/BlockState;)Z
public getDimensions()Lnet/minecraft/world/entity/EntityDimensions;
public static by(Lnet/minecraft/world/level/storage/ValueInput;)Ljava/util/Optional;
public static loadEntityRecursive(Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
public static loadEntityRecursive(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/nbt/CompoundTag;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
public static loadEntityRecursive(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
public static loadEntityRecursive(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
public static loadEntityRecursive(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
private static loadPassengersRecursive(Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;Lnet/minecraft/world/entity/EntityProcessor;)Lnet/minecraft/world/entity/Entity;
public static loadEntitiesRecursive(Lnet/minecraft/world/level/storage/ValueInput$ValueInputList;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;)Ljava/util/stream/Stream;
private static loadStaticEntity(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;)Ljava/util/Optional;
private static loadStaticEntity(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;)Ljava/util/Optional;
public clientTrackingRange()I
public updateInterval()I
public hasUpdateInterval()Z
public trackDeltas()Z
public tryCast(Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
public getBaseClass()Ljava/lang/Class;
public builtInRegistryHolder()Lnet/minecraft/core/Holder$Reference;
public isAllowedInPeaceful()Z
public onlyOpCanSetNbt()Z
public synthetic tryCast(Ljava/lang/Object;)Ljava/lang/Object;
private static synthetic lambda$loadEntitiesRecursive$0(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/level/storage/ValueInput;Ljava/util/function/Consumer;)V
private static synthetic lambda$loadEntitiesRecursive$1(Ljava/util/function/Consumer;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$loadEntityRecursive$1(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/EntityProcessor;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$loadEntityRecursive$0(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;Lnet/minecraft/world/entity/EntityProcessor;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$create$3(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$create$2(Lnet/minecraft/world/level/storage/ValueInput;)V
private static synthetic lambda$create$1(Lnet/minecraft/world/level/storage/ValueInput;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$create$0(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/EntitySpawnRequest;Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$appendCustomEntityStackConfig$0(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/component/TypedEntityData;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$appendComponentsConfig$0(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/entity/Entity;)V
static <clinit>()V
```
