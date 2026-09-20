---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.MovingBlockRenderState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.MovingBlockRenderState

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/renderer/block/BlockAndTintGetter`, `net/fabricmc/fabric/api/client/rendering/v1/FabricRenderState`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `biome` | `Lnet/minecraft/core/Holder;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-block-getter-api-v2|fabric-block-getter-api-v2]] | declared |
| reads | `blockPos` | `Lnet/minecraft/core/BlockPos;` | exact | getfield@27 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `blockState` | `Lnet/minecraft/world/level/block/state/BlockState;` | exact | getfield@1 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `randomSeedPos` | `Lnet/minecraft/core/BlockPos;` | exact | getfield@9 in `SubmitNodeCollectionMixin.hasMaterialFlagProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (6 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public randomSeedPos : Lnet/minecraft/core/BlockPos;
public blockPos : Lnet/minecraft/core/BlockPos;
public blockState : Lnet/minecraft/world/level/block/state/BlockState;
public biome : Lnet/minecraft/core/Holder;
public cardinalLighting : Lnet/minecraft/world/level/CardinalLighting;
public lightEngine : Lnet/minecraft/world/level/lighting/LevelLightEngine;
public <init>()V
public cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;
public getLightEngine()Lnet/minecraft/world/level/lighting/LevelLightEngine;
public getBlockTint(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
public getBlockEntity(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/entity/BlockEntity;
public getBlockState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/BlockState;
public getFluidState(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/material/FluidState;
public getHeight()I
public getMinY()I
```
