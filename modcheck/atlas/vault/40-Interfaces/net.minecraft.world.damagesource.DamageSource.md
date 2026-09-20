---
type: "interface"
fqcn: "net.minecraft.world.damagesource.DamageSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.damagesource.DamageSource

System: [[20-Systems/net.minecraft.world.damagesource|net.minecraft.world.damagesource]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getEntity()Lnet/minecraft/world/entity/Entity;` | `` | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (25, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.damagesource.DamageSource {
    private final net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType> type;
    private final net.minecraft.world.entity.Entity causingEntity;
    private final net.minecraft.world.entity.Entity directEntity;
    private final net.minecraft.world.phys.Vec3 damageSourcePosition;
    public java.lang.String toString();
    public float getFoodExhaustion();
    public boolean isDirect();
    private net.minecraft.world.damagesource.DamageSource(net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType>, net.minecraft.world.entity.Entity, net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3);
    public net.minecraft.world.damagesource.DamageSource(net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType>, net.minecraft.world.entity.Entity, net.minecraft.world.entity.Entity);
    public net.minecraft.world.damagesource.DamageSource(net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType>, net.minecraft.world.phys.Vec3);
    public net.minecraft.world.damagesource.DamageSource(net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType>, net.minecraft.world.entity.Entity);
    public net.minecraft.world.damagesource.DamageSource(net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType>);
    public net.minecraft.world.entity.Entity getDirectEntity();
    public net.minecraft.world.entity.Entity getEntity();
    public net.minecraft.world.item.ItemStack getWeaponItem();
    public net.minecraft.network.chat.Component getLocalizedDeathMessage(net.minecraft.world.entity.LivingEntity);
    public java.lang.String getMsgId();
    public boolean scalesWithDifficulty();
    public boolean isCreativePlayer();
    public net.minecraft.world.phys.Vec3 getSourcePosition();
    public net.minecraft.world.phys.Vec3 sourcePositionRaw();
    public boolean is(net.minecraft.tags.TagKey<net.minecraft.world.damagesource.DamageType>);
    public boolean is(net.minecraft.resources.ResourceKey<net.minecraft.world.damagesource.DamageType>);
    public net.minecraft.world.damagesource.DamageType type();
    public net.minecraft.core.Holder<net.minecraft.world.damagesource.DamageType> typeHolder();
}
```
