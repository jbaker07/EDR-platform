---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.ALLOW"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents$Allow"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.ALLOW

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents$Allow`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.injectElytraTick` @15 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`updateFallFlying` @Inject INVOKE `Lnet/minecraft/util/Util;getRandom(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object;` | unknown | static_inference |
| `LivingEntityMixin.injectElytraCheck` @15 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`canGlide` @Inject FIELD `Lnet/minecraft/world/entity/EquipmentSlot;VALUES:Ljava/util/List;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
