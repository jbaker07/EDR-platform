---
type: "system_note"
id: "net.minecraft.network.protocol"
side: "both"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Packets and protocol phases

Package `net.minecraft.network.protocol` -- generated view: [[20-Systems/net.minecraft.network.protocol|inventory and hooked types]]

**Responsibility.** The wire protocol (368 classes): packet types for the handshake, login, configuration and play phases, and the clientbound/serverbound listener interfaces. Custom payloads travel inside the common CustomPayload packets.

**Side.** both

**Threads.** Packets are decoded on netty threads; vanilla re-schedules handling onto the game thread for most packets. Whether Fabric's custom payload receivers follow that re-scheduling is [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]].

## Extension points

- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] registers payload types and receivers per phase (`capability/sync_state.fabric_custom_payload`).
- Channel registration between sides is announced through [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ClientboundPlayChannelEvents.REGISTER|REGISTER]] and its serverbound counterpart.
- Connection lifecycle: [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN|JOIN]] (server) and [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents.JOIN|JOIN]] (client).

## Interactions to expect

- A payload id must be registered on both sides with the same codec; a mismatch disconnects the client.
- Protocol version 777 (`extracted/corpus.json`) is the compatibility boundary; a client and server on different protocols cannot connect at all.

## Evidence

- `extracted/corpus.json`
- `extracted/edges.json#callback_of`
- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]]

## Open questions

- [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]

