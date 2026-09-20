---
type: "interface"
fqcn: "net.minecraft.world.level.material.FluidState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.FluidState

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public final; extends `net/minecraft/world/level/block/state/StateHolder`; implements `net/minecraft/core/TypedInstance`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createLegacyBlock` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@9 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `createLegacyBlock` | `()Lnet/minecraft/world/level/block/state/BlockState;` | exact | invokevirtual@9 in `FluidVariantAttributeHandler.getLightEmission` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/level/material/Fluid;` | exact | invokevirtual@2 in `FlowingFluidMixin.shouldSpreadLiquid` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `getType` | `()Lnet/minecraft/world/level/material/Fluid;` | exact | invokevirtual@12 in `FluidRendererMixin.onHeadRender` | unknown | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@35 in `AbstractBoatMixin.customFluidSupport` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | inherited_exact | invokevirtual@71 in `EntityMixin.checkIfStandingInSwimmableFluid` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (3 fields, 25 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final AMOUNT_MAX : I
public static final AMOUNT_FULL : I
public <init>(Lnet/minecraft/world/level/material/Fluid;[Lnet/minecraft/world/level/block/state/properties/Property;[Ljava/lang/Comparable;)V
public getType()Lnet/minecraft/world/level/material/Fluid;
public isSource()Z
public isSourceOfType(Lnet/minecraft/world/level/material/Fluid;)Z
public isEmpty()Z
public getHeight(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public getHeightForCamera(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public getOwnHeight()F
public isFull()Z
public getAmount()I
public shouldRenderBackwardUpFace(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
public tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)V
public animateTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public isRandomlyTicking()Z
public randomTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)V
public getFlow(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
public createLegacyBlock()Lnet/minecraft/world/level/block/state/BlockState;
public getDripParticle()Lnet/minecraft/core/particles/ParticleOptions;
public typeHolder()Lnet/minecraft/core/Holder;
public getExplosionResistance()F
public canBeReplacedWith(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/Direction;)Z
public getShape(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getAABB(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/AABB;
public entityInside(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;)V
static <clinit>()V
```
