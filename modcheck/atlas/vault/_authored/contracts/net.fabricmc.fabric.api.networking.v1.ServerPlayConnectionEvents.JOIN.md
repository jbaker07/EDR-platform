---
type: "contract"
subject: "event:net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN|JOIN]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Published from ServerPlayNetworkAddon.onClientReady in the networking module's impl code, not from a mixin; the extraction therefore does not place it in the vanilla call graph. | `static_inference` | `extracted/edges.json#publishes_event` |
| Callback onPlayReady(ServerGamePacketListenerImpl, PacketSender, MinecraftServer) hands a sender, which is the intended way to push initial state to a client that has just become able to receive custom payloads. | `declared` | `extracted/edges.json#callback_of`; `capability/sync_state.fabric_custom_payload` |

## Not established

- The vanilla method from which onClientReady is reached, and its thread ([[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]], [[80-Unresolved/q.event_publishers_missing|q.event_publishers_missing]]).

## Evidence

- `extracted/edges.json#publishes_event`

## Open questions

- [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]
