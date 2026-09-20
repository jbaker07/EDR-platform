---
type: "interface"
fqcn: "net.minecraft.world.level.material.FlowingFluid"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.FlowingFluid

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/material/Fluid`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getSource` | `()Lnet/minecraft/world/level/material/Fluid;` | exact | invokevirtual@45 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `spreadTo` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (5 fields, 39 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FALLING : Lnet/minecraft/world/level/block/state/properties/BooleanProperty;
public static final LEVEL : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
private static final CACHE_SIZE : I
private static final OCCLUSION_CACHE : Ljava/lang/ThreadLocal;
private final shapes : Ljava/util/Map;
public <init>()V
protected createFluidStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
public getFlow(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/world/phys/Vec3;
private affectsFlow(Lnet/minecraft/world/level/material/FluidState;)Z
protected isSolidFace(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)Z
protected spread(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V
private spreadToSides(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/block/state/BlockState;)V
protected getNewLiquid(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/FluidState;
private static canPassThroughWall(Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z
public abstract getFlowing()Lnet/minecraft/world/level/material/Fluid;
public getFlowing(IZ)Lnet/minecraft/world/level/material/FluidState;
public abstract getSource()Lnet/minecraft/world/level/material/Fluid;
public getSource(Z)Lnet/minecraft/world/level/material/FluidState;
protected abstract canConvertToSource(Lnet/minecraft/server/level/ServerLevel;)Z
protected spreadTo(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/material/FluidState;)V
protected abstract beforeDestroyingBlock(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
protected getSlopeDistance(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;ILnet/minecraft/core/Direction;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FlowingFluid$SpreadContext;)I
private isWaterHole(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z
private canPassThrough(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)Z
private canMaybePassThrough(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)Z
private isSourceBlockOfThisType(Lnet/minecraft/world/level/material/FluidState;)Z
protected abstract getSlopeFindDistance(Lnet/minecraft/world/level/LevelReader;)I
private sourceNeighborCount(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)I
protected getSpread(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Ljava/util/Map;
private static canHoldAnyFluid(Lnet/minecraft/world/level/block/state/BlockState;)Z
private static canHoldFluid(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/Fluid;)Z
private static canHoldSpecificFluid(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/Fluid;)Z
protected abstract getDropOff(Lnet/minecraft/world/level/LevelReader;)I
protected getSpreadDelay(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/material/FluidState;)I
public tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V
protected static getLegacyLevel(Lnet/minecraft/world/level/material/FluidState;)I
private static hasSameAbove(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public getHeight(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public getOwnHeight(Lnet/minecraft/world/level/material/FluidState;)F
public abstract getAmount(Lnet/minecraft/world/level/material/FluidState;)I
public getShape(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$getShape$0(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$static$0()Lit/unimi/dsi/fastutil/objects/Object2ByteLinkedOpenHashMap;
static <clinit>()V
```
