---
type: "event"
event: "net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.STOP_TRACKING"
callback: "net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents$StopTracking"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.STOP_TRACKING

Callback interface: `net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents$StopTracking`

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ServerEntityMixin.onStopTracking` @14 | [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]].`removePairing` @Inject HEAD | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
