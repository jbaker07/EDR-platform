---
type: "interface"
fqcn: "net.minecraft.world.entity.npc.villager.Villager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.npc.villager.Villager

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `net/minecraft/world/entity/npc/villager/AbstractVillager`; implements `net/minecraft/world/entity/npc/villager/VillagerDataHolder`, `net/minecraft/world/entity/ReputationEventHandler`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `wantsToPickUp` | `(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/Ite` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (28 fields, 94 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DATA_VILLAGER_DATA : Lnet/minecraft/network/syncher/EntityDataAccessor;
private static final DATA_VILLAGER_DATA_FINALIZED : Lnet/minecraft/network/syncher/EntityDataAccessor;
public static final BREEDING_FOOD_THRESHOLD : I
private static final MAX_GOSSIP_TOPICS : I
private static final GOSSIP_COOLDOWN : I
private static final GOSSIP_DECAY_INTERVAL : I
private static final HOW_FAR_AWAY_TO_TALK_TO_OTHER_VILLAGERS_ABOUT_GOLEMS : I
private static final HOW_MANY_VILLAGERS_NEED_TO_AGREE_TO_SPAWN_A_GOLEM : I
private static final TIME_SINCE_SLEEPING_FOR_GOLEM_SPAWNING : J
public static final SPEED_MODIFIER : F
private static final DEFAULT_XP : I
private static final DEFAULT_FOOD_LEVEL : B
private static final DEFAULT_LAST_RESTOCK : I
private static final DEFAULT_LAST_GOSSIP_DECAY : I
private static final DEFAULT_RESTOCKS_TODAY : I
private static final BABY_DIMENSIONS : Lnet/minecraft/world/entity/EntityDimensions;
private lastTradedPlayer : Lnet/minecraft/world/entity/player/Player;
private foodLevel : I
private final gossips : Lnet/minecraft/world/entity/ai/gossip/GossipContainer;
private lastGossipTime : J
private lastGossipDecayTime : J
private villagerXp : I
private lastRestockGameTime : J
private numberOfRestocksToday : I
private lastRestockCheckDay : J
private static final BRAIN_PROVIDER : Lnet/minecraft/world/entity/ai/Brain$Provider;
public static final POI_MEMORIES : Ljava/util/Map;
public <init>(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)V
public getBrain()Lnet/minecraft/world/entity/ai/Brain;
protected makeBrain(Lnet/minecraft/world/entity/ai/Brain$Packed;)Lnet/minecraft/world/entity/ai/Brain;
public refreshBrain(Lnet/minecraft/server/level/ServerLevel;)V
private registerBrainGoals(Lnet/minecraft/world/entity/ai/Brain;)V
protected ageBoundaryReached()V
public static createAttributes()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
protected customServerAiStep(Lnet/minecraft/server/level/ServerLevel;)V
public tick()V
public mobInteract(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/world/InteractionHand;)Lnet/minecraft/world/InteractionResult;
private setUnhappy()V
private startTrading(Lnet/minecraft/world/entity/player/Player;)V
public setTradingPlayer(Lnet/minecraft/world/entity/player/Player;)V
private resetSpecialPrices()V
public getDefaultDimensions(Lnet/minecraft/world/entity/Pose;)Lnet/minecraft/world/entity/EntityDimensions;
public canRestock()Z
public restock()V
private needsToRestock()Z
private allowedToRestock()Z
public shouldRestock(Lnet/minecraft/server/level/ServerLevel;)Z
private catchUpDemand()V
private updateDemand()V
private updateSpecialPrices(Lnet/minecraft/world/entity/player/Player;)V
protected defineSynchedData(Lnet/minecraft/network/syncher/SynchedEntityData$Builder;)V
public static createDefaultVillagerData()Lnet/minecraft/world/entity/npc/villager/VillagerData;
protected addAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueOutput;)V
protected readAdditionalSaveData(Lnet/minecraft/world/level/storage/ValueInput;)V
public postDataManipulated()V
public removeWhenFarAway(D)Z
protected getAmbientSound()Lnet/minecraft/sounds/SoundEvent;
protected getHurtSound(Lnet/minecraft/world/damagesource/DamageSource;)Lnet/minecraft/sounds/SoundEvent;
protected getDeathSound()Lnet/minecraft/sounds/SoundEvent;
public playWorkSound()V
public setVillagerData(Lnet/minecraft/world/entity/npc/villager/VillagerData;)V
public getVillagerDataFinalized()Z
public setVillagerDataFinalized(Z)V
public getVillagerData()Lnet/minecraft/world/entity/npc/villager/VillagerData;
protected rewardTradeXp(Lnet/minecraft/world/item/trading/MerchantOffer;)V
public setLastHurtByMob(Lnet/minecraft/world/entity/LivingEntity;)V
public die(Lnet/minecraft/world/damagesource/DamageSource;)V
private releaseAllPois()V
private tellWitnessesThatIWasMurdered(Lnet/minecraft/world/entity/Entity;)V
public releasePoi(Lnet/minecraft/world/entity/ai/memory/MemoryModuleType;)V
public canBreed()Z
private hungry()Z
private eatUntilFull()V
public getPlayerReputation(Lnet/minecraft/world/entity/player/Player;)I
private digestFood(I)V
public eatAndDigestFood()V
public setOffers(Lnet/minecraft/world/item/trading/MerchantOffers;)V
private shouldIncreaseLevel()Z
private increaseMerchantCareer(Lnet/minecraft/server/level/ServerLevel;)V
protected getTypeName()Lnet/minecraft/network/chat/Component;
public handleEntityEvent(B)V
public finalizeSpawn(Lnet/minecraft/world/level/ServerLevelAccessor;Lnet/minecraft/world/DifficultyInstance;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/SpawnGroupData;)Lnet/minecraft/world/entity/SpawnGroupData;
public getBreedOffspring(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/AgeableMob;)Lnet/minecraft/world/entity/npc/villager/Villager;
public thunderHit(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LightningBolt;)V
protected pickUpItem(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/item/ItemEntity;)V
public wantsToPickUp(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/item/ItemStack;)Z
public hasExcessFood()Z
public wantsMoreFood()Z
private countFoodPointsInInventory()I
public hasFarmSeeds()Z
protected updateTrades(Lnet/minecraft/server/level/ServerLevel;)V
public gossip(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/npc/villager/Villager;J)V
private maybeDecayGossip()V
public spawnGolemIfNeeded(Lnet/minecraft/server/level/ServerLevel;JI)V
public wantsToSpawnGolem(J)Z
public onReputationEventFrom(Lnet/minecraft/world/entity/ai/village/ReputationEventType;Lnet/minecraft/world/entity/Entity;)V
public getVillagerXp()I
public setVillagerXp(I)V
private resetNumberOfRestocks()V
public getGossips()Lnet/minecraft/world/entity/ai/gossip/GossipContainer;
public setGossips(Lnet/minecraft/world/entity/ai/gossip/GossipContainer;)V
public stopSleeping()V
private golemSpawnConditionsMet(J)Z
public get(Lnet/minecraft/core/component/DataComponentType;)Ljava/lang/Object;
protected applyImplicitComponents(Lnet/minecraft/core/component/DataComponentGetter;)V
protected applyImplicitComponent(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/Object;)Z
public synthetic getBreedOffspring(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/AgeableMob;)Lnet/minecraft/world/entity/AgeableMob;
private static synthetic lambda$golemSpawnConditionsMet$0(JLjava/lang/Long;)Z
private static synthetic lambda$spawnGolemIfNeeded$0(JLnet/minecraft/world/entity/npc/villager/Villager;)Z
private static synthetic lambda$hasFarmSeeds$0(Lnet/minecraft/world/item/ItemStack;)Z
private synthetic lambda$thunderHit$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/monster/Witch;)V
private static synthetic lambda$getPlayerReputation$0(Lnet/minecraft/world/entity/ai/gossip/GossipType;)Z
private synthetic lambda$releasePoi$0(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/world/entity/ai/memory/MemoryModuleType;Lnet/minecraft/core/GlobalPos;)V
private static synthetic lambda$tellWitnessesThatIWasMurdered$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/LivingEntity;)V
private static synthetic lambda$shouldRestock$0(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/core/Holder$Reference;)Ljava/lang/Integer;
private static synthetic lambda$static$4(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$static$3(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$static$2(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$static$1(Lnet/minecraft/world/entity/npc/villager/Villager;Lnet/minecraft/core/Holder;)Z
private static synthetic lambda$static$0(Lnet/minecraft/world/entity/npc/villager/Villager;)Ljava/util/List;
static <clinit>()V
```
