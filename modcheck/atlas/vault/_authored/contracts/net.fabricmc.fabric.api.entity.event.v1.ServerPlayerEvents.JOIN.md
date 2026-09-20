---
type: "contract"
subject: "event:net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN|JOIN]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Fired from an @Inject at RETURN of PlayerList.placeNewPlayer, so the ServerPlayer is already in its level and the player list when the listener runs. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Server thread, both environments. | `analyst_inference` | `extracted/fabric_api.json#fabric-entity-events-v1` |

## Not established

- Whether the player's client has completed the play-phase handshake (ServerPlayConnectionEvents.JOIN is the networking-ready signal).

## Evidence

- `extracted/edges.json#publishes_event`
