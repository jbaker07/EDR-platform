---
type: "workflow"
id: "wf.behaviour.lifecycle_events"
area: "behaviour"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Run code at lifecycle points (start, tick, load, join, reload)

**Intent.** Something should happen when the server starts, every tick, when a level or entity loads, when a player joins, or when data packs reload -- reliably, on the right side, without stalling the game.

## Must be preserved

- The tick budget: a listener's cost is paid by every player.
- Idempotence across reloads and chunk reloads.

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] (server and level ticks, lifecycle, entity, chunk, block entity events) and its client counterpart.
- [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] (player join/leave/respawn, living entity events).
- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] connection events.
- Contracts for the main ones are under _authored/contracts (END_SERVER_TICK, END_LEVEL_TICK, SERVER_STARTING/STARTED/STOPPING, SYNC_DATA_PACK_CONTENTS, ENTITY_LOAD, CHUNK_GENERATE).

## Tools and artifacts used today

- Register on the Event field from the right entrypoint ([[40-Interfaces/net.fabricmc.api.ModInitializer|ModInitializer]] common, [[40-Interfaces/net.fabricmc.api.ClientModInitializer|ClientModInitializer]] client).

## Decisions the creator must make

- Which event: the atlas's per-event notes give the injection site, side and callback.
- Server tick versus level tick (the level tick hands the ServerLevel).
- Phase ordering when order matters (not extracted; [[80-Unresolved/q.event_phase_ordering|q.event_phase_ordering]]).

## Information those decisions need

- Injection site and side of each event (`extracted/edges.json#publishes_event` joined to injects_into).
- Thread of each event (analyst inference only; [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]).

## Existing automation

- Server tick generator (`capability/subscribe_event.fabric_server_tick`); client entrypoint recipe (`capability/subscribe_event.fabric_add_client_entrypoint`).

## Remaining manual or unsupported work

- Every other event; ordering.

## ModCheck's contribution

- Event notes with site and contract for all 189 events; a contract checker ([[80-Unresolved/q.contract_checker|q.contract_checker]]).

## Interactions to check

- Listener ordering across mods on the same event.

## Evidence

- `capability/subscribe_event.fabric_server_tick`
- `capability/subscribe_event.fabric_add_client_entrypoint`
- `extracted/edges.json#publishes_event`
- `extracted/edges.json#callback_of`

## Open questions

- [[80-Unresolved/q.event_phase_ordering|q.event_phase_ordering]]
- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]
- [[80-Unresolved/q.event_publishers_missing|q.event_publishers_missing]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: server tick and client entrypoint generators compile against the pinned corpus (the discarded lantern scaffold built with them); the hand-authored reference lantern does not use a Fabric tick event -- it uses a block-entity ticker
