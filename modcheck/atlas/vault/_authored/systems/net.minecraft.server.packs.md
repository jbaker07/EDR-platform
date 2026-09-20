---
type: "system_note"
id: "net.minecraft.server.packs"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Packs, resources and reload

Package `net.minecraft.server.packs` -- generated view: [[20-Systems/net.minecraft.server.packs|hooked types]]

**Responsibility.** Pack repositories, pack resources, the resource manager and reload listeners; the server side loads data packs, the client side resource packs, through the same classes. 25 hooked types.

**Side.** shared_by_design

**Threads.** Reloads run partly on worker executors with a completion on the game thread; the split is not extracted.

## Extension points

- [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] injects mod resources as built-in packs and exposes reload listeners with ordering.
- [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] lets a JSON resource load only when a condition (a mod present, a tag populated) holds.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.END_DATA_PACK_RELOAD|END_DATA_PACK_RELOAD]] signals completion on the server; SYNC_DATA_PACK_CONTENTS fires per player when reloaded data is sent.

## Interactions to expect

- Pack precedence between a mod's built-in pack and a world's data pack decides which same-named JSON wins ([[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]).

## Evidence

- `extracted/edges.json#injects_into`
- `extracted/edges.json#publishes_event`
- [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]]

## Open questions

- [[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]

