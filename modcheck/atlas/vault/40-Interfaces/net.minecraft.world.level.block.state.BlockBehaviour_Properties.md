---
type: "interface"
fqcn: "net.minecraft.world.level.block.state.BlockBehaviour$Properties"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.block.state.BlockBehaviour$Properties

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/block/v1/FabricBlock$FabricProperties`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `id` | `Lnet/minecraft/resources/ResourceKey;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | declared |

## Declared members (35 fields, 60 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private mapColor : Ljava/util/function/Function;
private hasCollision : Z
private soundType : Lnet/minecraft/world/level/block/SoundType;
private lightEmission : Ljava/util/function/ToIntFunction;
private explosionResistance : F
private destroyTime : F
private requiresCorrectToolForDrops : Z
private isRandomlyTicking : Z
private friction : F
private speedFactor : F
private jumpFactor : F
private bounceRestitution : F
private id : Lnet/minecraft/resources/ResourceKey;
private drops : Lnet/minecraft/resources/DependantName;
private descriptionId : Lnet/minecraft/resources/DependantName;
private canOcclude : Z
private isAir : Z
private ignitedByLava : Z
private liquid : Z
private forceSolidOff : Z
private forceSolidOn : Z
private pushReaction : Lnet/minecraft/world/level/material/PushReaction;
private spawnTerrainParticles : Z
private instrument : Lnet/minecraft/world/level/block/state/properties/NoteBlockInstrument;
private replaceable : Z
private isValidSpawn : Lnet/minecraft/world/level/block/state/BlockBehaviour$StateArgumentPredicate;
private isRedstoneConductor : Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;
private isSuffocating : Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;
private isViewBlocking : Lnet/minecraft/world/level/block/state/BlockBehaviour$StateArgumentPredicate;
private postProcess : Lnet/minecraft/world/level/block/state/BlockBehaviour$PostProcess;
private emissiveRendering : Ljava/util/function/Predicate;
private dynamicShape : Z
private requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private offsetFunction : Lnet/minecraft/world/level/block/state/BlockBehaviour$OffsetFunction;
private fallDistanceReduction : F
private <init>()V
public static of()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public static ofFullCopy(Lnet/minecraft/world/level/block/state/BlockBehaviour;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public static ofLegacyCopy(Lnet/minecraft/world/level/block/state/BlockBehaviour;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public mapColor(Lnet/minecraft/world/item/DyeColor;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public mapColor(Lnet/minecraft/world/level/material/MapColor;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public mapColor(Ljava/util/function/Function;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public noCollision()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public noOcclusion()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public friction(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public speedFactor(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public jumpFactor(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public bounceRestitution(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public fallDistanceReduction(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public sound(Lnet/minecraft/world/level/block/SoundType;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public lightLevel(Ljava/util/function/ToIntFunction;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public strength(FF)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public instabreak()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public strength(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public randomTicks()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public dynamicShape()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public noLootTable()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public overrideLootTable(Ljava/util/Optional;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
protected effectiveDrops()Ljava/util/Optional;
public ignitedByLava()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public liquid()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public forceSolidOn()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public forceSolidOff()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public pushReaction(Lnet/minecraft/world/level/material/PushReaction;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public air()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public isValidSpawn(Lnet/minecraft/world/level/block/state/BlockBehaviour$StateArgumentPredicate;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public isRedstoneConductor(Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public isSuffocating(Lnet/minecraft/world/level/block/state/BlockBehaviour$StatePredicate;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public isViewBlocking(Lnet/minecraft/world/level/block/state/BlockBehaviour$StateArgumentPredicate;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public postProcess(Lnet/minecraft/world/level/block/state/BlockBehaviour$PostProcess;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public emissiveRendering(Ljava/util/function/Predicate;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public requiresCorrectToolForDrops()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public destroyTime(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public explosionResistance(F)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public offsetType(Lnet/minecraft/world/level/block/state/BlockBehaviour$OffsetType;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public noTerrainParticles()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public requiredFeatures([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public instrument(Lnet/minecraft/world/level/block/state/properties/NoteBlockInstrument;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public replaceable()Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public setId(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
public overrideDescription(Ljava/lang/String;)Lnet/minecraft/world/level/block/state/BlockBehaviour$Properties;
protected effectiveDescriptionId()Ljava/lang/String;
private static synthetic lambda$offsetType$1(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$offsetType$0(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/world/phys/Vec3;
private static synthetic lambda$mapColor$1(Lnet/minecraft/world/level/material/MapColor;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/MapColor;
private static synthetic lambda$mapColor$0(Lnet/minecraft/world/item/DyeColor;Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/MapColor;
private static synthetic lambda$new$8(Lnet/minecraft/world/level/block/state/BlockState;)Z
private static synthetic lambda$new$7(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Lnet/minecraft/core/BlockPos;
private synthetic lambda$new$6(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/phys/AABB;)Z
private static synthetic lambda$new$5(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;)Z
private static synthetic lambda$new$4(Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/BlockGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/entity/EntityType;)Z
private static synthetic lambda$new$3(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$new$2(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private static synthetic lambda$new$1(Lnet/minecraft/world/level/block/state/BlockState;)I
private static synthetic lambda$new$0(Lnet/minecraft/world/level/block/state/BlockState;)Lnet/minecraft/world/level/material/MapColor;
```
