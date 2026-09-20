---
type: "interface"
fqcn: "net.minecraft.core.Holder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Holder

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@9 in `BlockTransformerHelperImpl.createStripping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@9 in `BlockTransformerHelperImpl.createOxidationScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@9 in `BlockTransformerHelperImpl.createWaxScraping` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@1 in `LootUtil.lambda$getEntryOrDirect$2` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@5 in `LootPoolBuilderMixin.when` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@5 in `LootPoolBuilderMixin.apply` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `direct` | `(Ljava/lang/Object;)Lnet/minecraft/core/Holder;` | exact | invokestatic@5 in `LootTableBuilderMixin.apply` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | inherited_exact | invokeinterface@5 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `equals` | `(Ljava/lang/Object;)Z` | inherited_exact | invokeinterface@53 in `BrewingProviderMixin.preventDuplicatingDefaultTransformations` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `getRegisteredName` | `()Ljava/lang/String;` | exact | invokeinterface@8 in `DebugMessages.forGlobalPos` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/util/function/Predicate;)Z` | exact | invokeinterface@16 in `TheEndBiomeData$Overrides.pick` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/util/function/Predicate;)Z` | exact | invokeinterface@44 in `TheEndBiomeData$Overrides.pick` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `is` | `(Ljava/util/function/Predicate;)Z` | exact | invokeinterface@58 in `TheEndBiomeData$Overrides.pick` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokeinterface@4 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$re | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokeinterface@53 in `ClientTagsImpl.isInWithLocalFallback` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `kind` | `()Lnet/minecraft/core/Holder$Kind;` | exact | invokeinterface@18 in `TheEndBiomeData$ResourceKeyHashStrategy.equals` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `kind` | `()Lnet/minecraft/core/Holder$Kind;` | exact | invokeinterface@24 in `TheEndBiomeData$ResourceKeyHashStrategy.equals` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrap` | `()Lcom/mojang/datafixers/util/Either;` | exact | invokeinterface@35 in `TheEndBiomeData$ResourceKeyHashStrategy.equals` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrap` | `()Lcom/mojang/datafixers/util/Either;` | exact | invokeinterface@7 in `TheEndBiomeData$ResourceKeyHashStrategy.hashCode` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@112 in `TheEndBiomeData$Overrides.pick` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@1 in `TheEndBiomeData$ResourceKeyHashStrategy.lambda$equals$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@2 in `FabricSoundsProvider$SoundExporter.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@11 in `SoundTypeBuilderImpl$RegistrationBuilderImpl.ofEvent` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@121 in `FabricItem.getCreatorNamespace` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@60 in `ClientTagsImpl.isInWithLocalFallback` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `unwrapKey` | `()Ljava/util/Optional;` | exact | invokeinterface@80 in `ClientTagsImpl.isInWithLocalFallback` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@73 in `BiomeSelectionContext.hasFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@74 in `BiomeSelectionContext.hasPlacedFeature` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@2 in `BiomeSelectionContext.lambda$hasFeature$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@47 in `TheEndBiomeData$ResourceKeyHashStrategy.equals` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$re | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$re | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.lambda$re | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `BiomeSelectionContextImpl.lambda$canGenerateIn$0` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@107 in `FlammableBlockRegistryImpl.getEntryMap` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@2 in `FabricLanguageProvider$TranslationBuilder.addAttribute` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@66 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@4 in `ServerMobEffectEvents.lambda$static$14` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@4 in `ServerMobEffectEvents.lambda$static$13` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@4 in `ServerMobEffectEvents.lambda$static$12` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@8 in `FabricItem.canBeEnchantedWith` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@24 in `FabricItem.canBeEnchantedWith` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@2 in `AllIngredient.lambda$items$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `FluidVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokeinterface@1 in `ItemVariantImpl.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract value()Ljava/lang/Object;
public abstract isBound()Z
public abstract areComponentsBound()Z
public abstract is(Lnet/minecraft/resources/Identifier;)Z
public abstract is(Lnet/minecraft/resources/ResourceKey;)Z
public abstract is(Ljava/util/function/Predicate;)Z
public abstract is(Lnet/minecraft/tags/TagKey;)Z
public abstract is(Lnet/minecraft/core/Holder;)Z
public abstract tags()Ljava/util/stream/Stream;
public abstract components()Lnet/minecraft/core/component/DataComponentMap;
public abstract unwrap()Lcom/mojang/datafixers/util/Either;
public abstract unwrapKey()Ljava/util/Optional;
public abstract kind()Lnet/minecraft/core/Holder$Kind;
public abstract canSerializeIn(Lnet/minecraft/core/HolderOwner;)Z
public getRegisteredNameIfPresent()Ljava/util/Optional;
public getRegisteredName()Ljava/lang/String;
public static direct(Ljava/lang/Object;)Lnet/minecraft/core/Holder;
public static direct(Ljava/lang/Object;Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecraft/core/Holder;
private static synthetic lambda$getRegisteredNameIfPresent$0(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
```
