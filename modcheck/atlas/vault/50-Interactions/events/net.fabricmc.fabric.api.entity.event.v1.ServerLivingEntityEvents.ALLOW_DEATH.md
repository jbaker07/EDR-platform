---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH"
callback: "net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents$AllowDeath"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents$AllowDeath`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `LivingEntityMixin.beforeEntityKilled` @20 | [[40-Interfaces/net.minecraft.world.entity.LivingEntity|LivingEntity]].`hurtServer` @Redirect INVOKE `Lnet/minecraft/world/entity/LivingEntity;isDeadOrDying()Z` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.ALLOW_DEATH|read it]].
