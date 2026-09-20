---
type: "interface"
fqcn: "net.minecraft.client.renderer.block.BlockModelLighter$Cache"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.block.BlockModelLighter$Cache

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@96 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@183 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@270 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@357 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@493 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@590 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@687 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@784 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@838 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@863 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getLightCoords` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@86 in `FlatLighter.light` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@111 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@198 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@285 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@372 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@478 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@575 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@672 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@769 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |
| calls | `getShadeBrightness` | `(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/clie` | exact | invokevirtual@892 in `AoCalculator.computeFace` | unknown | [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] | direct_reference |

## Declared members (4 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private enabled : Z
private final colorCache : Lit/unimi/dsi/fastutil/longs/Long2IntLinkedOpenHashMap;
private final brightnessCache : Lit/unimi/dsi/fastutil/longs/Long2FloatLinkedOpenHashMap;
private final cachedBrightnessGetter : Lnet/minecraft/util/LightCoordsUtil$BrightnessGetter;
public <init>()V
public enable()V
public disable()V
public getLightCoords(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public getShadeBrightness(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)F
private synthetic lambda$new$2(Lnet/minecraft/world/level/BlockAndLightGetter;Lnet/minecraft/core/BlockPos;)I
private synthetic lambda$new$1()Lit/unimi/dsi/fastutil/longs/Long2FloatLinkedOpenHashMap;
private synthetic lambda$new$0()Lit/unimi/dsi/fastutil/longs/Long2IntLinkedOpenHashMap;
```
