---
type: "contract"
subject: "event:net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS"
kind: "event"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS

Subject: [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SYNC_DATA_PACK_CONTENTS|SYNC_DATA_PACK_CONTENTS]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Has two publication sites -- PlayerList.placeNewPlayer (per joining player, at the construction of the recipes packet) and PlayerList.reloadResources (per player, after a reload) -- so a listener receives every player once on join and again on each reload. | `static_inference` | `extracted/edges.json#publishes_event`; `extracted/edges.json#injects_into` |
| Callback carries (ServerPlayer, boolean joined); the boolean distinguishes the two sites. | `declared` | `extracted/edges.json#callback_of` |
| This is the correct point to send a mod's data-driven content to a client, and the wrong point to send per-player state that exists before login. | `analyst_inference` | `extracted/edges.json#injects_into` |

## Not established

- Whether the client is ready to receive custom payloads at the placeNewPlayer site (networking readiness is ServerPlayConnectionEvents.JOIN's concern).

## Evidence

- `extracted/edges.json#publishes_event`
