---
type: "interface"
fqcn: "net.minecraft.world.level.biome.TheEndBiomeSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.TheEndBiomeSource

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `net/minecraft/world/level/biome/BiomeSource`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/Th` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `create` | `(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/Th` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| injects_into | `getNoiseBiome` | `(IIILnet/minecraft/world/level/biome/Climate$Sampler;)Lnet/minecraft/c` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/MapCodec;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | declared |

## Declared members (6 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private final end : Lnet/minecraft/core/Holder;
private final highlands : Lnet/minecraft/core/Holder;
private final midlands : Lnet/minecraft/core/Holder;
private final islands : Lnet/minecraft/core/Holder;
private final barrens : Lnet/minecraft/core/Holder;
public static create(Lnet/minecraft/core/HolderGetter;)Lnet/minecraft/world/level/biome/TheEndBiomeSource;
private <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;Lnet/minecraft/core/Holder;)V
protected collectPossibleBiomes()Ljava/util/stream/Stream;
protected codec()Lcom/mojang/serialization/MapCodec;
public createResolver(Lnet/minecraft/world/level/biome/Climate$Sampler;)Lnet/minecraft/world/level/biome/BiomeResolver;
private getNoiseBiome(IIILnet/minecraft/world/level/biome/Climate$Sampler;)Lnet/minecraft/core/Holder;
private synthetic lambda$createResolver$0(Lnet/minecraft/world/level/biome/Climate$Sampler;III)Lnet/minecraft/core/Holder;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
