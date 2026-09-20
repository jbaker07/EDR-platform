---
type: "event"
event: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_LOAD"
callback: "net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents$Load"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents.CHUNK_LOAD

Callback interface: `net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientChunkEvents$Load`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ClientChunkCacheMixin.onChunkLoad` | `ClientChunkCache.replaceWithPacketData` @Inject at TAIL | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
