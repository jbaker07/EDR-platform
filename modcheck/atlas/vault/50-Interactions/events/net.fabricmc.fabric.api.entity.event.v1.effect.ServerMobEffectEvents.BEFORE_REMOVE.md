---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.BEFORE_REMOVE"
callback: "net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents$BeforeRemove"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.BEFORE_REMOVE

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents$BeforeRemove`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.beforeRemoveEffect` @39 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeEffect` @Inject HEAD | unknown | static_inference |
| `LivingEntityMixin.beforeExpireRemoveEffect` @25 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`tickEffects` @Inject INVOKE `Ljava/util/Iterator;remove()V` | unknown | static_inference |
| `LivingEntityMixin.beforeRemoveAllEffects` @57 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`removeAllEffects` @Inject INVOKE `Lcom/google/common/collect/Maps;newHashMap(Ljava/util/Map;)Ljava/util/HashMap;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
