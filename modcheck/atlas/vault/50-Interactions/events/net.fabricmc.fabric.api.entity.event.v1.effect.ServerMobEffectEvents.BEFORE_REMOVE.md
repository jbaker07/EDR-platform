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

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.beforeRemoveEffect` | `LivingEntity.removeEffect` @Inject at HEAD | both | static_inference |
| `LivingEntityMixin.beforeExpireRemoveEffect` | `LivingEntity.tickEffects` @Inject at INVOKE Ljava/util/Iterator;remove()V | both | static_inference |
| `LivingEntityMixin.beforeRemoveAllEffects` | `LivingEntity.removeAllEffects` @Inject at INVOKE Lcom/google/common/collect/Maps;newHashMap(Ljava/util/Map;)Ljava/util/HashMap; | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
