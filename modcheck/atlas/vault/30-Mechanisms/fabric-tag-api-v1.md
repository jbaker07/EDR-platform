---
type: "mechanism"
module: "fabric-tag-api-v1"
version: "2.1.10+fcdff87f5d"
sha256: "6c2fa7a4d870ee33305c946e5dce6870f92f7f909fa0aa84011d976527e1b3cb"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-tag-api-v1

**Version** `2.1.10+fcdff87f5d` -- **artifact sha256** `6c2fa7a4d870ee33305c946e5dce6870f92f7f909fa0aa84011d976527e1b3cb`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.tag.TagInit"]}`
- mixin configs: `["fabric-tag-api-v1.mixins.json"]`
- access widener: `fabric-tag-api-v1.classtweaker`
- mixin classes: 8 found by annotation, 8 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.core.MappedRegistry_3|MappedRegistry$3]].`apply` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/core/MappedRegistry;refreshTagsInHolders()V` (exact) | both | 1000 (default) | `MappedRegistry3Mixin.applyTagAliases` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`<init>` | `(Lnet/minecraft/server/ReloadableServerRegistries$LoadResult;Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/commands/Commands$CommandSelection;Ljava/util/List;Lnet/minecraft/server/permissions/PermissionSet;Ljava/util/List;)V` | name_only | @Inject | RETURN | both | 999 | `ReloadableServerResourcesMixin.storeDynamicRegistries` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]].`updateComponentsAndStaticRegistryTags` | `()V` | name_only | @Inject | RETURN | both | 999 | `ReloadableServerResourcesMixin.applyDynamicTagAliases` |
| [[40-Interfaces/net.minecraft.tags.TagFile|TagFile]].`<clinit>` | `()V` | exact | @ModifyExpressionValue | INVOKE `Lcom/mojang/serialization/codecs/RecordCodecBuilder;create(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;` (exact) | both | 1000 (default) | `TagFileMixin.modifyCodec` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`build` | `(Ljava/util/Map;)Ljava/util/Map;` | name_only | @Inject | RETURN | both | 1000 (default) | `TagLoaderMixin.removeTagRemovalReferencesWhenFinished` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`lambda$build$0` | `(Lnet/minecraft/util/DependencySorter;Lnet/minecraft/resources/Identifier;Ljava/util/List;)V` | name_only | @ModifyArg | INVOKE `Lnet/minecraft/tags/TagLoader$SortingEntry;<init>(Ljava/util/List;)V` (exact) | both | 1000 (default) | `TagLoaderMixin.addTagRemovalReferencesToDependencySorter` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`lambda$build$1` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/Map;Lnet/minecraft/resources/Identifier;Lnet/minecraft/tags/TagLoader$SortingEntry;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/tags/TagLoader;tryBuildTag(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/List;)Lcom/mojang/datafixers/util/Either;` (exact) | both | 1000 (default) | `TagLoaderMixin.scopeIdToTryBuildTag` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;` | name_only | @Inject | INVOKE `Ljava/util/List;clear()V` (exact) | both | 1000 (default) | `TagLoaderMixin.removeTagRemovalReferenceOnReplace` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`load` | `(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljava/util/Map;` | name_only | @Inject | INVOKE `Ljava/util/List;forEach(Ljava/util/function/Consumer;)V` (inherited_exact) | both | 1000 (default) | `TagLoaderMixin.loadRemoveEntries` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`loadTagsForRegistry` | `(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/core/WritableRegistry;)V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/tags/TagLoader;loadTagsForRegistry(Lnet/minecraft/server/packs/resources/ResourceManager;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/tags/TagLoader$ElementLookup;)Ljava/util/Map;` (exact) | both | 1000 (default) | `TagLoaderMixin.loadTagsForRegistry` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]].`tryBuildTag` | `(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/List;)Lcom/mojang/datafixers/util/Either;` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/tags/TagEntry;build(Lnet/minecraft/tags/TagEntry$Lookup;Ljava/util/function/Consumer;)Z` (exact) | both | 1000 (default) | `TagLoaderMixin.removeEntriesFromTags` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.tag.client.v1.ClientTags|ClientTags]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.v1.FabricTagFile|FabricTagFile]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
