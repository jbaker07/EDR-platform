---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.FluidRenderer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.FluidRenderer

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `tesselate` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | exact | invokevirtual@8 in `FluidRenderingImpl.lambda$renderVanillaDefault$0` | unknown | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |
| injects_into | `tesselate` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | name_only | @Inject at ['HEAD'] | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |
| injects_into | `tesselate` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraf` | name_only | @ModifyExpressionValue at ['MIXINEXTRAS:EXPRESSION'] | client | [[30-Mechanisms/fabric-rendering-fluids-v1|fabric-rendering-fluids-v1]] | direct_reference |

## Declared members (2 fields, 14 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MAX_FLUID_HEIGHT : F
public final fluidModels : Lnet/minecraft/client/renderer/block/FluidStateModelSet;
public <init>(Lnet/minecraft/client/renderer/block/FluidStateModelSet;)V
private static isNeighborSameFluid(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/material/FluidState;)Z
private static isFaceOccludedByState(Lnet/minecraft/core/Direction;FLnet/minecraft/world/level/block/state/BlockState;)Z
private static isFaceOccludedByNeighbor(Lnet/minecraft/core/Direction;FLnet/minecraft/world/level/block/state/BlockState;)Z
private static isFaceOccludedBySelf(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;)Z
public static shouldRenderFace(Lnet/minecraft/world/level/material/FluidState;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/Direction;Lnet/minecraft/world/level/material/FluidState;)Z
public tesselate(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/renderer/block/FluidRenderer$Output;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V
private addFace(Lcom/mojang/blaze3d/vertex/VertexConsumer;FFFFFFFFFFFFFFFFFFFFIIZ)V
private calculateAverageHeight(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/material/Fluid;FFFLnet/minecraft/core/BlockPos;)F
private addWeightedHeight([FF)V
private getHeight(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/BlockPos;)F
private getHeight(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/world/level/material/Fluid;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)F
private vertex(Lcom/mojang/blaze3d/vertex/VertexConsumer;FFFIFFI)V
private getLightCoords(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
```
