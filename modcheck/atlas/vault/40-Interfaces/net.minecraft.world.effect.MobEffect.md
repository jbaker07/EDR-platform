---
type: "interface"
fqcn: "net.minecraft.world.effect.MobEffect"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.effect.MobEffect

System: [[20-Systems/net.minecraft.world.effect|net.minecraft.world.effect]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/world/flag/FeatureElement`, `net/fabricmc/fabric/api/entity/event/v1/effect/FabricMobEffect`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@2 in `FabricLanguageProvider$TranslationBuilder.add` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `onEffectAdded` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | inherited_exact | invokevirtual@14 in `ServerMobEffectEvents.lambda$static$12` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `onEffectRemoved` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | inherited_exact | invokevirtual@14 in `ServerMobEffectEvents.lambda$static$14` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `onEffectStarted` | `(Lnet/minecraft/world/effect/MobEffectInstance;Lnet/minecraft/world/en` | inherited_exact | invokevirtual@14 in `ServerMobEffectEvents.lambda$static$13` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (13 fields, 34 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private static final AMBIENT_ALPHA : I
private final attributeModifiers : Ljava/util/Map;
private final category : Lnet/minecraft/world/effect/MobEffectCategory;
private final color : I
private final particleFactory : Ljava/util/function/Function;
private descriptionId : Ljava/lang/String;
private blendInDurationTicks : I
private blendOutDurationTicks : I
private blendOutAdvanceTicks : I
private soundOnAdded : Ljava/util/Optional;
private requiredFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
protected <init>(Lnet/minecraft/world/effect/MobEffectCategory;I)V
protected <init>(Lnet/minecraft/world/effect/MobEffectCategory;ILnet/minecraft/core/particles/ParticleOptions;)V
public getBlendInDurationTicks()I
public getBlendOutDurationTicks()I
public getBlendOutAdvanceTicks()I
public applyEffectTick(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;I)Z
public applyInstantaneousEffect(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/LivingEntity;ID)V
public shouldApplyEffectTickThisTick(II)Z
public onEffectStarted(Lnet/minecraft/world/entity/LivingEntity;I)V
public onEffectAdded(Lnet/minecraft/world/entity/LivingEntity;I)V
public onMobRemoved(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;ILnet/minecraft/world/entity/Entity$RemovalReason;)V
public onMobHurt(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;ILnet/minecraft/world/damagesource/DamageSource;F)V
public isInstantaneous()Z
protected getOrCreateDescriptionId()Ljava/lang/String;
public getDescriptionId()Ljava/lang/String;
public getDisplayName()Lnet/minecraft/network/chat/Component;
public getCategory()Lnet/minecraft/world/effect/MobEffectCategory;
public getColor()I
public addAttributeModifier(Lnet/minecraft/core/Holder;Lnet/minecraft/resources/Identifier;DLnet/minecraft/world/entity/ai/attributes/AttributeModifier$Operation;)Lnet/minecraft/world/effect/MobEffect;
public setBlendDuration(I)Lnet/minecraft/world/effect/MobEffect;
public setBlendDuration(III)Lnet/minecraft/world/effect/MobEffect;
public createModifiers(ILjava/util/function/BiConsumer;)V
public removeAttributeModifiers(Lnet/minecraft/world/entity/ai/attributes/AttributeMap;)V
public addAttributeModifiers(Lnet/minecraft/world/entity/ai/attributes/AttributeMap;I)V
public isBeneficial()Z
public createParticleOptions(Lnet/minecraft/world/effect/MobEffectInstance;)Lnet/minecraft/core/particles/ParticleOptions;
public withSoundOnAdded(Lnet/minecraft/sounds/SoundEvent;)Lnet/minecraft/world/effect/MobEffect;
public requiredFeatures([Lnet/minecraft/world/flag/FeatureFlag;)Lnet/minecraft/world/effect/MobEffect;
public requiredFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
private static synthetic lambda$createModifiers$0(Ljava/util/function/BiConsumer;ILnet/minecraft/core/Holder;Lnet/minecraft/world/effect/MobEffect$AttributeTemplate;)V
private static synthetic lambda$onEffectAdded$0(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/sounds/SoundEvent;)V
private static synthetic lambda$new$1(Lnet/minecraft/core/particles/ParticleOptions;Lnet/minecraft/world/effect/MobEffectInstance;)Lnet/minecraft/core/particles/ParticleOptions;
private static synthetic lambda$new$0(ILnet/minecraft/world/effect/MobEffectInstance;)Lnet/minecraft/core/particles/ParticleOptions;
static <clinit>()V
```
