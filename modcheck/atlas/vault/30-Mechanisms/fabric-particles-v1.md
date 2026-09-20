---
type: "mechanism"
module: "fabric-particles-v1"
version: "5.0.24+3434d6d95d"
sha256: "0bf0c29bd7f1803eac7c5d4aec1af51bb25790ba41583108a2320740043cd0c2"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-particles-v1

**Version** `5.0.24+3434d6d95d` -- **artifact sha256** `0bf0c29bd7f1803eac7c5d4aec1af51bb25790ba41583108a2320740043cd0c2`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-networking-api-v1": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.particle.ExtendedBlockParticleOptionSync"], "client": ["net.fabricmc.fabric.impl.client.particle.ExtendedBlockParticleOptionSyncClient"]}`
- mixin configs: `["fabric-particles-v1.mixins.json", {"config": "fabric-particles-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-particles-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT|ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.particle.ParticleEngine|ParticleEngine]] | `<clinit>` | injects_into `@Inject at RETURN` | client | `ParticleEngineMixin.classInit` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleEngine|ParticleEngine]] | `createParticleGroup` | injects_into `@Inject at NEW (Lnet/minecraft/client/particle/ParticleEngine;Lnet/minecraft/client/particle/ParticleRenderType;)Lnet/minecraft/client/particle/QuadParticleGroup;` | client | `ParticleEngineMixin.createParticleGroup` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleResources|ParticleResources]] | `registerProviders` | injects_into `@Inject at RETURN` | client | `ParticleResourcesMixin.onRegisterDefaultFactories` |
| [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]] | `createTerrainParticle` | wraps `@Redirect at NEW (Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/particle/TerrainParticle;` | client | `TerrainParticleMixin.constructTerrainParticle` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.FabricSpriteSet|FabricSpriteSet]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleGroupRegistry|ParticleGroupRegistry]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry|ParticleProviderRegistry]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents|ParticleRenderEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.particle.v1.FabricBlockParticleOption|FabricBlockParticleOption]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.particle.v1.FabricParticleTypes|FabricParticleTypes]] (class, 6 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
