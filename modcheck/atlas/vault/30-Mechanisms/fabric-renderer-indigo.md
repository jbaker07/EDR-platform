---
type: "mechanism"
module: "fabric-renderer-indigo"
version: "9.1.23+b9d63e335d"
sha256: "1e71ac3ddf0dd1d1657425e02f5d9eae383fd8cbc47d0def4980e41d8a8b9c93"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-renderer-indigo

**Version** `9.1.23+b9d63e335d` -- **artifact sha256** `1e71ac3ddf0dd1d1657425e02f5d9eae383fd8cbc47d0def4980e41d8a8b9c93`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=26.1-rc.2", "fabric-api-base": "*", "fabric-renderer-api-v1": "*"}`
- entrypoints: `{"client": ["net.fabricmc.fabric.impl.client.indigo.Indigo"]}`
- mixin configs: `["fabric-renderer-indigo.mixins.json"]`
- access widener: `fabric-renderer-indigo.classtweaker`
- mixin classes: 2 found by annotation, 2 declared in configs; extraction failures: 0

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
