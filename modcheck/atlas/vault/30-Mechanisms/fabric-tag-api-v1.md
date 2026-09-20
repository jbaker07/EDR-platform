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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.core.MappedRegistry_3|MappedRegistry$3]] | `apply` | injects_into `@Inject at INVOKE Lnet/minecraft/core/MappedRegistry;refreshTagsInHolders()V` | both | `MappedRegistry3Mixin.applyTagAliases` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `<init>` | injects_into `@Inject at RETURN` | both | `ReloadableServerResourcesMixin.storeDynamicRegistries` |
| [[40-Interfaces/net.minecraft.server.ReloadableServerResources|ReloadableServerResources]] | `updateComponentsAndStaticRegistryTags` | injects_into `@Inject at RETURN` | both | `ReloadableServerResourcesMixin.applyDynamicTagAliases` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]] | `build` | injects_into `@Inject at RETURN` | both | `TagLoaderMixin.removeTagRemovalReferencesWhenFinished` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]] | `lambda$build$0` | injects_into `@ModifyArg at INVOKE Lnet/minecraft/tags/TagLoader$SortingEntry;<init>(Ljava/util/List;)V` | both | `TagLoaderMixin.addTagRemovalReferencesToDependencySorter` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]] | `load` | injects_into `@Inject at INVOKE Ljava/util/List;clear()V` | both | `TagLoaderMixin.removeTagRemovalReferenceOnReplace` |
| [[40-Interfaces/net.minecraft.tags.TagLoader|TagLoader]] | `load` | injects_into `@Inject at INVOKE Ljava/util/List;forEach(Ljava/util/function/Consumer;)V` | both | `TagLoaderMixin.loadRemoveEntries` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.tag.client.v1.ClientTags|ClientTags]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.v1.FabricTagFile|FabricTagFile]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
