---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.MOB_CONVERSION"
callback: "net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents$MobConversion"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents.MOB_CONVERSION

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.ServerLivingEntityEvents$MobConversion`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `MobMixin.afterEntityConverted` | `Mob.convertTo(Lnet/minecraft/world/entity/EntityType;Lnet/minecraft/world/entity/ConversionParams;Lnet/minecraft/world/entity/EntitySpawnReason;Lnet/minecraft/world/entity/ConversionParams$AfterConversion;)Lnet/minecraft/world/entity/Mob;` @ModifyArg at INVOKE Lnet/minecraft/server/level/ServerLevel;addFreshEntity(Lnet/minecraft/world/entity/Entity;)Z | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
