---
type: "event"
event: "net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT"
callback: "net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents$AllowTerrainParticleTint"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT

Callback interface: `net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents$AllowTerrainParticleTint`

Module: [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `TerrainParticleMixin.removeUntintableParticles` @14 | [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]].`<init>` @WrapOperation INVOKE `Lnet/minecraft/client/color/block/BlockColors;getTintSource(Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/client/color/block/BlockTintSource;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
