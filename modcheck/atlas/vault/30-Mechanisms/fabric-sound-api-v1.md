---
type: "mechanism"
module: "fabric-sound-api-v1"
version: "2.0.7+3434d6d95d"
sha256: "d2c61f4bc044025112c9f3c18a1adfe72e15a4d29c856033a7bd8cbdeb0fede5"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-sound-api-v1

**Version** `2.0.7+3434d6d95d` -- **artifact sha256** `d2c61f4bc044025112c9f3c18a1adfe72e15a4d29c856033a7bd8cbdeb0fede5`

## Declared (fabric.mod.json)

- environment: `client`
- depends: `{"fabricloader": ">=0.19.3", "minecraft": ">=1.19.2"}`
- entrypoints: `null`
- mixin configs: `[{"config": "fabric-sound-api-v1.mixins.json", "environment": "client"}]`
- access widener: `fabric-sound-api-v1.classtweaker`
- mixin classes: 2 found by annotation, 2 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.sounds.SoundEngine|SoundEngine]].`play` | `(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/client/sounds/SoundEngine$PlayResult;` | exact | @Redirect | INVOKE `Lnet/minecraft/client/sounds/SoundBufferLibrary;getStream(Lnet/minecraft/resources/Identifier;Z)Ljava/util/concurrent/CompletableFuture;` (exact) | client | 1000 (default) | `SoundEngineMixin.getStream` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.sound.v1.FabricSoundInstance|FabricSoundInstance]] (interface, 2 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
