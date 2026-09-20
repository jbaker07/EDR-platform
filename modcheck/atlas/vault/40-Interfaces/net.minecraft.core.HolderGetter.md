---
type: "interface"
fqcn: "net.minecraft.core.HolderGetter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderGetter

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderOwner`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@2 in `AdvancementHolderProvider.lambda$get$0` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `AdvancementLookup.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@4 in `BuiltInResourceKeys.isBuiltinBiome` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@2 in `LootTableHolderProvider.lambda$get$0` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `LootTableLookup.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@87 in `ResourceConditionsImpl.registryContains` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@2 in `AdvancementHolderProvider.lambda$get$1` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `AdvancementLookup.get` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@2 in `LootTableHolderProvider.lambda$get$1` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `LootTableLookup.get` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `get` | `(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;` | exact | invokeinterface@87 in `ResourceConditionsImpl.tagsPopulated` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@20 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@59 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@72 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@85 in `TheEndBiomeData$Overrides.<init>` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@110 in `TheEndBiomeData$Overrides.resolveOverrides` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | exact | invokeinterface@20 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;` | exact | invokeinterface@20 in `AdvancementHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;` | exact | invokeinterface@20 in `LootTableHolderProvider.getOrThrow` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (0 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract get(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public getOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;
public abstract get(Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
public getOrThrow(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/core/HolderSet$Named;
public getRandomElementOf(Lnet/minecraft/tags/TagKey;Lnet/minecraft/util/RandomSource;)Ljava/util/Optional;
private static synthetic lambda$getRandomElementOf$0(Lnet/minecraft/util/RandomSource;Lnet/minecraft/core/HolderSet$Named;)Ljava/util/Optional;
private static synthetic lambda$getOrThrow$1(Lnet/minecraft/tags/TagKey;)Ljava/lang/IllegalStateException;
private static synthetic lambda$getOrThrow$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/IllegalStateException;
```
