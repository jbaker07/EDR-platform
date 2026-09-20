---
type: "mechanism"
module: "fabric-convention-tags-v2"
version: "4.10.3+6b5b47fd5d"
sha256: "96fa76cd74df23aa2332980b0577eb88ec3ef8a2ab7c0eb0c99810bbc6390fe9"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-convention-tags-v2

**Version** `4.10.3+6b5b47fd5d` -- **artifact sha256** `96fa76cd74df23aa2332980b0577eb88ec3ef8a2ab7c0eb0c99810bbc6390fe9`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=1.20.5-beta.1", "fabric-lifecycle-events-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.tag.convention.v2.TranslationConventionLogWarnings"]}`
- mixin configs: `["fabric-convention-tags-api-v2.mixins.json"]`
- access widener: `fabric-convention-tags-v2.classtweaker`
- mixin classes: 1 found by annotation, 1 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.tag.FabricTagKey|FabricTagKey]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalBiomeTags|ConventionalBiomeTags]] (class, 94 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalBlockItemTags|ConventionalBlockItemTags]] (class, 132 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalBlockTags|ConventionalBlockTags]] (class, 133 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalEnchantmentTags|ConventionalEnchantmentTags]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalEntityTypeTags|ConventionalEntityTypeTags]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalFluidTags|ConventionalFluidTags]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalItemTags|ConventionalItemTags]] (class, 272 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalPotionTags|ConventionalPotionTags]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.ConventionalStructureTags|ConventionalStructureTags]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.tag.convention.v2.TagUtil|TagUtil]] (class, 4 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
