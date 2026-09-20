---
type: "interface"
fqcn: "net.minecraft.core.HolderLookup$Provider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.HolderLookup$Provider

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/HolderGetter$Provider`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `createSerializationContext` | `(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/Regist` | exact | invokeinterface@30 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createSerializationContext` | `(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/Regist` | exact | invokeinterface@12 in `FabricCodecDataProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createSerializationContext` | `(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/Regist` | exact | invokeinterface@4 in `FabricDynamicRegistryProvider.lambda$run$2` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createSerializationContext` | `(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/Regist` | exact | invokeinterface@19 in `FabricLootTableProviderImpl.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `listRegistryKeys` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@4 in `AdvancementHolderProvider.listRegistryKeys` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `listRegistryKeys` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@4 in `LootTableHolderProvider.listRegistryKeys` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `listRegistryKeys` | `()Ljava/util/stream/Stream;` | exact | invokeinterface@20 in `TagAliasLoader.prepare` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@46 in `AdvancementHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@5 in `FabricDynamicRegistryProvider$Entries.lambda$new$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@47 in `FabricRecipeProvider$2.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@46 in `LootTableHolderProvider.lookup` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookup` | `(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;` | exact | invokeinterface@15 in `LootUtil.getEntryOrDirect` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@6 in `BuiltInResourceKeys.biomeHolderGetter` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@5 in `FabricDynamicRegistryProvider$Entries.getLookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@5 in `FabricDynamicRegistryProvider$RegistryEntries.create` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@21 in `FabricRecipeProvider$FabricBootstrapContext.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@5 in `FabricRecipeProvider$FabricBootstrapContext.listContextElements` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@7 in `FabricLootTableContext.accept` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@5 in `FabricLootTableContext.lookup` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@5 in `FabricLootTableContext.listContextElements` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLooku` | exact | invokeinterface@265 in `TagAliasLoader.apply` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (0 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract listRegistryKeys()Ljava/util/stream/Stream;
public listRegistries()Ljava/util/stream/Stream;
public abstract lookup(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
public lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderLookup$RegistryLookup;
public createSerializationContext(Lcom/mojang/serialization/DynamicOps;)Lnet/minecraft/resources/RegistryOps;
public static create(Ljava/util/stream/Stream;)Lnet/minecraft/core/HolderLookup$Provider;
public allRegistriesLifecycle()Lcom/mojang/serialization/Lifecycle;
public synthetic lookupOrThrow(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/HolderGetter;
private static synthetic lambda$create$0(Lnet/minecraft/core/HolderLookup$RegistryLookup;)Lnet/minecraft/core/HolderLookup$RegistryLookup;
private static synthetic lambda$lookupOrThrow$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/IllegalStateException;
```
