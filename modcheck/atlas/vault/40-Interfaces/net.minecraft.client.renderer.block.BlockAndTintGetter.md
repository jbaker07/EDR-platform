---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockAndTintGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockAndTintGetter

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/level/BlockAndLightGetter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `cardinalLighting` | `()Lnet/minecraft/world/level/CardinalLighting;` | exact | invokeinterface@899 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `cardinalLighting` | `()Lnet/minecraft/world/level/CardinalLighting;` | exact | invokeinterface@199 in `AltModelBlockRendererImpl.shadeQuad` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@79 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@133 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@166 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@220 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@253 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@307 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@340 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@394 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@461 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@558 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@655 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@752 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@809 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@883 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getBlockState` | `(Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/level/block/state/` | inherited_exact | invokeinterface@48 in `AltModelBlockRendererImpl.shouldCullFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/client/renderer/block/BlockAndTintGetter;` | exact | getstatic@9 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/client/renderer/block/BlockAndTintGetter;` | exact | getstatic@39 in `BlockStateModelWrapperMixin.update` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| reads | `EMPTY` | `Lnet/minecraft/client/renderer/block/BlockAndTintGetter;` | exact | getstatic@28 in `LevelRendererMixin.submitBreakingBlockModelProxy` | unknown | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final EMPTY : Lnet/minecraft/client/renderer/block/BlockAndTintGetter;
public abstract cardinalLighting()Lnet/minecraft/world/level/CardinalLighting;
public abstract getBlockTint(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
static <clinit>()V
```
