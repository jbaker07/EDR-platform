---
type: "interface"
fqcn: "net.minecraft.world.level.block.LayeredCauldronBlock"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.LayeredCauldronBlock

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/block/AbstractCauldronBlock`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `LEVEL` | `Lnet/minecraft/world/level/block/state/properties/IntegerProperty;` | exact | getstatic@35 in `CauldronFluidContent.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MIN_FILL_LEVEL : I
public static final MAX_FILL_LEVEL : I
public static final LEVEL : Lnet/minecraft/world/level/block/state/properties/IntegerProperty;
private static final BASE_CONTENT_HEIGHT : I
private static final HEIGHT_PER_LEVEL : D
private static final FILLED_SHAPES : [Lnet/minecraft/world/phys/shapes/VoxelShape;
private final precipitationType : Lnet/minecraft/world/level/biome/Biome$Precipitation;
public <init>(Lnet/minecraft/world/level/biome/Biome$Precipitation;Lnet/minecraft/core/cauldron/CauldronInteraction$Dispatcher;Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;)V
public isFull(Lnet/minecraft/world/level/block/state/BlockState;)Z
protected canReceiveStalactiteDrip(Lnet/minecraft/world/level/material/Fluid;)Z
protected getContentHeight(Lnet/minecraft/world/level/block/state/BlockState;)D
private static getPixelContentHeight(I)D
protected getEntityInsideCollisionShape(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;)Lnet/minecraft/world/phys/shapes/VoxelShape;
protected entityInside(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;Z)V
private handleEntityOnFireInside(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public static lowerFillLevel(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;)V
public handlePrecipitation(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/biome/Biome$Precipitation;)V
protected getAnalogOutputSignal(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/core/Direction;)I
protected createBlockStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
protected receiveStalactiteDrip(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/Fluid;)V
private synthetic lambda$entityInside$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/Level;Lnet/minecraft/world/entity/Entity;)V
private static synthetic lambda$static$0()[Lnet/minecraft/world/phys/shapes/VoxelShape;
private static synthetic lambda$static$1(I)Lnet/minecraft/world/phys/shapes/VoxelShape;
static <clinit>()V
```
