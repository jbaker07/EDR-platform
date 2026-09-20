---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.behavior.GiveGiftToHero"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.behavior.GiveGiftToHero

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `net/minecraft/world/entity/ai/behavior/Behavior`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `()V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| reads | `GIFTS` | `Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | declared |

## Declared members (9 fields, 20 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final THROW_GIFT_AT_DISTANCE : I
private static final MIN_TIME_BETWEEN_GIFTS : I
private static final MAX_TIME_BETWEEN_GIFTS : I
private static final TIME_TO_DELAY_FOR_HEAD_TO_FINISH_TURNING : I
private static final GIFTS : Ljava/util/Map;
private static final SPEED_MODIFIER : F
private timeUntilNextGift : I
private giftGivenDuringThisRun : Z
private timeSinceStart : J
public <init>(I)V
protected checkExtraStartConditions(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;)Z
protected start(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;J)V
protected canStillUse(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;J)Z
protected tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;J)V
protected stop(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;J)V
private throwGift(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/world/entity/LivingEntity;)V
private static getLootTableToThrow(Lnet/minecraft/world/entity/npc/villager/Villager;)Lnet/minecraft/resources/ResourceKey;
private isHeroVisible(Lnet/minecraft/world/entity/npc/villager/Villager;)Z
private getNearestTargetableHero(Lnet/minecraft/world/entity/npc/villager/Villager;)Ljava/util/Optional;
private isHero(Lnet/minecraft/world/entity/player/Player;)Z
private isWithinThrowingDistance(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/world/entity/player/Player;)Z
private static calculateTimeUntilNextGift(Lnet/minecraft/server/level/ServerLevel;)I
protected synthetic checkExtraStartConditions(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;)Z
protected synthetic canStillUse(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;J)Z
protected synthetic stop(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;J)V
protected synthetic tick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;J)V
protected synthetic start(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;J)V
private static synthetic lambda$throwGift$0(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)V
static <clinit>()V
```
