---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.BlockBehaviour$BlockStateBase

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/block/state/StateHolder`; implements `net/minecraft/core/TypedInstance`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `asState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |
| calls | `asState` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | declared |
| injects_into | `useItemOn` | `(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useWithoutItem` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/P` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| reads | `isRandomlyTicking` | `Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |

## Declared members (31 fields, 100 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DIRECTIONS : [Lnet/minecraft/core/Direction;
private static final EMPTY_OCCLUSION_SHAPES : [Lnet/minecraft/world/phys/shapes/VoxelShape;
private static final FULL_BLOCK_OCCLUSION_SHAPES : [Lnet/minecraft/world/phys/shapes/VoxelShape;
private final lightEmission : I
private final useShapeForLightOcclusion : Z
private final isAir : Z
private final ignitedByLava : Z
private final liquid : Z
private legacySolid : Z
private final pushReaction : Lnet/minecraft/world/level/material/PushReaction;
private final mapColor : Lnet/minecraft/world/level/material/MapColor;
private final destroySpeed : F
private final requiresCorrectToolForDrops : Z
private final canOcclude : Z
private final isRedstoneConductor : Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;
private final isSuffocating : Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;
private final isViewBlocking : Lnet/minecraft/world/level/block/state/BlockBehaviour$StateArgumentPredicate;
private final postProcess : Lnet/minecraft/world/level/block/state/BlockBehaviour$PostProcess;
private final emissiveRendering : Ljava/util/function/Predicate;
private final offsetFunction : Lnet/minecraft/world/level/block/state/BlockBehaviour$OffsetFunction;
private final spawnTerrainParticles : Z
private final instrument : Lnet/minecraft/world/level/block/state/properties/NoteBlockInstrument;
private final replaceable : Z
private cache : Lnet/minecraft/world/level/block/state/BlockBehaviour$BlockStateBase$Cache;
private fluidState : Lnet/minecraft/world/level/material/FluidState;
private isRandomlyTicking : Z
private solidRender : Z
private occlusionShape : Lnet/minecraft/world/phys/shapes/VoxelShape;
private occlusionShapesByFace : [Lnet/minecraft/world/phys/shapes/VoxelShape;
private propagatesSkylightDown : Z
private lightDampening : I
protected <init>(Lnet/minecraft/world/level/block/Block;[Lnet/minecraft/world/level/block/state/properties/Property;[Ljava/lang/Comparable;)V
private calculateSolid()Z
public initCache()V
public getBlock()Lnet/minecraft/world/level/block/Block;
public typeHolder()Lnet/minecraft/core/Holder;
public isSolid()Z
public isValidSpawn(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntityType;)Z
public propagatesSkylightDown()Z
public getLightDampening()I
public getFaceOcclusionShape(Lnet/minecraft/core/Direction;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getOcclusionShape()Lnet/minecraft/world/phys/shapes/VoxelShape;
public hasLargeCollisionShape()Z
public useShapeForLightOcclusion()Z
public getLightEmission()I
public isAir()Z
public ignitedByLava()Z
public liquid()Z
public getMapColor(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/MapColor;
public rotate(Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/world/level/block/state/BlockState;
public mirror(Lnet/minecraft/world/level/block/Mirror;)Lnet/minecraft/world/level/block/state/BlockState;
public getRenderShape()Lnet/minecraft/world/level/block/RenderShape;
public emissiveRendering()Z
public getShadeBrightness(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public isRedstoneConductor(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public isSignalSource()Z
public shouldRedstoneWireConnectTo(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
public getOwnSignal(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)I
public getSignal(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
public hasAnalogOutputSignal()Z
public getAnalogOutputSignal(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
public getDestroySpeed(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public getDestroyProgress(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public getDirectSignal(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
public getPistonPushReaction()Lnet/minecraft/world/level/material/PushReaction;
public isSolidRender()Z
public canOcclude()Z
public skipRendering(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;)Z
public getShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getCollisionShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getCollisionShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getEntityInsideCollisionShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getBlockSupportShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getVisualShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getInteractionShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public final entityCanStandOn(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;)Z
public final entityCanStandOnFace(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/core/Direction;)Z
public getOffset(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
public hasOffsetFunction()Z
public triggerEvent(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;II)Z
public handleNeighborChanged(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;Z)V
public final updateNeighbourShapes(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;I)V
public final updateNeighbourShapes(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;II)V
public final updateIndirectNeighbourShapes(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;I)V
public updateIndirectNeighbourShapes(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;II)V
public onPlace(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Z)V
public affectNeighborsAfterRemoval(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Z)V
public onExplosionHit(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Explosion;Ljava/util/function/BiConsumer;)V
public tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public randomTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public entityInside(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;Z)V
public spawnAfterBreak(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/item/ItemStack;Z)V
public getDrops(Lnet/minecraft/world/level/storage/loot/LootParams$Builder;)Ljava/util/List;
public useItemOn(Lnet/minecraft/world/item/ItemStack;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
public useWithoutItem(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
public showAsInteractableInSpectatorMode(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/BlockHitResult;)Z
public attack(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)V
public isSuffocating(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public isLightPermeable()Z
public isViewBlocking(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/AABB;)Z
public updateShape(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
public isPathfindable(Lnet/minecraft/world/level/pathfinder/PathComputationType;)Z
public canBeReplaced(Lnet/minecraft/world/item/context/BlockPlaceContext;)Z
public canBeReplaced(Lnet/minecraft/world/level/material/Fluid;)Z
public canBeReplaced()Z
public canSurvive(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
public getPostProcessPos(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
public getMenuProvider(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/MenuProvider;
public is(Lnet/minecraft/tags/TagKey;Ljava/util/function/Predicate;)Z
public hasBlockEntity()Z
public shouldChangedStateKeepBlockEntity(Lnet/minecraft/world/level/block/state/BlockState;)Z
public getTicker(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/entity/BlockEntityType;)Lnet/minecraft/world/level/block/entity/BlockEntityTicker;
public getFluidState()Lnet/minecraft/world/level/material/FluidState;
public isRandomlyTicking()Z
public getSeed(Lnet/minecraft/core/BlockPos;)J
public getSoundType()Lnet/minecraft/world/level/block/SoundType;
public onProjectileHit(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/phys/BlockHitResult;Lnet/minecraft/world/entity/projectile/Projectile;)V
public isFaceSturdy(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
public isFaceSturdy(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/SupportType;)Z
public isCollisionShapeFullBlock(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public getCloneItemStack(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;Z)Lnet/minecraft/world/item/ItemStack;
protected abstract asState()Lnet/minecraft/world/level/block/state/BlockState;
public requiresCorrectToolForDrops()Z
public shouldSpawnTerrainParticles()Z
public withPropertiesOf(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/state/BlockState;
public static copyProperty(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/state/properties/Property;)Lnet/minecraft/world/level/block/state/BlockState;
public instrument()Lnet/minecraft/world/level/block/state/properties/NoteBlockInstrument;
private static synthetic lambda$static$1([Lnet/minecraft/world/phys/shapes/VoxelShape;)V
private static synthetic lambda$static$0([Lnet/minecraft/world/phys/shapes/VoxelShape;)V
static <clinit>()V
```
