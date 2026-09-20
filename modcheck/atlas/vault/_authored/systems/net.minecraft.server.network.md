---
type: "system_note"
id: "net.minecraft.server.network"
side: "server"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Server-side connection handling

Package `net.minecraft.server.network` -- generated view: [[20-Systems/net.minecraft.server.network|inventory and hooked types]]

**Responsibility.** ServerGamePacketListenerImpl and the configuration/login listeners: the server end of each player's connection, where serverbound packets are handled.

**Side.** server

**Threads.** Handlers run on the server thread after vanilla's re-scheduling for game packets; login and configuration have their own flow.

## Extension points

- [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] attaches its per-connection addon here; ServerPlayConnectionEvents.JOIN is published from that addon (impl code, not a mixin).
- [[50-Interactions/events/net.fabricmc.fabric.api.entity.event.v1.ServerPlayerEvents.JOIN|JOIN]] fires at RETURN of PlayerList.placeNewPlayer, after the player is in the level.

## Evidence

- `extracted/edges.json#publishes_event`
- `extracted/edges.json#injects_into`

