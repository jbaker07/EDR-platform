---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.ALLOW_EARLY_REMOVE"
callback: "net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents$AllowEarlyRemove"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents.ALLOW_EARLY_REMOVE

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.effect.ServerMobEffectEvents$AllowEarlyRemove`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.allowRemoveAllEffects` | (handler is not itself an injector method: fired from a helper or impl class) | both | static_inference |
| `LivingEntityMixin.allowRemoveEffect` | (handler is not itself an injector method: fired from a helper or impl class) | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
