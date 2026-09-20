---
type: "interface"
fqcn: "net.minecraft.world.level.levelgen.presets.WorldPresets"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.levelgen.presets.WorldPresets

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `FLAT` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@15 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NORMAL : Lnet/minecraft/resources/ResourceKey;
public static final FLAT : Lnet/minecraft/resources/ResourceKey;
public static final FLAT_ALL_DIMENSIONS : Lnet/minecraft/resources/ResourceKey;
public static final LARGE_BIOMES : Lnet/minecraft/resources/ResourceKey;
public static final AMPLIFIED : Lnet/minecraft/resources/ResourceKey;
public static final SINGLE_BIOME_SURFACE : Lnet/minecraft/resources/ResourceKey;
public static final DEBUG : Lnet/minecraft/resources/ResourceKey;
public <init>()V
public static bootstrap(Lnet/minecraft/data/worldgen/BootstrapContext;)V
private static register(Ljava/lang/String;)Lnet/minecraft/resources/ResourceKey;
public static fromSettings(Lnet/minecraft/world/level/levelgen/WorldDimensions;)Ljava/util/Optional;
public static createNormalWorldDimensions(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/levelgen/WorldDimensions;
public static getNormalOverworld(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/dimension/LevelStem;
public static createTestWorldDimensions(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/world/level/levelgen/WorldDimensions;
private static synthetic lambda$fromSettings$0(Lnet/minecraft/world/level/dimension/LevelStem;)Ljava/util/Optional;
static <clinit>()V
```
