---
type: "interface"
fqcn: "net.minecraft.world.level.material.Fluid"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.Fluid

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@1 in `FlowingFluidMixin.<init>` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `<init>` | `()V` | exact | invokespecial@1 in `LavaFluidMixin.<init>` | unknown | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |
| calls | `builtInRegistryHolder` | `()Lnet/minecraft/core/Holder$Reference;` | exact | invokevirtual@6 in `FluidVariant.typeHolder` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultFluidState` | `()Lnet/minecraft/world/level/material/FluidState;` | exact | invokevirtual@6 in `FluidVariantRenderHandler.getColor` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultFluidState` | `()Lnet/minecraft/world/level/material/FluidState;` | exact | invokevirtual@6 in `FluidVariantAttributeHandler.getName` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultFluidState` | `()Lnet/minecraft/world/level/material/FluidState;` | exact | invokevirtual@6 in `FluidVariantAttributeHandler.getLightEmission` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `defaultFluidState` | `()Lnet/minecraft/world/level/material/FluidState;` | exact | invokevirtual@16 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBucket` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@39 in `FluidStorage.lambda$static$2` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getBucket` | `()Lnet/minecraft/world/item/Item;` | exact | invokevirtual@33 in `EmptyBucketStorage.insert` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getPickupSound` | `()Ljava/util/Optional;` | exact | invokevirtual@6 in `FluidVariantAttributes.lambda$getFillSound$0` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `getStateDefinition` | `()Lnet/minecraft/world/level/block/state/StateDefinition;` | exact | invokevirtual@1 in `BootstrapMixin.lambda$afterInitialize$1` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `isSource` | `(Lnet/minecraft/world/level/material/FluidState;)Z` | exact | invokevirtual@19 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| injects_into | `getPickupSound` | `()Ljava/util/Optional;` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLUID_STATE_REGISTRY` | `Lnet/minecraft/core/IdMapper;` | exact | getstatic@29 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final FLUID_STATE_REGISTRY : Lnet/minecraft/core/IdMapper;
protected final stateDefinition : Lnet/minecraft/world/level/block/state/StateDefinition;
private defaultFluidState : Lnet/minecraft/world/level/material/FluidState;
private final builtInRegistryHolder : Lnet/minecraft/core/Holder$Reference;
protected <init>()V
protected createFluidStateDefinition(Lnet/minecraft/world/level/block/state/StateDefinition$Builder;)V
public getStateDefinition()Lnet/minecraft/world/level/block/state/StateDefinition;
protected final registerDefaultState(Lnet/minecraft/world/level/material/FluidState;)V
public final defaultFluidState()Lnet/minecraft/world/level/material/FluidState;
public abstract getBucket()Lnet/minecraft/world/item/Item;
protected animateTick(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/util/RandomSource;)V
protected tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V
protected randomTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/util/RandomSource;)V
protected entityInside(Lnet/minecraft/world/level/Level;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/InsideBlockEffectApplier;)V
protected getDripParticle()Lnet/minecraft/core/particles/ParticleOptions;
protected abstract canBeReplacedWith(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/Direction;)Z
protected abstract getFlow(Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/world/phys/Vec3;
public abstract getTickDelay(Lnet/minecraft/world/level/LevelReader;)I
protected isRandomlyTicking()Z
protected isEmpty()Z
protected abstract getExplosionResistance()F
public abstract getHeight(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)F
public abstract getOwnHeight(Lnet/minecraft/world/level/material/FluidState;)F
protected abstract createLegacyBlock(Lnet/minecraft/world/level/material/FluidState;)Lnet/minecraft/world/level/block/state/BlockState;
public abstract isSource(Lnet/minecraft/world/level/material/FluidState;)Z
public abstract getAmount(Lnet/minecraft/world/level/material/FluidState;)I
public isSame(Lnet/minecraft/world/level/material/Fluid;)Z
public is(Lnet/minecraft/tags/TagKey;)Z
public abstract getShape(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/shapes/VoxelShape;
public getAABB(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/AABB;
public getPickupSound()Ljava/util/Optional;
public builtInRegistryHolder()Lnet/minecraft/core/Holder$Reference;
static <clinit>()V
```
