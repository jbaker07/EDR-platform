---
type: "system"
package: "net.minecraft.world.entity"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity

Analyst note: [[_authored/systems/net.minecraft.world.entity|Entities, living entities, mobs and AI]]

1157 classes (727 top-level) across 78 packages in the processed jar; 16 changed by Loom processing; 32 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]] -- calls:32, injects_into:13, reads:5, wraps:2, writes:1 -- by fabric-api-lookup-api-v1, fabric-content-registries-v0, fabric-data-attachment-api-v1, fabric-debug-api-v1, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-lifecycle-events-v1, fabric-networking-api-v1, fabric-object-builder-api-v1, fabric-particles-v1, fabric-permission-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityFluidInteraction|EntityFluidInteraction]] -- calls:5 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.EntitySelector|EntitySelector]] -- reads:1 -- by fabric-api-lookup-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntitySpawnReason|EntitySpawnReason]] -- reads:1 -- by fabric-api-lookup-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntitySpawnRequest|EntitySpawnRequest]] -- calls:1 -- by fabric-lifecycle-events-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]] -- calls:6, injects_into:3 -- by fabric-api-lookup-api-v1, fabric-biome-api-v1, fabric-client-gametest-api-v1, fabric-data-generation-api-v1, fabric-lifecycle-events-v1, fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityType_Builder|EntityType$Builder]] -- calls:3, injects_into:1, wraps:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.EntityTypes|EntityTypes]] -- reads:1 -- by fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]] -- calls:42, injects_into:28, reads:1, wraps:10 -- by fabric-block-api-v1, fabric-content-registries-v0, fabric-entity-events-v1, fabric-item-api-v1, fabric-lifecycle-events-v1, fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]] -- injects_into:2 -- by fabric-debug-api-v1, fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.entity.MobCategory|MobCategory]] -- calls:3, reads:1 -- by fabric-biome-api-v1
- [[40-Interfaces/net.minecraft.world.entity.MoverType|MoverType]] -- reads:3 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.SpawnPlacements|SpawnPlacements]] -- calls:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.Attribute|Attribute]] -- calls:1 -- by fabric-data-generation-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.AttributeSupplier|AttributeSupplier]] -- calls:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.AttributeSupplier_Builder|AttributeSupplier$Builder]] -- calls:2 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.Attributes|Attributes]] -- reads:3 -- by fabric-content-registries-v0, fabric-rendering-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.attributes.DefaultAttributes|DefaultAttributes]] -- injects_into:1, reads:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.ai.behavior.GiveGiftToHero|GiveGiftToHero]] -- injects_into:1, reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.ai.behavior.Swim|Swim]] -- injects_into:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.ai.goal.FloatGoal|FloatGoal]] -- injects_into:1, reads:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.ai.village.poi.PoiTypes|PoiTypes]] -- calls:1 -- by fabric-object-builder-api-v1
- [[40-Interfaces/net.minecraft.world.entity.monster.Monster|Monster]] -- calls:2 -- by fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.entity.monster.breeze.Breeze|Breeze]] -- injects_into:2 -- by fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.entity.monster.warden.Warden|Warden]] -- injects_into:1 -- by fabric-particles-v1
- [[40-Interfaces/net.minecraft.world.entity.npc.villager.Villager|Villager]] -- wraps:1 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.player.Abilities|Abilities]] -- reads:1 -- by fabric-events-interaction-v0
- [[40-Interfaces/net.minecraft.world.entity.player.Inventory|Inventory]] -- calls:5, reads:4 -- by fabric-events-interaction-v0, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]] -- calls:30, injects_into:3, writes:1 -- by fabric-content-registries-v0, fabric-entity-events-v1, fabric-events-interaction-v0, fabric-menu-api-v1, fabric-permission-api-v1, fabric-rendering-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.world.entity.player.Player_BedSleepingProblem|Player$BedSleepingProblem]] -- reads:1 -- by fabric-entity-events-v1
- [[40-Interfaces/net.minecraft.world.entity.vehicle.boat.AbstractBoat|AbstractBoat]] -- wraps:2 -- by fabric-content-registries-v0
- [[40-Interfaces/net.minecraft.world.entity.vehicle.minecart.AbstractMinecart|AbstractMinecart]] -- calls:1 -- by fabric-object-builder-api-v1

## Declared inventory

### `net.minecraft.world.entity` (85 top-level)

`AbstractInterpolationHandler`, `AgeableMob`, `AnimationState`, `AreaEffectCloud`, `Attackable`, `Avatar`, `Bucketable`, `ContainerUser`, `ConversionParams`, `ConversionTracker`, `ConversionType`, `Crackiness`, `Display`, `DropChances`, `ElytraAnimationState`, [[40-Interfaces/net.minecraft.world.entity.Entity|Entity]], `EntityAttachment`, `EntityAttachments`, `EntityDimensions`, `EntityEquipment`, `EntityEvent`, [[40-Interfaces/net.minecraft.world.entity.EntityFluidInteraction|EntityFluidInteraction]], `EntityProcessor`, `EntityReference`, [[40-Interfaces/net.minecraft.world.entity.EntitySelector|EntitySelector]], [[40-Interfaces/net.minecraft.world.entity.EntitySpawnReason|EntitySpawnReason]], [[40-Interfaces/net.minecraft.world.entity.EntitySpawnRequest|EntitySpawnRequest]], [[40-Interfaces/net.minecraft.world.entity.EntityType|EntityType]], `EntityTypeIds`, [[40-Interfaces/net.minecraft.world.entity.EntityTypes|EntityTypes]], `EquipmentSlot`, `EquipmentSlotGroup`, `EquipmentTable`, `EquipmentUser`, `ExperienceOrb`, `HasCustomInventoryScreen`, `HumanoidArm`, `InsideBlockEffectApplier`, `InsideBlockEffectType`, `Interaction`, `InterpolationHandler`, `InterpolationTracker`, `ItemBasedSteering`, `ItemOwner`, `ItemSteerable`, `Leashable`, `LightningBolt`, `LinearInterpolationHandler`, [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]], `Marker`, [[40-Interfaces/net.minecraft.world.entity.Mob|Mob]], [[40-Interfaces/net.minecraft.world.entity.MobCategory|MobCategory]], `MoveSimulationType`, [[40-Interfaces/net.minecraft.world.entity.MoverType|MoverType]], `NeutralMob`, `OminousItemSpawner`, `OwnableEntity`, `PathfinderMob`, `PlayerRideable`, `PlayerRideableJumping`, `PortalProcessor`, `Pose`, `PositionMoveRotation`, `PositionPath`, `PositionStep`, `PostSpawnProcessor`, `Relative`, `ReputationEventHandler`, `Shearable`, `SlotAccess`, `SlotProvider`, `SpawnGroupData`, `SpawnPlacementType`, `SpawnPlacementTypes`, [[40-Interfaces/net.minecraft.world.entity.SpawnPlacements|SpawnPlacements]], `SteppedInterpolationHandler`, `SteppedInterpolationTracker`, `SulfurCubeArchetype`, `SulfurCubeArchetypes`, `TamableAnimal`, `Targeting`, `TraceableEntity`, `UpdateInterval`, `WalkAnimationState`, `package-info`

### `net.minecraft.world.entity.ai` (3 top-level)

`ActivityData`, `Brain`, `package-info`

### `net.minecraft.world.entity.ai.attributes` (9 top-level)

[[40-Interfaces/net.minecraft.world.entity.ai.attributes.Attribute|Attribute]], `AttributeInstance`, `AttributeMap`, `AttributeModifier`, [[40-Interfaces/net.minecraft.world.entity.ai.attributes.AttributeSupplier|AttributeSupplier]], [[40-Interfaces/net.minecraft.world.entity.ai.attributes.Attributes|Attributes]], [[40-Interfaces/net.minecraft.world.entity.ai.attributes.DefaultAttributes|DefaultAttributes]], `RangedAttribute`, `package-info`

### `net.minecraft.world.entity.ai.behavior` (104 top-level)

`AcquirePoi`, `AnimalMakeLove`, `AnimalPanic`, `AssignProfessionFromJobSite`, `BabyFollowAdult`, `BackUpIfTooClose`, `BecomePassiveIfMemoryPresent`, `Behavior`, `BehaviorControl`, `BehaviorUtils`, `BlockPosTracker`, `CelebrateVillagersSurvivedRaid`, `ChargeAttack`, `CopyMemoryWithExpiry`, `CountDownCooldownTicks`, `Croak`, `CrossbowAttack`, `DismountOrSkipMounting`, `DoNothing`, `EntityTracker`, `EraseMemoryIf`, `FollowTemptation`, `GateBehavior`, [[40-Interfaces/net.minecraft.world.entity.ai.behavior.GiveGiftToHero|GiveGiftToHero]], `GoAndGiveItemsToTarget`, `GoToClosestVillage`, `GoToPotentialJobSite`, `GoToTargetLocation`, `GoToWantedItem`, `HarvestFarmland`, `InsideBrownianWalk`, `InteractWith`, `InteractWithDoor`, `JumpOnBed`, `LocateHidingPlace`, `LongJumpMidJump`, `LongJumpToPreferredBlock`, `LongJumpToRandomPos`, `LongJumpUtil`, `LookAndFollowTradingPlayerSink`, `LookAtTargetSink`, `MeleeAttack`, `Mount`, `MoveToSkySeeingSpot`, `MoveToTargetSink`, `OneShot`, `PlayTagWithOtherKids`, `PoiCompetitorScan`, `PositionTracker`, `PrepareRamNearestTarget`, `RamTarget`, `RandomLookAround`, `RandomStroll`, `ReactToBell`, `ResetProfession`, `ResetRaidStatus`, `RingBell`, `RunOne`, `SetClosestHomeAsWalkTarget`, `SetEntityLookTarget`, `SetEntityLookTargetSometimes`, `SetHiddenState`, `SetLookAndInteract`, `SetRaidStatus`, `SetWalkTargetAwayFrom`, `SetWalkTargetFromAttackTargetIfTargetOutOfReach`, `SetWalkTargetFromBlockMemory`, `SetWalkTargetFromLookTarget`, `ShowTradesToPlayer`, `ShufflingList`, `SleepInBed`, `SocializeAtBell`, `SpearApproach`, `SpearAttack`, `SpearRetreat`, `StartAttacking`, `StartCelebratingIfTargetDead`, `StayCloseToTarget`, `StopAttackingIfTargetInvalid`, `StopBeingAngryIfTargetDead`, `StrollAroundPoi`, `StrollToPoi`, `StrollToPoiList`, [[40-Interfaces/net.minecraft.world.entity.ai.behavior.Swim|Swim]], `TradeWithVillager`, `TransportItemsBetweenContainers`, `TriggerGate`, `TryFindLand`, `TryFindLandNearLiquid`, `TryFindLiquid`, `TryLaySpawnOnFluidNearLand`, `UpdateActivityFromSchedule`, `UseBonemeal`, `ValidateNearbyPoi`, `VillageBoundRandomStroll`, `VillagerCalmDown`, `VillagerGoalPackages`, `VillagerMakeLove`, `VillagerPanicTrigger`, `WakeUp`, `WorkAtComposter`, `WorkAtPoi`, `YieldJobSite`, `package-info`

### `net.minecraft.world.entity.ai.behavior.declarative` (5 top-level)

`BehaviorBuilder`, `MemoryAccessor`, `MemoryCondition`, `Trigger`, `package-info`

### `net.minecraft.world.entity.ai.behavior.warden` (10 top-level)

`Digging`, `Emerging`, `ForceUnmount`, `Roar`, `SetRoarTarget`, `SetWardenLookTarget`, `Sniffing`, `SonicBoom`, `TryToSniff`, `package-info`

### `net.minecraft.world.entity.ai.control` (9 top-level)

`BodyRotationControl`, `Control`, `FlyingMoveControl`, `JumpControl`, `LookControl`, `MoveControl`, `SmoothSwimmingLookControl`, `SmoothSwimmingMoveControl`, `package-info`

### `net.minecraft.world.entity.ai.goal` (62 top-level)

`AvoidEntityGoal`, `BegGoal`, `BreakDoorGoal`, `BreathAirGoal`, `BreedGoal`, `CatLieOnBlockGoal`, `CatSitOnBlockGoal`, `ClimbOnTopOfPowderSnowGoal`, `DolphinJumpGoal`, `DoorInteractGoal`, `EatBlockGoal`, `FleeSunGoal`, [[40-Interfaces/net.minecraft.world.entity.ai.goal.FloatGoal|FloatGoal]], `FollowFlockLeaderGoal`, `FollowMobGoal`, `FollowOwnerGoal`, `FollowParentGoal`, `FollowPlayerRiddenEntityGoal`, `Goal`, `GoalSelector`, `GolemRandomStrollInVillageGoal`, `InteractGoal`, `JumpGoal`, `LandOnOwnersShoulderGoal`, `LeapAtTargetGoal`, `LlamaFollowCaravanGoal`, `LookAtPlayerGoal`, `LookAtTradingPlayerGoal`, `MeleeAttackGoal`, `MoveBackToVillageGoal`, `MoveThroughVillageGoal`, `MoveToBlockGoal`, `MoveTowardsRestrictionGoal`, `MoveTowardsTargetGoal`, `OcelotAttackGoal`, `OfferFlowerGoal`, `OpenDoorGoal`, `PanicGoal`, `PathfindToRaidGoal`, `RandomLookAroundGoal`, `RandomStandGoal`, `RandomStrollGoal`, `RandomSwimmingGoal`, `RangedAttackGoal`, `RangedBowAttackGoal`, `RangedCrossbowAttackGoal`, `RemoveBlockGoal`, `RestrictSunGoal`, `RunAroundLikeCrazyGoal`, `SitWhenOrderedToGoal`, `SpearUseGoal`, `StrollThroughVillageGoal`, `SwellGoal`, `TemptGoal`, `TradeWithPlayerGoal`, `TryFindLiquidGoal`, `UseItemGoal`, `WaterAvoidingRandomFlyingGoal`, `WaterAvoidingRandomStrollGoal`, `WrappedGoal`, `ZombieAttackGoal`, `package-info`

### `net.minecraft.world.entity.ai.goal.target` (11 top-level)

`DefendVillageTargetGoal`, `HurtByTargetGoal`, `NearestAttackableTargetGoal`, `NearestAttackableWitchTargetGoal`, `NearestHealableRaiderTargetGoal`, `NonTameRandomTargetGoal`, `OwnerHurtByTargetGoal`, `OwnerHurtTargetGoal`, `ResetUniversalAngerTargetGoal`, `TargetGoal`, `package-info`

### `net.minecraft.world.entity.ai.gossip` (3 top-level)

`GossipContainer`, `GossipType`, `package-info`

### `net.minecraft.world.entity.ai.memory` (8 top-level)

`ExpirableValue`, `MemoryMap`, `MemoryModuleType`, `MemorySlot`, `MemoryStatus`, `NearestVisibleLivingEntities`, `WalkTarget`, `package-info`

### `net.minecraft.world.entity.ai.navigation` (7 top-level)

`AmphibiousPathNavigation`, `FlyingPathNavigation`, `GroundPathNavigation`, `PathNavigation`, `WallClimberNavigation`, `WaterBoundPathNavigation`, `package-info`

### `net.minecraft.world.entity.ai.sensing` (27 top-level)

`AdultSensor`, `AdultSensorAnyType`, `AxolotlAttackablesSensor`, `BreezeAttackEntitySensor`, `DummySensor`, `FrogAttackablesSensor`, `GolemSensor`, `HoglinSpecificSensor`, `HurtBySensor`, `IsInWaterSensor`, `MobSensor`, `NearestBedSensor`, `NearestItemSensor`, `NearestLivingEntitySensor`, `NearestVisibleLivingEntitySensor`, `PiglinBruteSpecificSensor`, `PiglinSpecificSensor`, `PlayerSensor`, `SecondaryPoiSensor`, `Sensing`, `Sensor`, `SensorType`, `TemptingSensor`, `VillagerBabiesSensor`, `VillagerHostilesSensor`, `WardenEntitySensor`, `package-info`

### `net.minecraft.world.entity.ai.targeting` (2 top-level)

`TargetingConditions`, `package-info`

### `net.minecraft.world.entity.ai.util` (8 top-level)

`AirAndWaterRandomPos`, `AirRandomPos`, `DefaultRandomPos`, `GoalUtils`, `HoverRandomPos`, `LandRandomPos`, `RandomPos`, `package-info`

### `net.minecraft.world.entity.ai.village` (3 top-level)

`ReputationEventType`, `VillageSiege`, `package-info`

### `net.minecraft.world.entity.ai.village.poi` (6 top-level)

`PoiManager`, `PoiRecord`, `PoiSection`, `PoiType`, [[40-Interfaces/net.minecraft.world.entity.ai.village.poi.PoiTypes|PoiTypes]], `package-info`

### `net.minecraft.world.entity.ambient` (3 top-level)

`AmbientCreature`, `Bat`, `package-info`

### `net.minecraft.world.entity.animal` (4 top-level)

`AgeableWaterCreature`, `Animal`, `TemperatureVariants`, `package-info`

### `net.minecraft.world.entity.animal.allay` (3 top-level)

`Allay`, `AllayAi`, `package-info`

### `net.minecraft.world.entity.animal.armadillo` (3 top-level)

`Armadillo`, `ArmadilloAi`, `package-info`

### `net.minecraft.world.entity.animal.axolotl` (5 top-level)

`Axolotl`, `AxolotlAi`, `PlayDead`, `ValidatePlayDead`, `package-info`

### `net.minecraft.world.entity.animal.bee` (2 top-level)

`Bee`, `package-info`

### `net.minecraft.world.entity.animal.camel` (4 top-level)

`Camel`, `CamelAi`, `CamelHusk`, `package-info`

### `net.minecraft.world.entity.animal.chicken` (6 top-level)

`Chicken`, `ChickenSoundVariant`, `ChickenSoundVariants`, `ChickenVariant`, `ChickenVariants`, `package-info`

### `net.minecraft.world.entity.animal.cow` (8 top-level)

`AbstractCow`, `Cow`, `CowSoundVariant`, `CowSoundVariants`, `CowVariant`, `CowVariants`, `MushroomCow`, `package-info`

### `net.minecraft.world.entity.animal.dolphin` (2 top-level)

`Dolphin`, `package-info`

### `net.minecraft.world.entity.animal.equine` (13 top-level)

`AbstractChestedHorse`, `AbstractHorse`, `Donkey`, `Horse`, `Llama`, `Markings`, `Mule`, `SkeletonHorse`, `SkeletonTrapGoal`, `TraderLlama`, `Variant`, `ZombieHorse`, `package-info`

### `net.minecraft.world.entity.animal.feline` (7 top-level)

`Cat`, `CatSoundVariant`, `CatSoundVariants`, `CatVariant`, `CatVariants`, `Ocelot`, `package-info`

### `net.minecraft.world.entity.animal.fish` (8 top-level)

`AbstractFish`, `AbstractSchoolingFish`, `Cod`, `Pufferfish`, `Salmon`, `TropicalFish`, `WaterAnimal`, `package-info`

### `net.minecraft.world.entity.animal.fox` (2 top-level)

`Fox`, `package-info`

### `net.minecraft.world.entity.animal.frog` (8 top-level)

`Frog`, `FrogAi`, `FrogVariant`, `FrogVariants`, `ShootTongue`, `Tadpole`, `TadpoleAi`, `package-info`

### `net.minecraft.world.entity.animal.goat` (3 top-level)

`Goat`, `GoatAi`, `package-info`

### `net.minecraft.world.entity.animal.golem` (9 top-level)

`AbstractGolem`, `CopperGolem`, `CopperGolemAi`, `CopperGolemOxidationLevel`, `CopperGolemOxidationLevels`, `CopperGolemState`, `IronGolem`, `SnowGolem`, `package-info`

### `net.minecraft.world.entity.animal.happyghast` (3 top-level)

`HappyGhast`, `HappyGhastAi`, `package-info`

### `net.minecraft.world.entity.animal.nautilus` (8 top-level)

`AbstractNautilus`, `Nautilus`, `NautilusAi`, `ZombieNautilus`, `ZombieNautilusAi`, `ZombieNautilusVariant`, `ZombieNautilusVariants`, `package-info`

### `net.minecraft.world.entity.animal.panda` (2 top-level)

`Panda`, `package-info`

### `net.minecraft.world.entity.animal.parrot` (3 top-level)

`Parrot`, `ShoulderRidingEntity`, `package-info`

### `net.minecraft.world.entity.animal.pig` (6 top-level)

`Pig`, `PigSoundVariant`, `PigSoundVariants`, `PigVariant`, `PigVariants`, `package-info`

### `net.minecraft.world.entity.animal.polarbear` (2 top-level)

`PolarBear`, `package-info`

### `net.minecraft.world.entity.animal.rabbit` (2 top-level)

`Rabbit`, `package-info`

### `net.minecraft.world.entity.animal.sheep` (3 top-level)

`Sheep`, `SheepColorSpawnRules`, `package-info`

### `net.minecraft.world.entity.animal.sniffer` (3 top-level)

`Sniffer`, `SnifferAi`, `package-info`

### `net.minecraft.world.entity.animal.squid` (3 top-level)

`GlowSquid`, `Squid`, `package-info`

### `net.minecraft.world.entity.animal.turtle` (2 top-level)

`Turtle`, `package-info`

### `net.minecraft.world.entity.animal.wolf` (6 top-level)

`Wolf`, `WolfSoundVariant`, `WolfSoundVariants`, `WolfVariant`, `WolfVariants`, `package-info`

### `net.minecraft.world.entity.boss.enderdragon` (5 top-level)

`DragonFlightHistory`, `EndCrystal`, `EnderDragon`, `EnderDragonPart`, `package-info`

### `net.minecraft.world.entity.boss.enderdragon.phases` (17 top-level)

`AbstractDragonPhaseInstance`, `AbstractDragonSittingPhase`, `DragonChargePlayerPhase`, `DragonDeathPhase`, `DragonHoldingPatternPhase`, `DragonHoverPhase`, `DragonLandingApproachPhase`, `DragonLandingPhase`, `DragonPhaseInstance`, `DragonSittingAttackingPhase`, `DragonSittingFlamingPhase`, `DragonSittingScanningPhase`, `DragonStrafePlayerPhase`, `DragonTakeoffPhase`, `EnderDragonPhase`, `EnderDragonPhaseManager`, `package-info`

### `net.minecraft.world.entity.boss.wither` (2 top-level)

`WitherBoss`, `package-info`

### `net.minecraft.world.entity.decoration` (9 top-level)

`ArmorStand`, `BlockAttachedEntity`, `Cushion`, `GlowItemFrame`, `HangingEntity`, `ItemFrame`, `LeashFenceKnotEntity`, `Mannequin`, `package-info`

### `net.minecraft.world.entity.decoration.painting` (4 top-level)

`Painting`, `PaintingVariant`, `PaintingVariants`, `package-info`

### `net.minecraft.world.entity.item` (4 top-level)

`FallingBlockEntity`, `ItemEntity`, `PrimedTnt`, `package-info`

### `net.minecraft.world.entity.monster` (22 top-level)

`Blaze`, `Creeper`, `CrossbowAttackMob`, `ElderGuardian`, `Enderman`, `Endermite`, `Enemy`, `Ghast`, `Giant`, `Guardian`, [[40-Interfaces/net.minecraft.world.entity.monster.Monster|Monster]], `PatrollingMonster`, `Phantom`, `RangedAttackMob`, `Ravager`, `Shulker`, `Silverfish`, `Strider`, `Vex`, `Witch`, `Zoglin`, `package-info`

### `net.minecraft.world.entity.monster.breeze` (8 top-level)

[[40-Interfaces/net.minecraft.world.entity.monster.breeze.Breeze|Breeze]], `BreezeAi`, `BreezeUtil`, `LongJump`, `Shoot`, `ShootWhenStuck`, `Slide`, `package-info`

### `net.minecraft.world.entity.monster.creaking` (3 top-level)

`Creaking`, `CreakingAi`, `package-info`

### `net.minecraft.world.entity.monster.cubemob` (5 top-level)

`AbstractCubeMob`, `MagmaCube`, `Slime`, `SulfurCube`, `package-info`

### `net.minecraft.world.entity.monster.hoglin` (4 top-level)

`Hoglin`, `HoglinAi`, `HoglinBase`, `package-info`

### `net.minecraft.world.entity.monster.illager` (7 top-level)

`AbstractIllager`, `Evoker`, `Illusioner`, `Pillager`, `SpellcasterIllager`, `Vindicator`, `package-info`

### `net.minecraft.world.entity.monster.piglin` (13 top-level)

`AbstractPiglin`, `Piglin`, `PiglinAi`, `PiglinArmPose`, `PiglinBrute`, `PiglinBruteAi`, `RememberIfHoglinWasKilled`, `StartAdmiringItemIfSeen`, `StartHuntingHoglin`, `StopAdmiringIfItemTooFarAway`, `StopAdmiringIfTiredOfTryingToReachItem`, `StopHoldingItemIfNoLongerAdmiring`, `package-info`

### `net.minecraft.world.entity.monster.skeleton` (7 top-level)

`AbstractSkeleton`, `Bogged`, `Parched`, `Skeleton`, `Stray`, `WitherSkeleton`, `package-info`

### `net.minecraft.world.entity.monster.spider` (3 top-level)

`CaveSpider`, `Spider`, `package-info`

### `net.minecraft.world.entity.monster.warden` (6 top-level)

`AngerLevel`, `AngerManagement`, [[40-Interfaces/net.minecraft.world.entity.monster.warden.Warden|Warden]], `WardenAi`, `WardenSpawnTracker`, `package-info`

### `net.minecraft.world.entity.monster.zombie` (6 top-level)

`Drowned`, `Husk`, `Zombie`, `ZombieVillager`, `ZombifiedPiglin`, `package-info`

### `net.minecraft.world.entity.npc` (5 top-level)

`CatSpawner`, `ClientSideMerchant`, `InventoryCarrier`, `Npc`, `package-info`

### `net.minecraft.world.entity.npc.villager` (7 top-level)

`AbstractVillager`, [[40-Interfaces/net.minecraft.world.entity.npc.villager.Villager|Villager]], `VillagerData`, `VillagerDataHolder`, `VillagerProfession`, `VillagerType`, `package-info`

### `net.minecraft.world.entity.npc.wanderingtrader` (3 top-level)

`WanderingTrader`, `WanderingTraderSpawner`, `package-info`

### `net.minecraft.world.entity.player` (14 top-level)

[[40-Interfaces/net.minecraft.world.entity.player.Abilities|Abilities]], `ChatVisiblity`, `Input`, [[40-Interfaces/net.minecraft.world.entity.player.Inventory|Inventory]], [[40-Interfaces/net.minecraft.world.entity.player.Player|Player]], `PlayerEquipment`, `PlayerModelPart`, `PlayerModelType`, `PlayerSkin`, `ProfileKeyPair`, `ProfilePublicKey`, `StackedContents`, `StackedItemContents`, `package-info`

### `net.minecraft.world.entity.projectile` (12 top-level)

`EvokerFangs`, `EyeOfEnder`, `FireworkRocketEntity`, `FishingHook`, `ItemSupplier`, `LlamaSpit`, `Projectile`, `ProjectileDeflection`, `ProjectileUtil`, `ShulkerBullet`, `ThrowableProjectile`, `package-info`

### `net.minecraft.world.entity.projectile.arrow` (5 top-level)

`AbstractArrow`, `Arrow`, `SpectralArrow`, `ThrownTrident`, `package-info`

### `net.minecraft.world.entity.projectile.hurtingprojectile` (7 top-level)

`AbstractHurtingProjectile`, `DragonFireball`, `Fireball`, `LargeFireball`, `SmallFireball`, `WitherSkull`, `package-info`

### `net.minecraft.world.entity.projectile.hurtingprojectile.windcharge` (4 top-level)

`AbstractWindCharge`, `BreezeWindCharge`, `WindCharge`, `package-info`

### `net.minecraft.world.entity.projectile.throwableitemprojectile` (9 top-level)

`AbstractThrownPotion`, `Snowball`, `ThrowableItemProjectile`, `ThrownEgg`, `ThrownEnderpearl`, `ThrownExperienceBottle`, `ThrownLingeringPotion`, `ThrownSplashPotion`, `package-info`

### `net.minecraft.world.entity.raid` (4 top-level)

`Raid`, `Raider`, `Raids`, `package-info`

### `net.minecraft.world.entity.schedule` (2 top-level)

`Activity`, `package-info`

### `net.minecraft.world.entity.variant` (11 top-level)

`BiomeCheck`, `ModelAndTexture`, `MoonBrightnessCheck`, `PriorityProvider`, `SpawnCondition`, `SpawnConditions`, `SpawnContext`, `SpawnPrioritySelectors`, `StructureCheck`, `VariantUtils`, `package-info`

### `net.minecraft.world.entity.vehicle` (4 top-level)

`ContainerEntity`, `DismountHelper`, `VehicleEntity`, `package-info`

### `net.minecraft.world.entity.vehicle.boat` (7 top-level)

[[40-Interfaces/net.minecraft.world.entity.vehicle.boat.AbstractBoat|AbstractBoat]], `AbstractChestBoat`, `Boat`, `ChestBoat`, `ChestRaft`, `Raft`, `package-info`

### `net.minecraft.world.entity.vehicle.minecart` (13 top-level)

[[40-Interfaces/net.minecraft.world.entity.vehicle.minecart.AbstractMinecart|AbstractMinecart]], `AbstractMinecartContainer`, `Minecart`, `MinecartBehavior`, `MinecartChest`, `MinecartCommandBlock`, `MinecartFurnace`, `MinecartHopper`, `MinecartSpawner`, `MinecartTNT`, `NewMinecartBehavior`, `OldMinecartBehavior`, `package-info`

