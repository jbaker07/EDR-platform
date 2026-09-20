---
type: "workflow"
id: "wf.presentation.hud_screens"
area: "presentation"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Show information on the HUD or in a screen

**Intent.** The player should see something: a counter, a status, a menu. It must render only on the client, never load on a dedicated server, and reflect state the server owns.

## Must be preserved

- Vanilla HUD layout and other mods' HUD elements.
- Dedicated-server startup (no client class references from common code).

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- HUD element registration; HudElement in 26.3 uses extractRenderState(GuiGraphicsExtractor, DeltaTracker) (`capability/display_information.fabric_hud_element`).
- [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] -- screen events ([[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT|AFTER_INIT]]).
- [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] -- key bindings to open a screen.
- Client entrypoint (`capability/subscribe_event.fabric_add_client_entrypoint`).

## Tools and artifacts used today

- The generated HUD element with a client-held copy of server state ([[10-Workflows/wf.multiplayer.networking|wf.multiplayer.networking]]).

## Decisions the creator must make

- HUD element (always visible) versus screen (opened).
- What state the client needs and how it arrives (payload).

## Information those decisions need

- The 26.3 GUI render-state API; the earlier GuiGraphics signature does not exist (corrected record).

## Existing automation

- HUD element generator in ModCheck.

## Remaining manual or unsupported work

- Screens; layout; key bindings.

## ModCheck's contribution

- Delivered for HUD elements; the client-only check belongs to [[10-Workflows/wf.multiplayer.side_separation|wf.multiplayer.side_separation]].

## Evidence

- `capability/display_information.fabric_hud_element`
- `capability/subscribe_event.fabric_add_client_entrypoint`
- `extracted/fabric_api.json#fabric-rendering-v1`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: HUD element generator compiles against the pinned corpus; the reference lantern's HUD (viewed-lantern selection, staleness) is hand-authored and unit-tested with fakes (12 tests); no game run
