---
type: "interface"
fqcn: "net.minecraft.world.damagesource.DamageSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.damagesource.DamageSource

System: [[20-Systems/net.minecraft.world.damagesource|net.minecraft.world.damagesource]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getEntity` | `()Lnet/minecraft/world/entity/Entity;` | exact | invokevirtual@1 in `ServerPlayerMixin.callOnKillForPlayer` | unknown | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (4 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final type : Lnet/minecraft/core/Holder;
private final causingEntity : Lnet/minecraft/world/entity/Entity;
private final directEntity : Lnet/minecraft/world/entity/Entity;
private final damageSourcePosition : Lnet/minecraft/world/phys/Vec3;
public toString()Ljava/lang/String;
public getFoodExhaustion()F
public isDirect()Z
private <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/phys/Vec3;)V
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/Entity;)V
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/phys/Vec3;)V
public <init>(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/Entity;)V
public <init>(Lnet/minecraft/core/Holder;)V
public getDirectEntity()Lnet/minecraft/world/entity/Entity;
public getEntity()Lnet/minecraft/world/entity/Entity;
public getWeaponItem()Lnet/minecraft/world/item/ItemStack;
public getLocalizedDeathMessage(Lnet/minecraft/world/entity/LivingEntity;)Lnet/minecraft/network/chat/Component;
public getMsgId()Ljava/lang/String;
public scalesWithDifficulty()Z
public isCreativePlayer()Z
public getSourcePosition()Lnet/minecraft/world/phys/Vec3;
public sourcePositionRaw()Lnet/minecraft/world/phys/Vec3;
public is(Lnet/minecraft/tags/TagKey;)Z
public is(Lnet/minecraft/resources/ResourceKey;)Z
public type()Lnet/minecraft/world/damagesource/DamageType;
public typeHolder()Lnet/minecraft/core/Holder;
```
