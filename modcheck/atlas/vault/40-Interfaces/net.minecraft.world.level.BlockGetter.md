---
type: "interface"
fqcn: "net.minecraft.world.level.BlockGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.BlockGetter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/LevelHeightAccessor`, `net/fabricmc/fabric/api/blockgetter/v2/FabricBlockGetter`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getBlockEntity` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity` | exact | invokeinterface@5 in `FabricBlockGetter.getBlockEntityRenderData` | unknown | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | direct_reference |

## Declared members (0 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getBlockEntity(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/entity/BlockEntityType;)Ljava/util/Optional;
public abstract getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public abstract getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public getLightEmission(Lnet/minecraft/core/BlockPos;)I
public getBlockStates(Lnet/minecraft/world/phys/AABB;)Ljava/util/stream/Stream;
public isBlockInLine(Lnet/minecraft/world/level/ClipBlockStateContext;)Lnet/minecraft/world/phys/BlockHitResult;
public clip(Lnet/minecraft/world/level/ClipContext;)Lnet/minecraft/world/phys/BlockHitResult;
public clipWithInteractionOverride(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/shapes/VoxelShape;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/phys/BlockHitResult;
public getBlockFloorHeight(Lnet/minecraft/world/phys/shapes/VoxelShape;Ljava/util/function/Supplier;)D
public getBlockFloorHeight(Lnet/minecraft/core/BlockPos;)D
public static traverseBlocks(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Ljava/lang/Object;Ljava/util/function/BiFunction;Ljava/util/function/Function;)Ljava/lang/Object;
public static forEachBlockIntersectedBetween(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/level/BlockGetter$BlockStepVisitor;)Z
private static addCollisionsAlongTravel(Lit/unimi/dsi/fastutil/longs/LongSet;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/phys/AABB;Lnet/minecraft/world/level/BlockGetter$BlockStepVisitor;)I
private static getFurthestCorner(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/core/Vec3i;
private synthetic lambda$getBlockFloorHeight$0(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$clip$1(Lnet/minecraft/world/level/ClipContext;)Lnet/minecraft/world/phys/BlockHitResult;
private synthetic lambda$clip$0(Lnet/minecraft/world/level/ClipContext;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/BlockHitResult;
private static synthetic lambda$isBlockInLine$1(Lnet/minecraft/world/level/ClipBlockStateContext;)Lnet/minecraft/world/phys/BlockHitResult;
private synthetic lambda$isBlockInLine$0(Lnet/minecraft/world/level/ClipBlockStateContext;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/BlockHitResult;
```
