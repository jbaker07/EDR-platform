---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup$RegistryLookup"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup$RegistryLookup

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderLookup`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@4 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Refe` | inherited_exact | invokeinterface@4 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@59 in `RegistryCustomContentState.lambda$construct$1` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokeinterface@1 in `RegistryCustomContentState.lambda$construct$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElementIds` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@1 in `FabricDynamicRegistryProvider$Entries.addAll` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listElementIds` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@9 in `RegistryCustomContentState.lambda$construct$1` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@11 in `AdvancementLookup.listElements` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@10 in `FabricRecipeProvider$FabricBootstrapContext.listContextElements` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@10 in `FabricLootTableContext.listContextElements` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listElements` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@11 in `LootTableLookup.listElements` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `listTags` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@11 in `AdvancementLookup.listTags` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listTags` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@11 in `LootTableLookup.listTags` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registryLifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokeinterface@11 in `AdvancementLookup.registryLifecycle` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `registryLifecycle` | `()Lcom/mojang/serialization/Lifecycle;` | exact | invokeinterface@11 in `LootTableLookup.registryLifecycle` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |

## Declared members (0 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract key()Lnet/minecraft/resources/ResourceKey;
public abstract registryLifecycle()Lcom/mojang/serialization/Lifecycle;
public filterFeatures(Lnet/minecraft/world/flag/FeatureFlagSet;)Lnet/minecraft/core/HolderLookup$RegistryLookup;
public filterElements(Ljava/util/function/Predicate;)Lnet/minecraft/core/HolderLookup$RegistryLookup;
private static synthetic lambda$filterFeatures$0(Lnet/minecraft/world/flag/FeatureFlagSet;Ljava/lang/Object;)Z
```
