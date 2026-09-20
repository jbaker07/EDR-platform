---
type: "interface"
fqcn: "net.minecraft.world.level.block.TrapDoorBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.TrapDoorBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/HorizontalDirectionalBlock`; implements `net/minecraft/world/level/block/SimpleWaterloggedBlock`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `FACING` | `Lnet/minecraft/world/level/block/state/properties/EnumProperty;` | inherited_exact | getstatic@31 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (6 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final OPEN : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final HALF : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final POWERED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final WATERLOGGED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
private static final SHAPES : Ljava/util/Map;
private final type : Lnet/minecraft/world/level/block/state/properties/BlockSetType;
public <init>(Lnet/minecraft/world/level/block/state/properties/BlockSetType;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
protected getShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
protected isPathfindable(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/pathfinder/PathComputationType;)Z
protected useWithoutItem(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/phys/BlockHitResult;)Lnet/minecraft/world/InteractionResult;
protected onExplosionHit(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/Explosion;Ljava/util/function/BiConsumer;)V
private toggle(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/player/Player;)V
protected playSound(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Z)V
protected neighborChanged(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Block;Lnet/minecraft/world/level/redstone/Orientation;Z)V
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
protected getFluidState(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/FluidState;
protected updateShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
protected getType()Lnet/minecraft/world/level/block/state/properties/BlockSetType;
static <clinit>()V
```
