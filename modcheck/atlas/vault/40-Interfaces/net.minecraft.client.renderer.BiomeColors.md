---
type: "interface"
fqcn: "net.minecraft.client.renderer.BiomeColors"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.BiomeColors

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `FOLIAGE_COLOR_RESOLVER` | `Lnet/minecraft/world/level/ColorResolver;` | exact | getstatic@53 in `ColorResolverRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `GRASS_COLOR_RESOLVER` | `Lnet/minecraft/world/level/ColorResolver;` | exact | getstatic@41 in `ColorResolverRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| reads | `WATER_COLOR_RESOLVER` | `Lnet/minecraft/world/level/ColorResolver;` | exact | getstatic@65 in `ColorResolverRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final GRASS_COLOR_RESOLVER : Lnet/minecraft/world/level/ColorResolver;
public static final FOLIAGE_COLOR_RESOLVER : Lnet/minecraft/world/level/ColorResolver;
public static final DRY_FOLIAGE_COLOR_RESOLVER : Lnet/minecraft/world/level/ColorResolver;
public static final WATER_COLOR_RESOLVER : Lnet/minecraft/world/level/ColorResolver;
public <init>()V
private static getAverageColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/ColorResolver;)I
public static getAverageGrassColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public static getAverageFoliageColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public static getAverageDryFoliageColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
public static getAverageWaterColor(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;)I
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/Biome;DD)I
private static synthetic lambda$static$1(Lnet/minecraft/world/level/biome/Biome;DD)I
private static synthetic lambda$static$0(Lnet/minecraft/world/level/biome/Biome;DD)I
static <clinit>()V
```
