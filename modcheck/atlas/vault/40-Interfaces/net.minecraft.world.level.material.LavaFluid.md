---
type: "interface"
fqcn: "net.minecraft.world.level.material.LavaFluid"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.LavaFluid

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `net/minecraft/world/level/material/FlowingFluid`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `spreadTo` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (2 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LIGHT_EMISSION : I
public static final MIN_LEVEL_CUTOFF : F
public <init>()V
public getFlowing()Lnet/minecraft/world/level/material/Fluid;
public getSource()Lnet/minecraft/world/level/material/Fluid;
public getBucket()Lnet/minecraft/world/item/Item;
public animateTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/util/RandomSource;)V
public randomTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/util/RandomSource;)V
protected entityInside(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;)V
private hasFlammableNeighbours(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
private isFlammable(Lnet/minecraft/world/level/LevelReader;Lnet/minecraft/core/BlockPos;)Z
public getDripParticle()Lnet/minecraft/core/particles/ParticleOptions;
protected beforeDestroyingBlock(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public getSlopeFindDistance(Lnet/minecraft/world/level/LevelReader;)I
public createLegacyBlock(Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/world/level/block/state/BlockState;
public isSame(Lnet/minecraft/world/level/material/Fluid;)Z
public getDropOff(Lnet/minecraft/world/level/LevelReader;)I
public canBeReplacedWith(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/Direction;)Z
public getTickDelay(Lnet/minecraft/world/level/LevelReader;)I
public getSpreadDelay(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/material/FluidState;)I
private fizz(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;)V
protected canConvertToSource(Lnet/minecraft/server/level/ServerLevel;)Z
protected spreadTo(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/material/FluidState;)V
protected isRandomlyTicking()Z
protected getExplosionResistance()F
public getPickupSound()Ljava/util/Optional;
private static isFastLava(Lnet/minecraft/world/level/LevelReader;)Z
```
