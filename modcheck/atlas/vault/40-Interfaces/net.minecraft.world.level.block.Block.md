---
type: "interface"
fqcn: "net.minecraft.world.level.block.Block"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.Block

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/state/BlockBehaviour`; implements `net/minecraft/world/level/ItemLike`, `net/fabricmc/fabric/api/block/v1/FabricBlock`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V` | exact | invokespecial@2 in `LiquidBlockMixin.<init>` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@60 in `BlockApiLookupImpl.registerSelf` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@28 in `FlammableBlockRegistryImpl.get` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@45 in `LivingEntityMixin.modifyBedForOccupiedState` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@72 in `BlockColorRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@53 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@72 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@9 in `CauldronStorage.updateLevel` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultBlockState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@92 in `CauldronStorage.extract` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getAppearance` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | inherited_exact | invokevirtual@20 in `FabricBlockState.getAppearance` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getDescriptionId` | `()Ljava/lang/String;` | inherited_exact | invokevirtual@2 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getLootTable` | `()Ljava/util/Optional;` | inherited_exact | invokevirtual@161 in `FabricBlockLootSubProvider.generate` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getName` | `()Lnet/minecraft/network/chat/MutableComponent;` | exact | invokevirtual@56 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getProvidedEnchantmentPower` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | inherited_exact | invokevirtual@12 in `FabricBlockState.getProvidedEnchantmentPower` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@1 in `OxidizableBlocksRegistryImpl.refreshRandomTickCache` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@20 in `ModelLoadingEventDispatcher.resolveBlockStates` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@34 in `PoiHelper.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@1 in `BlockInitTracker.lambda$postFreeze$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@1 in `BlocksMixin.lambda$initShapeCache$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@1 in `BootstrapMixin.lambda$afterInitialize$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `shouldRenderFace` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/worl` | exact | invokestatic@60 in `AltModelBlockRendererImpl.shouldCullFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `BLOCK_STATE_REGISTRY` | `Lnet/minecraft/core/IdMapper;` | exact | getstatic@15 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (26 fields, 85 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final builtInRegistryHolder : Lnet/minecraft/core/Holder$Reference;
public static final BLOCK_STATE_REGISTRY : Lnet/minecraft/core/IdMapper;
private static final SHAPE_FULL_BLOCK_CACHE : Lcom/google/common/cache/LoadingCache;
public static final UPDATE_NEIGHBORS : I
public static final UPDATE_CLIENTS : I
public static final UPDATE_INVISIBLE : I
public static final UPDATE_IMMEDIATE : I
public static final UPDATE_KNOWN_SHAPE : I
public static final UPDATE_SUPPRESS_DROPS : I
public static final UPDATE_MOVE_BY_PISTON : I
public static final UPDATE_SKIP_SHAPE_UPDATE_ON_WIRE : I
public static final UPDATE_SKIP_BLOCK_ENTITY_SIDEEFFECTS : I
public static final UPDATE_SKIP_ON_PLACE : I
public static final UPDATE_NONE : I
public static final UPDATE_ALL : I
public static final UPDATE_ALL_IMMEDIATE : I
public static final UPDATE_SKIP_ALL_SIDEEFFECTS : I
public static final INDESTRUCTIBLE : F
public static final INSTANT : F
public static final UPDATE_LIMIT : I
protected final stateDefinition : Lnet/minecraft/world/level/block/state/StateDefinition;
private defaultBlockState : Lnet/minecraft/world/level/block/state/BlockState;
private item : Lnet/minecraft/world/item/Item;
private static final CACHE_SIZE : I
private static final OCCLUSION_CACHE : Ljava/lang/ThreadLocal;
public static getId(Lnet/minecraft/world/level/block/state/BlockState;)I
public static stateById(I)Lnet/minecraft/world/level/block/state/BlockState;
public static byItem(Lnet/minecraft/world/item/Item;)Lnet/minecraft/world/level/block/Block;
public static pushEntitiesUp(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public static box(DDDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static boxes(ILjava/util/function/IntFunction;)[Lnet/minecraft/world/phys/shapes/VoxelShape;
public static cube(D)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static cube(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static column(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static column(DDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static boxZ(DDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static boxZ(DDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static boxZ(DDDDD)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static updateFromNeighbourShapes(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public static updateOrDestroy(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;I)V
public static updateOrDestroy(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;II)V
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
public static isExceptionForConnection(Lnet/minecraft/world/level/block/state/BlockState;)Z
public static dropFromBlockInteractLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/item/ItemInstance;Lnet/minecraft/world/entity/Entity;Ljava/util/function/BiConsumer;)Z
protected static dropFromLootTable(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/resources/ResourceKey;Ljava/util/function/Function;Ljava/util/function/BiConsumer;)Z
public static shouldRenderFace(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;)Z
public static canSupportRigidBlock(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public static canSupportCenter(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
public static isFaceFull(Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/core/Direction;)Z
public static isShapeFullBlock(Lnet/minecraft/world/phys/shapes/VoxelShape;)Z
public animateTick(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public destroy(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public static getDrops(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;)Ljava/util/List;
public static getDrops(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemInstance;)Ljava/util/List;
public static dropResources(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public static dropResources(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;)V
public static dropResources(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemStack;)V
public static popResource(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
public static popResourceFromFace(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/world/item/ItemStack;)V
private static popResource(Lnet/minecraft/world/level/Level;Ljava/util/function/Supplier;Lnet/minecraft/world/item/ItemStack;)V
protected popExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;I)V
public getExplosionResistance()F
public wasExploded(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Explosion;)V
public stepOn(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/Entity;)V
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
public playerDestroy(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/item/ItemStack;)V
public setPlacedBy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;)V
public isPossibleToRespawnInThis(Lnet/minecraft/world/level/block/state/BlockState;)Z
public getName()Lnet/minecraft/network/chat/MutableComponent;
public fallOn(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;D)V
public bounceOn(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;D)V
public getBounceRestitution()F
public getFallDistanceReduction()F
public getFriction()F
public getSpeedFactor()F
public getJumpFactor()F
public spawnDestroyByEntityParticles(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public spawnDestroyParticles(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public playerWillDestroy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/level/block/state/BlockState;
public handlePrecipitation(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/biome/Biome$Precipitation;)V
public dropFromExplosion(Lnet/minecraft/world/level/Explosion;)Z
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
public getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;
protected final registerDefaultState(Lnet/minecraft/world/level/block/state/BlockState;)V
public final defaultBlockState()Lnet/minecraft/world/level/block/state/BlockState;
public final withPropertiesOf(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
public asItem()Lnet/minecraft/world/item/Item;
public hasDynamicShape()Z
public toString()Ljava/lang/String;
protected asBlock()Lnet/minecraft/world/level/block/Block;
protected getShapeForEachState(Ljava/util/function/Function;)Ljava/util/function/Function;
protected getShapeForEachState(Ljava/util/function/Function;[Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/util/function/Function;
private static setValueHelper(Lnet/minecraft/world/level/block/state/StateHolder;Lnet/minecraft/world/level/block/state/properties/Property;Ljava/lang/Object;)Lnet/minecraft/world/level/block/state/StateHolder;
public builtInRegistryHolder()Lnet/minecraft/core/Holder$Reference;
protected tryDropExperience(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/util/valueproviders/IntProvider;)V
private static synthetic lambda$getShapeForEachState$4(Ljava/util/Map;Lcom/google/common/collect/ImmutableMap;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$getShapeForEachState$2(Ljava/util/Map;Lnet/minecraft/world/level/block/state/BlockState;)Z
private static synthetic lambda$getShapeForEachState$3(Lnet/minecraft/world/level/block/state/BlockState;Ljava/util/Map$Entry;)Z
private static synthetic lambda$getShapeForEachState$1(Lnet/minecraft/world/level/block/state/properties/Property;)Ljava/lang/Object;
private static synthetic lambda$getShapeForEachState$0(Lnet/minecraft/world/level/block/state/properties/Property;)Lnet/minecraft/world/level/block/state/properties/Property;
private static synthetic lambda$popResourceFromFace$0(Lnet/minecraft/world/level/Level;DDDLnet/minecraft/world/item/ItemStack;DDD)Lnet/minecraft/world/entity/item/ItemEntity;
private static synthetic lambda$popResource$0(Lnet/minecraft/world/level/Level;DDDLnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/entity/item/ItemEntity;
private static synthetic lambda$dropResources$2(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$dropResources$1(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$dropResources$0(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$static$0()Lit/unimi/dsi/fastutil/objects/Object2ByteLinkedOpenHashMap;
private static synthetic lambda$dropFromLootTable$0(Ljava/util/function/BiConsumer;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)V
private static synthetic lambda$dropFromBlockInteractLootTable$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/item/ItemInstance;Lnet/minecraft/world/level/storage/loot/LootParams$Builder;)Lnet/minecraft/world/level/storage/loot/LootParams;
private static synthetic lambda$boxes$0(I)[Lnet/minecraft/world/phys/shapes/VoxelShape;
static <clinit>()V
```
