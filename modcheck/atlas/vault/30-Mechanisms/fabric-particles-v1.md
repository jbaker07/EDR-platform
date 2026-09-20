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
- mixin classes: 12 found by annotation, 12 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT|ParticleRenderEvents.ALLOW_TERRAIN_PARTICLE_TINT]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.particle.ParticleEngine|ParticleEngine]].`<clinit>` | `()V` | exact | @Inject | RETURN | client | 1000 (default) | `ParticleEngineMixin.classInit` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleEngine|ParticleEngine]].`createParticleGroup` | `(Lnet/minecraft/client/particle/ParticleRenderType;)Lnet/minecraft/client/particle/ParticleGroup;` | name_only | @Inject | NEW `(Lnet/minecraft/client/particle/ParticleEngine;Lnet/minecraft/client/particle/ParticleRenderType;)Lnet/minecraft/client/particle/QuadParticleGroup;` (exact) | client | 1000 (default) | `ParticleEngineMixin.createParticleGroup` |
| [[40-Interfaces/net.minecraft.client.particle.ParticleResources|ParticleResources]].`registerProviders` | `()V` | name_only | @Inject | RETURN | client | 1000 (default) | `ParticleResourcesMixin.onRegisterDefaultFactories` |
| [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]].`<init>` | `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/client/color/block/BlockColors;getTintSource(Lnet/minecraft/world/level/block/state/BlockState;I)Lnet/minecraft/client/color/block/BlockTintSource;` (exact) | client | 1000 (default) | `TerrainParticleMixin.removeUntintableParticles` |
| [[40-Interfaces/net.minecraft.client.particle.TerrainParticle|TerrainParticle]].`createTerrainParticle` | `(Lnet/minecraft/core/particles/BlockParticleOption;Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDD)Lnet/minecraft/client/particle/TerrainParticle;` | name_only | @Redirect | NEW `(Lnet/minecraft/client/multiplayer/ClientLevel;DDDDDDLnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/client/particle/TerrainParticle;` (exact) | client | 1000 (default) | `TerrainParticleMixin.constructTerrainParticle` |
| [[40-Interfaces/net.minecraft.core.particles.BlockParticleOption|BlockParticleOption]].`streamCodec` | `(Lnet/minecraft/core/particles/ParticleType;)Lnet/minecraft/network/codec/StreamCodec;` | name_only | @ModifyReturnValue | RETURN | both | 1000 (default) | `BlockParticleOptionMixin.modifyStreamCodec` |
| [[40-Interfaces/net.minecraft.server.level.ServerPlayer|ServerPlayer]].`checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `ServerPlayerMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.util.ParticleUtils|ParticleUtils]].`spawnSmashAttackParticles` | `(Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/core/BlockPos;I)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `ParticleUtilsMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]].`spawnSprintParticle` | `()V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `EntityMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`checkFallDamage` | `(DZLnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `LivingEntityMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.entity.monster.breeze.Breeze|Breeze]].`emitGroundParticles` | `(I)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `BreezeMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.entity.monster.breeze.Breeze|Breeze]].`emitJumpTrailParticles` | `()V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `BreezeMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.entity.monster.warden.Warden|Warden]].`clientDiggingParticles` | `(Lnet/minecraft/world/entity/AnimationState;)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `WardenMixin.modifyBlockStateParticleOption` |
| [[40-Interfaces/net.minecraft.world.item.BrushItem|BrushItem]].`spawnDustParticles` | `(Lnet/minecraft/world/level/Level;Lnet/minecraft/world/phys/BlockHitResult;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/world/entity/HumanoidArm;)V` | name_only | @ModifyExpressionValue | NEW `(Lnet/minecraft/core/particles/ParticleType;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/core/particles/BlockParticleOption;` (exact) | both | 1000 (default) | `BrushItemMixin.modifyBlockStateParticleOption` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.FabricSpriteSet|FabricSpriteSet]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleGroupRegistry|ParticleGroupRegistry]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry|ParticleProviderRegistry]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.particle.v1.ParticleRenderEvents|ParticleRenderEvents]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.particle.v1.FabricBlockParticleOption|FabricBlockParticleOption]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.particle.v1.FabricParticleTypes|FabricParticleTypes]] (class, 6 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
