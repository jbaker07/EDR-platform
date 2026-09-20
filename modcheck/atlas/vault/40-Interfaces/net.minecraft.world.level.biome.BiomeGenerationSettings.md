---
type: "interface"
fqcn: "net.minecraft.world.level.biome.BiomeGenerationSettings"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.biome.BiomeGenerationSettings

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `features` | `()Ljava/util/List;` | exact | invokevirtual@9 in `BiomeSelectionContext.hasFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `features` | `()Ljava/util/List;` | exact | invokevirtual@9 in `BiomeSelectionContext.hasPlacedFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `features` | `()Ljava/util/List;` | exact | invokevirtual@8 in `BiomeModificationImpl.lambda$finalizeWorldGen$3` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `carvers` | `Lnet/minecraft/core/HolderSet;` | exact | getfield@9 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addCarver` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `carvers` | `Lnet/minecraft/core/HolderSet;` | exact | getfield@23 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeCarver` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@12 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.unfreezeFeature | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@8 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.freezeFeatures` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@25 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@4 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@4 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$rebuildFl | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| reads | `features` | `Ljava/util/List;` | exact | getfield@4 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$freezeFea | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `boneMealFeatures` | `Ljava/util/function/Supplier;` | exact | putfield@13 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.rebuildFlowerFe | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `carvers` | `Lnet/minecraft/core/HolderSet;` | exact | putfield@23 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.addCarver` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `carvers` | `Lnet/minecraft/core/HolderSet;` | exact | putfield@63 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeCarver` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `featureSet` | `Ljava/util/function/Supplier;` | exact | putfield@30 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.freezeFeatures` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `features` | `Ljava/util/List;` | exact | putfield@18 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.unfreezeFeature | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| writes | `features` | `Ljava/util/List;` | exact | putfield@14 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.freezeFeatures` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (7 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final EMPTY : Lnet/minecraft/world/level/biome/BiomeGenerationSettings;
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private final carvers : Lnet/minecraft/core/HolderSet;
private final features : Ljava/util/List;
private final boneMealFeatures : Ljava/util/function/Supplier;
private final featureSet : Ljava/util/function/Supplier;
private <init>(Lnet/minecraft/core/HolderSet;Ljava/util/List;)V
public getCarvers()Ljava/lang/Iterable;
public getBoneMealFeatures()Ljava/util/List;
public features()Ljava/util/List;
public hasFeature(Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)Z
private static synthetic lambda$new$3(Ljava/util/List;)Ljava/util/Set;
private static synthetic lambda$new$0(Ljava/util/List;)Ljava/util/List;
private static synthetic lambda$new$2(Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$new$1(Lnet/minecraft/core/Holder;)Ljava/util/stream/Stream;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
private static synthetic lambda$static$2(Lnet/minecraft/world/level/biome/BiomeGenerationSettings;)Ljava/util/List;
private static synthetic lambda$static$1(Lnet/minecraft/world/level/biome/BiomeGenerationSettings;)Lnet/minecraft/core/HolderSet;
static <clinit>()V
```
