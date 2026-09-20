---
type: "interface"
fqcn: "net.minecraft.world.level.dimension.LevelStem"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.dimension.LevelStem

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `generator` | `()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | exact | invokevirtual@1 in `BiomeModificationImpl.lambda$finalizeWorldGen$1` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `generator` | `()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | exact | invokevirtual@1 in `BiomeModificationImpl.lambda$finalizeWorldGen$2` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `generator` | `()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | exact | invokevirtual@1 in `BiomeModificationImpl.lambda$finalizeWorldGen$3` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `generator` | `()Lnet/minecraft/world/level/chunk/ChunkGenerator;` | exact | invokevirtual@29 in `BiomeSelectionContextImpl.canGenerateIn` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `CODEC` | `Lcom/mojang/serialization/Codec;` | exact | getstatic@11 in `WorldDimensionsMixin.useFailSoftMap` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| reads | `END` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `BiomeSelectors.lambda$foundInTheEnd$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `NETHER` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `BiomeSelectors.lambda$foundInTheNether$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `OVERWORLD` | `Lnet/minecraft/resources/ResourceKey;` | exact | getstatic@1 in `BiomeSelectors.lambda$foundInOverworld$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (6 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/core/Holder;
private final generator : Lnet/minecraft/world/level/chunk/ChunkGenerator;
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final OVERWORLD : Lnet/minecraft/resources/ResourceKey;
public static final NETHER : Lnet/minecraft/resources/ResourceKey;
public static final END : Lnet/minecraft/resources/ResourceKey;
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/level/chunk/ChunkGenerator;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public type()Lnet/minecraft/core/Holder;
public generator()Lnet/minecraft/world/level/chunk/ChunkGenerator;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
