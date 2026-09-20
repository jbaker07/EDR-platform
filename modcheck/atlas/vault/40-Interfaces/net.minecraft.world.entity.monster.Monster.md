---
type: "interface"
fqcn: "net.minecraft.world.entity.monster.Monster"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.monster.Monster

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`abstract_class` public abstract; extends `net/minecraft/world/entity/PathfinderMob`; implements `net/minecraft/world/entity/monster/Enemy`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | exact | invokespecial@3 in `BreezeMixin.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Lev` | exact | invokespecial@3 in `WardenMixin.<init>` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (0 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
protected <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public getSoundSource()Lnet/minecraft/sounds/SoundSource;
public aiStep()V
protected updateNoActionTime()V
protected getSwimSound()Lnet/minecraft/sounds/SoundEvent;
protected getSwimSplashSound()Lnet/minecraft/sounds/SoundEvent;
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
public getFallSounds()Lnet/minecraft/world/entity/LivingEntity$Fallsounds;
public getWalkTargetValue(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/LevelReader;)F
public static isDarkEnoughToSpawn(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
public static checkMonsterSpawnRules(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
public static checkAnyLightMonsterSpawnRules(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/LevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
public static checkSurfaceMonstersSpawnRules(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/core/BlockPos;Lnet/minecraft/util/RandomSource;)Z
public static createMonsterAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public shouldDropExperience()Z
protected shouldDropLoot(Lnet/minecraft/server/level/ServerLevel;)Z
public isPreventingPlayerRest(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/player/Player;)Z
public getProjectile(Lnet/minecraft/world/item/ItemStack;)Lnet/minecraft/world/item/ItemStack;
```
