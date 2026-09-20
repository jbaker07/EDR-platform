---
type: "interface"
fqcn: "net.minecraft.core.Holder$Reference"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.Holder$Reference

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/core/Holder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bindComponents` | `(Lnet/minecraft/core/component/DataComponentMap;)V` | exact | invokevirtual@77 in `DefaultItemComponentImpl$ModifyContextImpl.modify` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `bindValue` | `(Ljava/lang/Object;)V` | exact | invokevirtual@9 in `MappedRegistryMixin.set` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `createStandAlone` | `(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/ResourceKey;` | exact | invokestatic@11 in `FabricDynamicRegistryProvider$Entries.ref` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createStandAlone` | `(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/ResourceKey;` | exact | invokestatic@44 in `FabricDynamicRegistryProvider$RegistryEntries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `createStandAlone` | `(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/ResourceKey;` | exact | invokestatic@13 in `FabricLootTableContext.accept` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@24 in `BiomeSelectionContextImpl.hasTag` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `is` | `(Lnet/minecraft/tags/TagKey;)Z` | exact | invokevirtual@113 in `TagUtil.isIn` | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@181 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@32 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@39 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@2 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@2 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `VanillaAdventureAdvancementsMixin.lambda$onlyCheckVanillaEntities$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `ModelProviderBlockStateGeneratorCollectorMixin.lambda$filterBlocksForP | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `ModelProviderItemInfoCollectorMixin.lambda$filterItemsForProcessingMod | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@1 in `EntityLootSubProviderMixin.lambda$onlyVanillaEntities$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `key` | `()Lnet/minecraft/resources/ResourceKey;` | exact | invokevirtual@755 in `MappedRegistryMixin.remap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@8 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeFeatu | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@8 in `BiomeModificationContextImpl$GenerationSettingsContextImpl.removeCarve | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@163 in `CreativeModeTabsMixin.paginateTabs` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@11 in `CreativeModeTabsMixin.lambda$paginateTabs$0` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@6 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@6 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@9 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@9 in `FabricDynamicRegistryProvider$Entries.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `LootUtil.lambda$getEntryOrDirect$1` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@1 in `ReloadableServerRegistriesMixin.lambda$modifyLootTables$1` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `value` | `()Ljava/lang/Object;` | exact | invokevirtual@1030 in `MappedRegistryMixin.remap` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (6 fields, 24 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final owner : Lnet/minecraft/core/HolderOwner;
private tags : Ljava/util/Set;
private components : Lnet/minecraft/core/component/DataComponentMap;
private final type : Lnet/minecraft/core/Holder$Reference$Type;
private key : Lnet/minecraft/resources/ResourceKey;
private value : Ljava/lang/Object;
protected <init>(Lnet/minecraft/core/Holder$Reference$Type;Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/ResourceKey;Ljava/lang/Object;)V
public static createStandAlone(Lnet/minecraft/core/HolderOwner;Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Holder$Reference;
public static createIntrusive(Lnet/minecraft/core/HolderOwner;Ljava/lang/Object;)Lnet/minecraft/core/Holder$Reference;
public key()Lnet/minecraft/resources/ResourceKey;
public value()Ljava/lang/Object;
public is(Lnet/minecraft/resources/Identifier;)Z
public is(Lnet/minecraft/resources/ResourceKey;)Z
private boundTags()Ljava/util/Set;
public is(Lnet/minecraft/tags/TagKey;)Z
public is(Lnet/minecraft/core/Holder;)Z
public is(Ljava/util/function/Predicate;)Z
public canSerializeIn(Lnet/minecraft/core/HolderOwner;)Z
public unwrap()Lcom/mojang/datafixers/util/Either;
public unwrapKey()Ljava/util/Optional;
public kind()Lnet/minecraft/core/Holder$Kind;
public isBound()Z
public areComponentsBound()Z
 bindKey(Lnet/minecraft/resources/ResourceKey;)V
protected bindValue(Ljava/lang/Object;)V
 bindTags(Ljava/util/Collection;)V
public bindComponents(Lnet/minecraft/core/component/DataComponentMap;)V
public tags()Ljava/util/stream/Stream;
public components()Lnet/minecraft/core/component/DataComponentMap;
public toString()Ljava/lang/String;
```
