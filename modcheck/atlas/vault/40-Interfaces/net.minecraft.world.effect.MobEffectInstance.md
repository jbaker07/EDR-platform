---
type: "interface"
fqcn: "net.minecraft.world.effect.MobEffectInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.effect.MobEffectInstance

System: [[20-Systems/net.minecraft.world.effect|net.minecraft.world.effect]]

`class` public; extends `java/lang/Object`; implements `java/lang/Comparable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getEffect` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@1 in `ServerMobEffectEvents.lambda$static$14` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getEffect` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@1 in `ServerMobEffectEvents.lambda$static$13` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| calls | `getEffect` | `()Lnet/minecraft/core/Holder;` | exact | invokevirtual@1 in `ServerMobEffectEvents.lambda$static$12` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (14 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final INFINITE_DURATION : I
public static final MIN_AMPLIFIER : I
public static final MAX_AMPLIFIER : I
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final effect : Lnet/minecraft/core/Holder;
private duration : I
private amplifier : I
private ambient : Z
private visible : Z
private showIcon : Z
private hiddenEffect : Lnet/minecraft/world/effect/MobEffectInstance;
private final blendState : Lnet/minecraft/world/effect/MobEffectInstance$BlendState;
public <init>(Lnet/minecraft/core/Holder;)V
public <init>(Lnet/minecraft/core/Holder;I)V
public <init>(Lnet/minecraft/core/Holder;II)V
public <init>(Lnet/minecraft/core/Holder;IIZZ)V
public <init>(Lnet/minecraft/core/Holder;IIZZZ)V
public <init>(Lnet/minecraft/core/Holder;IIZZZLnet/minecraft/world/effect/MobEffectInstance;)V
public <init>(Lnet/minecraft/world/effect/MobEffectInstance;)V
private <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/effect/MobEffectInstance$Details;)V
private asDetails()Lnet/minecraft/world/effect/MobEffectInstance$Details;
public getBlendFactor(Lnet/minecraft/world/entity/LivingEntity;F)F
public getParticleOptions()Lnet/minecraft/core/particles/ParticleOptions;
private setDetailsFrom(Lnet/minecraft/world/effect/MobEffectInstance;)V
public update(Lnet/minecraft/world/effect/MobEffectInstance;)Z
private isShorterDurationThan(Lnet/minecraft/world/effect/MobEffectInstance;)Z
public isInfiniteDuration()Z
public endsWithin(I)Z
public withScaledDuration(F)Lnet/minecraft/world/effect/MobEffectInstance;
public mapDuration(Lit/unimi/dsi/fastutil/ints/Int2IntFunction;)I
public getEffect()Lnet/minecraft/core/Holder;
public getDuration()I
public getAmplifier()I
public isAmbient()Z
public isVisible()Z
public showIcon()Z
public tickServer(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Ljava/lang/Runnable;)Z
public tickClient()V
private hasRemainingDuration()Z
private tickDownDuration()V
private downgradeToHiddenEffect()Z
public onEffectStarted(Lnet/minecraft/world/entity/LivingEntity;)V
public onMobRemoved(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/entity/Entity$RemovalReason;)V
public onMobHurt(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/world/damagesource/DamageSource;F)V
public getDescriptionId()Ljava/lang/String;
public toString()Ljava/lang/String;
private describeDuration()Ljava/lang/String;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public compareTo(Lnet/minecraft/world/effect/MobEffectInstance;)I
public onEffectAdded(Lnet/minecraft/world/entity/LivingEntity;)V
public is(Lnet/minecraft/core/Holder;)Z
public copyBlendState(Lnet/minecraft/world/effect/MobEffectInstance;)V
public skipBlending()V
public synthetic compareTo(Ljava/lang/Object;)I
private static synthetic lambda$tickDownDuration$0(I)I
private static synthetic lambda$withScaledDuration$0(FI)I
private static synthetic lambda$new$0(Lnet/minecraft/core/Holder;Lnet/minecraft/world/effect/MobEffectInstance$Details;)Lnet/minecraft/world/effect/MobEffectInstance;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
