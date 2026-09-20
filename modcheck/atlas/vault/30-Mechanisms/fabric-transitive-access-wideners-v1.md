---
type: "mechanism"
module: "fabric-transitive-access-wideners-v1"
version: "8.1.16+52268a0e5d"
sha256: "8bb488c84c37af37c6a07d383fdf81a30da404654208e5d66e10068c0a72b641"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-transitive-access-wideners-v1

**Version** `8.1.16+52268a0e5d` -- **artifact sha256** `8bb488c84c37af37c6a07d383fdf81a30da404654208e5d66e10068c0a72b641`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3"}`
- entrypoints: `null`
- mixin configs: `null`
- access widener: `fabric-transitive-access-wideners-v1.classtweaker`
- mixin classes: 0 found by annotation, 0 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|

## API surface


## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
