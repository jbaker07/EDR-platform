---
type: "system_note"
id: "net.minecraft.client.renderer"
side: "client"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Rendering: level, entities, block and item models

Package `net.minecraft.client.renderer` -- generated view: [[20-Systems/net.minecraft.client.renderer|inventory and hooked types]]

**Responsibility.** The client render pipeline (1006 classes): level rendering passes, entity and block-entity renderers, model baking and dispatch, render state extraction, textures and sprites. 66 hooked types.

**Side.** client

**Threads.** The render thread only; anything here referenced from server code crashes a dedicated server with a missing class.

## Extension points

- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] exposes level render passes as events (for example [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.level.LevelRenderEvents.AFTER_TRANSLUCENT_TERRAIN|AFTER_TRANSLUCENT_TERRAIN]]) and entity renderer/model-layer registration.
- Feature render layers on living entities through [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback.EVENT|EVENT]].
- [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] and [[30-Mechanisms/fabric-renderer-indigo|fabric-renderer-indigo]] define the mesh/quad emitting model used by block models (the unresolved emitQuads targets in `extracted/minecraft_surface.json.gz` are interface-declared members of this pipeline).
- [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] hooks model loading and baking.

## Interactions to expect

- Every level render event is per frame; work there is paid at frame rate, not tick rate.

## Evidence

- `extracted/edges.json#injects_into`
- `extracted/corpus.json`

## Open questions

- [[80-Unresolved/q.edge_targets_unresolved|q.edge_targets_unresolved]]

