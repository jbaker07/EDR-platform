---
type: "interface"
fqcn: "net.minecraft.world.level.block.AbstractBedBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.AbstractBedBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/block/HorizontalDirectionalBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `findStandUpPosition` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Col` | exact | invokestatic@32 in `LivingEntityMixin.modifyWakeUpPosition` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (2 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final PART : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final OCCUPIED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
public static getBedOrientation(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/Direction;
protected abstract getBedEnvironmentAttribute()Lnet/minecraft/world/attribute/EnvironmentAttribute;
protected abstract destroyOnUse(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/InteractionResult;
protected abstract destroyOnLeave(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public getBedRule(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/attribute/BedRule;
public getSleptInBedStatType()Lnet/minecraft/resources/Identifier;
public getSleepHeight(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Ljava/util/OptionalDouble;
protected useWithoutItem(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
public onStopSleeping(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
private kickVillagerOutOfBed(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Z
protected updateShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
private static getNeighbourDirection(Lnet/minecraft/world/level/block/state/properties/BedPart;Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Direction;
public playerWillDestroy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/world/level/block/state/BlockState;
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
public static getConnectedDirection(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/Direction;
public static getBlockType(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/DoubleBlockCombiner$BlockType;
private static isBunkBed(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public static findStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;F)Ljava/util/Optional;
private static findBunkBedStandUpPosition(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;)Ljava/util/Optional;
private static findStandUpPositionAtOffset(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/CollisionGetter;Lnet/minecraft/core/BlockPos;[[IZ)Ljava/util/Optional;
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
public setPlacedBy(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/item/ItemStack;)V
protected getSeed(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)J
protected isPathfindable(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/pathfinder/PathComputationType;)Z
private static bedStandUpOffsets(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;)[[I
private static bedSurroundStandUpOffsets(Lnet/minecraft/core/Direction;Lnet/minecraft/core/Direction;)[[I
private static bedAboveStandUpOffsets(Lnet/minecraft/core/Direction;)[[I
private static synthetic lambda$useWithoutItem$0(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/entity/player/Player$BedSleepingProblem;)V
static <clinit>()V
```
