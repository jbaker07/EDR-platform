---
type: "interface"
fqcn: "net.minecraft.world.entity.monster.Monster"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.monster.Monster

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world` | `` | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world` | `` | both | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.world.entity.monster.Monster extends net.minecraft.world.entity.PathfinderMob implements net.minecraft.world.entity.monster.Enemy {
    protected net.minecraft.world.entity.monster.Monster(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.monster.Monster>, net.minecraft.world.level.Level);
    public net.minecraft.sounds.SoundSource getSoundSource();
    public void aiStep();
    protected void updateNoActionTime();
    protected net.minecraft.sounds.SoundEvent getSwimSound();
    protected net.minecraft.sounds.SoundEvent getSwimSplashSound();
    protected net.minecraft.sounds.SoundEvent getHurtSound(net.minecraft.world.damagesource.DamageSource);
    protected net.minecraft.sounds.SoundEvent getDeathSound();
    public net.minecraft.world.entity.LivingEntity$Fallsounds getFallSounds();
    public float getWalkTargetValue(net.minecraft.core.BlockPos, net.minecraft.world.level.LevelReader);
    public static boolean isDarkEnoughToSpawn(net.minecraft.world.level.ServerLevelAccessor, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public static boolean checkMonsterSpawnRules(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.Mob>, net.minecraft.world.level.ServerLevelAccessor, net.minecraft.world.entity.EntitySpawnReason, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public static boolean checkAnyLightMonsterSpawnRules(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.monster.Monster>, net.minecraft.world.level.LevelAccessor, net.minecraft.world.entity.EntitySpawnReason, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public static boolean checkSurfaceMonstersSpawnRules(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.Mob>, net.minecraft.world.level.ServerLevelAccessor, net.minecraft.world.entity.EntitySpawnReason, net.minecraft.core.BlockPos, net.minecraft.util.RandomSource);
    public static net.minecraft.world.entity.ai.attributes.AttributeSupplier$Builder createMonsterAttributes();
    public boolean shouldDropExperience();
    protected boolean shouldDropLoot(net.minecraft.server.level.ServerLevel);
    public boolean isPreventingPlayerRest(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.player.Player);
    public net.minecraft.world.item.ItemStack getProjectile(net.minecraft.world.item.ItemStack);
}
```
