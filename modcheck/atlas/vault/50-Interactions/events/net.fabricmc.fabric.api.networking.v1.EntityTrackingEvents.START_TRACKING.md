---
type: "event"
event: "net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.START_TRACKING"
callback: "net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents$StartTracking"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.START_TRACKING

Callback interface: `net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents$StartTracking`

Module: [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ServerEntityMixin.onStartTracking` | `ServerEntity.addPairing` @Inject at TAIL | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
