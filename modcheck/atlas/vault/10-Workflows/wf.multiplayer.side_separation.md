---
type: "workflow"
id: "wf.multiplayer.side_separation"
area: "multiplayer"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Keep client and server code apart

**Intent.** The mod must run on a dedicated server without client classes, on a client with its integrated server, and on a client connecting to a modded server -- with the same jar.

## Must be preserved

- Dedicated-server startup: one client-class reference in common code is a crash.

## Mechanisms that can serve it

- Entrypoints: [[40-Interfaces/net.fabricmc.api.ModInitializer|ModInitializer]] (both), [[40-Interfaces/net.fabricmc.api.ClientModInitializer|ClientModInitializer]], [[40-Interfaces/net.fabricmc.api.DedicatedServerModInitializer|DedicatedServerModInitializer]] (loader surface in `extracted/fabric_api.json`).
- Mixin config client/server lists and the @Environment annotation (the source of every edge's applies_to.environment).
- Fabric API modules declaring environment: client (14 of 47 in `extracted/fabric_api.json`) can only be depended on from client code.

## Tools and artifacts used today

- Separate client source set or package; the client entrypoint recipe (`recipe/fabric_add_client_entrypoint`).

## Decisions the creator must make

- Source-set split versus package discipline.
- Which events are two-sided (interaction events fire on both sides; the contracts say so).

## Information those decisions need

- Per-module environment (extracted).
- Per-event side (extracted through the injection's environment).

## Existing automation

- Client entrypoint generator.

## Remaining manual or unsupported work

- Reviewing common code for client references.

## ModCheck's contribution

- A static check that flags a client-only module or class referenced from a common entrypoint ([[80-Unresolved/q.contract_checker|q.contract_checker]]).

## Interactions to check

- Single-player: server-side and client-side listeners both run in one JVM; shared static state is a bug.

## Evidence

- `extracted/fabric_api.json`
- `recipe/fabric_add_client_entrypoint`
- `capability/subscribe_event.fabric_add_client_entrypoint`

## Open questions

- [[80-Unresolved/q.contract_checker|q.contract_checker]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: client entrypoint generator compiles against the pinned corpus; no side-reference check exists
