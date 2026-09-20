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

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.sounds.SoundEngine|SoundEngine]] | `play(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/client/sounds/SoundEngine$PlayResult;` | wraps `@Redirect at INVOKE Lnet/minecraft/client/sounds/SoundBufferLibrary;getStream(Lnet/minecraft/resources/Identifier;Z)Ljava/util/concurrent/CompletableFuture;` | client | `SoundEngineMixin.getStream` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.sound.v1.FabricSoundInstance|FabricSoundInstance]] (interface, 3 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
