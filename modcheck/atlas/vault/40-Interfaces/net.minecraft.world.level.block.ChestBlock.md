---
type: "interface"
fqcn: "net.minecraft.world.level.block.ChestBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.ChestBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/AbstractChestBlock`; implements `net/minecraft/world/level/block/SimpleWaterloggedBlock`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getConnectedDirection` | `(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/cor` | exact | invokestatic@60 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getContainer` | `(Lnet/minecraft/world/level/block/ChestBlock;Lnet/minecraft/world/leve` | exact | invokestatic@115 in `ItemStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `TYPE` | `Lnet/minecraft/world/level/block/state/properties/EnumProperty;` | exact | getstatic@40 in `ContainerSlotWrapper.updateSnapshots` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (10 fields, 35 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FACING : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final TYPE : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final WATERLOGGED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final EVENT_SET_OPEN_COUNT : I
private static final SHAPE : Lnet/minecraft/world/phys/shapes/VoxelShape;
private static final HALF_SHAPES : Ljava/util/Map;
private final openSound : Lnet/minecraft/sounds/SoundEvent;
private final closeSound : Lnet/minecraft/sounds/SoundEvent;
private static final CHEST_COMBINER : Lnet/minecraft/world/level/block/DoubleBlockCombiner$Combiner;
private static final MENU_PROVIDER_COMBINER : Lnet/minecraft/world/level/block/DoubleBlockCombiner$Combiner;
public <init>(Ljava/util/function/Supplier;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/sounds/SoundEvent;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
public static getBlockType(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/DoubleBlockCombiner$BlockType;
protected updateShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
public chestCanConnectTo(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected getShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public static getConnectedDirection(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/Direction;
public static getConnectedBlockPos(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/BlockPos;
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
protected getChestType(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Lnet/minecraft/world/level/block/state/properties/ChestType;
protected getFluidState(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/FluidState;
private candidatePartnerFacing(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Lnet/minecraft/core/Direction;
protected affectNeighborsAfterRemoval(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Z)V
protected useWithoutItem(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
protected getOpenChestStat()Lnet/minecraft/stats/Stat;
public blockEntityType()Lnet/minecraft/world/level/block/entity/BlockEntityType;
public static getContainer(Lnet/minecraft/world/level/block/ChestBlock;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Z)Lnet/minecraft/world/Container;
public combine(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Z)Lnet/minecraft/world/level/block/DoubleBlockCombiner$NeighborCombineResult;
protected getMenuProvider(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/MenuProvider;
public static opennessCombiner(Lnet/minecraft/world/level/block/entity/LidBlockEntity;)Lnet/minecraft/world/level/block/DoubleBlockCombiner$Combiner;
public newBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getTicker(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/entity/BlockEntityType;)Lnet/minecraft/world/level/block/entity/BlockEntityTicker;
public static isChestBlockedAt(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Z
private static isBlockedChestByBlock(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
private static isCatSittingOnChest(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Z
protected hasAnalogOutputSignal(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected getAnalogOutputSignal(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
protected rotate(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/world/level/block/state/BlockState;
protected mirror(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Mirror;)Lnet/minecraft/world/level/block/state/BlockState;
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
protected isPathfindable(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/pathfinder/PathComputationType;)Z
protected tick(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public getOpenChestSound()Lnet/minecraft/sounds/SoundEvent;
public getCloseChestSound()Lnet/minecraft/sounds/SoundEvent;
private static synthetic lambda$combine$0(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)Z
static <clinit>()V
```
