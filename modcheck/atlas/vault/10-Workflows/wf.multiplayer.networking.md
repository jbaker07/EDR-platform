---
type: "workflow"
id: "wf.multiplayer.networking"
area: "multiplayer"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Send state between server and client

**Intent.** The client must know something only the server knows (or the reverse): a counter to display, a request to act. The creator wants it to arrive for every player, including those who join late, and to fail loudly on a version mismatch.

## Must be preserved

- Server authority: the client never decides game state, it displays it.
- Bandwidth: send deltas or on change, not every tick.

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] -- custom payload types registered per side with a StreamCodec; global receivers; per-player senders (`capability/sync_state.fabric_custom_payload`, which was corrected from playS2C() to clientboundPlay() against the real artifact).
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN|JOIN]] -- initial sync to a joining player through the handed PacketSender.
- Attachment sync as a no-code alternative for attached state ([[10-Workflows/wf.state.persistence|wf.state.persistence]]).
- Registry sync ([[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]]) ensures both sides agree on ids before play.

## Tools and artifacts used today

- The generated payload record, codec and registration; a client receiver updating a client-held copy.

## Decisions the creator must make

- Payload versus attachment sync versus vanilla data-tracker for entity fields.
- When to send: on change, on join, periodically.
- What the client does when the mod is absent on one side (the payload id is unknown; the loader rejects the client only if the mod declares itself required).

## Information those decisions need

- The receiver's thread ([[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]).
- Readiness point for the first send (ServerPlayConnectionEvents.JOIN contract).

## Existing automation

- Payload generator in ModCheck.

## Remaining manual or unsupported work

- Delta logic; join-time sync wiring.

## ModCheck's contribution

- Delivered for the payload; a join-sync capability is the next increment.

## Interactions to check

- Payload id collisions between mods (namespaced; collision only within one namespace).
- Protocol version mismatch (777) between client and server -- vanilla's own check precedes any mod code.

## Evidence

- `capability/sync_state.fabric_custom_payload`
- `extracted/fabric_api.json#fabric-networking-api-v1`
- `extracted/edges.json#callback_of`

## Open questions

- [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: payload generator compiles against the pinned corpus; exercised by the reference lantern's JUnit tests with fakes
