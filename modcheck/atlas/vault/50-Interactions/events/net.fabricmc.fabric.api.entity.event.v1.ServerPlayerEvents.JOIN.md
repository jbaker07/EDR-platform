---
type: "event"
event: "net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN"
callback: "net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents$Join"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN

Callback interface: `net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents$Join`

Module: [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `PlayerListMixin.firePlayerJoinEvent` @10 | [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` @Inject RETURN | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN|read it]].
