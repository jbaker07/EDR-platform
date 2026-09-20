---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.CUSTOM"
callback: "net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents$Custom"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents.CUSTOM

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.EntityElytraEvents$Custom`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.injectElytraTick` | `LivingEntity.updateFallFlying()V` @Inject at INVOKE Lnet/minecraft/util/Util;getRandom(Ljava/util/List;Lnet/minecraft/util/RandomSource;)Ljava/lang/Object; | both | static_inference |
| `LivingEntityMixin.injectElytraCheck` | `LivingEntity.canGlide` @Inject at FIELD Lnet/minecraft/world/entity/EquipmentSlot;VALUES:Ljava/util/List; | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
