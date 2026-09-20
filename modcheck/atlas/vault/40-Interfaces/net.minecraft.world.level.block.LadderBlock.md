---
type: "interface"
fqcn: "net.minecraft.world.level.block.LadderBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.LadderBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/Block`; implements `net/minecraft/world/level/block/SimpleWaterloggedBlock`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `FACING` | `Lnet/minecraft/world/level/block/state/properties/EnumProperty;` | exact | getstatic@24 in `LivingEntityMixin.allowTaggedBlocksForTrapdoorClimbing` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (3 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FACING : Lnet/minecraft/world/level/block/state/properties/EnumProperty;
public static final WATERLOGGED : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final SHAPES : Ljava/util/Map;
public <init>(Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
protected getShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/CollisionContext;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private canAttachTo(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
protected canSurvive(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
protected updateShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/world/level/ScheduledTickAccess;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/util/RandomSource;)Lnet/minecraft/world/level/block/state/BlockState;
public getStateForPlacement(Lnet/minecraft/world/item/context/BlockPlaceContext;)Lnet/minecraft/world/level/block/state/BlockState;
protected rotate(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Rotation;)Lnet/minecraft/world/level/block/state/BlockState;
protected mirror(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/block/Mirror;)Lnet/minecraft/world/level/block/state/BlockState;
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
protected getFluidState(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/FluidState;
static <clinit>()V
```
