---
type: "interface"
fqcn: "net.minecraft.data.registries.VanillaRegistries"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.registries.VanillaRegistries

System: [[20-Systems/net.minecraft.data.registries|net.minecraft.data.registries]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createWorldLookup` | `()Lnet/minecraft/core/HolderLookup$Provider;` | exact | invokestatic@0 in `BuiltInResourceKeys.<clinit>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `validateThatAllBiomeFeaturesHaveBiomeFilter` | `(Lnet/minecraft/core/HolderLookup$Provider;)V` | exact | invokestatic@176 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RELOADABLE_BUILDER` | `Lnet/minecraft/core/RegistrySetBuilder;` | exact | getstatic@16 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RELOADABLE_BUILDER` | `Lnet/minecraft/core/RegistrySetBuilder;` | exact | getstatic@113 in `FabricDataGenHelper.createReloadableLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `WORLD_BUILDER` | `Lnet/minecraft/core/RegistrySetBuilder;` | exact | getstatic@16 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `WORLD_BUILDER` | `Lnet/minecraft/core/RegistrySetBuilder;` | exact | getstatic@110 in `FabricDataGenHelper.createWorldLookupProvider` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final WORLD_BUILDER : Lnet/minecraft/core/RegistrySetBuilder;
private static final RELOADABLE_BUILDER : Lnet/minecraft/core/RegistrySetBuilder;
public <init>()V
public static validateThatAllBiomeFeaturesHaveBiomeFilter(Lnet/minecraft/core/HolderLookup$Provider;)V
private static validatePlacedFeature(Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)Z
public static validateLootData(Lnet/minecraft/core/HolderLookup$Provider;)V
public static createWorldLookup()Lnet/minecraft/core/HolderLookup$Provider;
public static createReloadableLookup(Lnet/minecraft/core/HolderLookup$Provider;)Lnet/minecraft/core/HolderLookup$Provider;
private static synthetic lambda$validateLootData$1(Ljava/lang/String;Lnet/minecraft/util/ProblemReporter$Problem;)V
private static synthetic lambda$validateLootData$0(Lnet/minecraft/world/level/storage/loot/ValidationContextSource;Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/world/level/storage/loot/LootDataType;)V
private static synthetic lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$0(Lnet/minecraft/core/HolderLookup$RegistryLookup;Lnet/minecraft/core/Holder$Reference;)V
private static synthetic lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$1(Lnet/minecraft/core/HolderLookup$RegistryLookup;Lnet/minecraft/resources/Identifier;Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/core/Holder;)V
private static synthetic lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$3(Lnet/minecraft/core/Holder$Reference;Lnet/minecraft/world/level/levelgen/placement/PlacedFeature;)V
private static synthetic lambda$validateThatAllBiomeFeaturesHaveBiomeFilter$2(Lnet/minecraft/core/HolderLookup$RegistryLookup;Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/ResourceKey;)V
static <clinit>()V
```
