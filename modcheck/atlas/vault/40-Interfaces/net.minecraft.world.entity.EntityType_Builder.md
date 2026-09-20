---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityType$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityType$Builder

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/object/builder/v1/entity/FabricEntityType$Builder`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/Ent` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | declared |
| calls | `of` | `(Lnet/minecraft/world/entity/EntityType$EntityFactory;Lnet/minecraft/w` | exact | invokestatic@2 in `FabricEntityTypeImpl$Builder.createLiving` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `of` | `(Lnet/minecraft/world/entity/EntityType$EntityFactory;Lnet/minecraft/w` | exact | invokestatic@2 in `FabricEntityTypeImpl$Builder.createMob` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| injects_into | `build` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/Ent` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| wraps | `build` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/Ent` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (17 fields, 29 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final factory : Lnet/minecraft/world/entity/EntityType$EntityFactory;
private final category : Lnet/minecraft/world/entity/MobCategory;
private immuneTo : Lnet/minecraft/tags/TagKey;
private serialize : Z
private summon : Z
private fireImmune : Z
private canSpawnFarFromPlayer : Z
private clientTrackingRange : I
private updateInterval : I
private dimensions : Lnet/minecraft/world/entity/EntityDimensions;
private spawnDimensionsScale : F
private attachments : Lnet/minecraft/world/entity/EntityAttachments$Builder;
private requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private lootTable : Lnet/minecraft/resources/DependantName;
private final descriptionId : Lnet/minecraft/resources/DependantName;
private allowedInPeaceful : Z
private trackDeltas : Z
private <init>(Lnet/minecraft/world/entity/EntityType$EntityFactory;Lnet/minecraft/world/entity/MobCategory;)V
public static of(Lnet/minecraft/world/entity/EntityType$EntityFactory;Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/world/entity/EntityType$Builder;
public static createNothing(Lnet/minecraft/world/entity/MobCategory;)Lnet/minecraft/world/entity/EntityType$Builder;
public sized(FF)Lnet/minecraft/world/entity/EntityType$Builder;
public spawnDimensionsScale(F)Lnet/minecraft/world/entity/EntityType$Builder;
public eyeHeight(F)Lnet/minecraft/world/entity/EntityType$Builder;
public passengerAttachments([F)Lnet/minecraft/world/entity/EntityType$Builder;
public passengerAttachments([Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/entity/EntityType$Builder;
public vehicleAttachment(Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/entity/EntityType$Builder;
public ridingOffset(F)Lnet/minecraft/world/entity/EntityType$Builder;
public nameTagOffset(F)Lnet/minecraft/world/entity/EntityType$Builder;
public attach(Lnet/minecraft/world/entity/EntityAttachment;FFF)Lnet/minecraft/world/entity/EntityType$Builder;
public attach(Lnet/minecraft/world/entity/EntityAttachment;Lnet/minecraft/world/phys/Vec3;)Lnet/minecraft/world/entity/EntityType$Builder;
public noSummon()Lnet/minecraft/world/entity/EntityType$Builder;
public noSave()Lnet/minecraft/world/entity/EntityType$Builder;
public fireImmune()Lnet/minecraft/world/entity/EntityType$Builder;
public immuneTo(Lnet/minecraft/tags/TagKey;)Lnet/minecraft/world/entity/EntityType$Builder;
public canSpawnFarFromPlayer()Lnet/minecraft/world/entity/EntityType$Builder;
public clientTrackingRange(I)Lnet/minecraft/world/entity/EntityType$Builder;
public updateInterval(I)Lnet/minecraft/world/entity/EntityType$Builder;
public noUpdateInterval()Lnet/minecraft/world/entity/EntityType$Builder;
public requiredFeatures([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/entity/EntityType$Builder;
public noLootTable()Lnet/minecraft/world/entity/EntityType$Builder;
public notInPeaceful()Lnet/minecraft/world/entity/EntityType$Builder;
public dontTrackDeltas()Lnet/minecraft/world/entity/EntityType$Builder;
public build(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/world/entity/EntityType;
private static synthetic lambda$createNothing$0(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/level/Level;)Lnet/minecraft/world/entity/Entity;
private static synthetic lambda$new$1(Lnet/minecraft/resources/ResourceKey;)Ljava/lang/String;
private static synthetic lambda$new$0(Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
```
