---
type: "interface"
fqcn: "net.minecraft.world.level.material.Fluids"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.material.Fluids

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@4 in `BootstrapMixin.afterInitialize` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@4 in `CauldronFluidContent.currentLevel` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@15 in `CauldronFluidContent.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@0 in `FluidVariant.blank` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@26 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@95 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/world/level/material/Fluid;` | exact | getstatic@4 in `FluidVariantImpl.isBlank` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `FLOWING_WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@31 in `FluidVariantRendering.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `LAVA` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@45 in `CauldronFluidContent.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `LAVA` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@29 in `FluidVariantAttributes.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@24 in `FluidVariantRendering.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@29 in `CauldronFluidContent.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@10 in `FluidStorage.lambda$static$3` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@43 in `FluidStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@200 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@16 in `FluidVariantAttributes.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `WATER` | `Lnet/minecraft/world/level/material/FlowingFluid;` | exact | getstatic@0 in `WaterPotionStorage.<clinit>` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (5 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/world/level/material/Fluid;
public static final FLOWING_WATER : Lnet/minecraft/world/level/material/FlowingFluid;
public static final WATER : Lnet/minecraft/world/level/material/FlowingFluid;
public static final FLOWING_LAVA : Lnet/minecraft/world/level/material/FlowingFluid;
public static final LAVA : Lnet/minecraft/world/level/material/FlowingFluid;
public <init>()V
private static register(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/world/level/material/Fluid;)Lnet/minecraft/world/level/material/Fluid;
static <clinit>()V
```
