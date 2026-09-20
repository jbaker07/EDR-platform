---
type: "system_note"
id: "net.minecraft.client.multiplayer"
side: "client"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Client-side world and connection

Package `net.minecraft.client.multiplayer` -- generated view: [[20-Systems/net.minecraft.client.multiplayer|inventory and hooked types]]

**Responsibility.** ClientLevel, ClientPacketListener (the client end of the connection), MultiPlayerGameMode (client-initiated interactions and prediction). 15 hooked types.

**Side.** client

**Threads.** Client thread after packet re-scheduling; interaction methods are called from input handling on the client thread.

## Extension points

- The client-side halves of the interaction events (UseBlockCallback, AttackEntityCallback) are injected into MultiPlayerGameMode.useItemOn and MultiPlayerGameMode.attack before the packet is sent, so a client-side cancel prevents the packet.
- [[50-Interactions/events/net.fabricmc.fabric.api.client.command.v2.ClientCommandRegistrationCallback.EVENT|EVENT]] fires at RETURN of ClientPacketListener.handleLogin.
- Client tick: [[50-Interactions/events/net.fabricmc.fabric.api.client.event.lifecycle.v1.ClientTickEvents.END_CLIENT_TICK|END_CLIENT_TICK]] at RETURN of Minecraft.tick.

## Interactions to expect

- Interaction events fire on both sides with the same callback; a listener that is not side-aware runs twice in single-player.

## Evidence

- `extracted/edges.json#injects_into`
- `extracted/edges.json#publishes_event`

