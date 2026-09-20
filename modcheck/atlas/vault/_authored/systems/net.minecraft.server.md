---
type: "system_note"
id: "net.minecraft.server"
side: "server"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# MinecraftServer -- the server run loop

Package `net.minecraft.server` -- generated view: [[20-Systems/net.minecraft.server|hooked types]]

**Responsibility.** Owns the server's main loop (runServer, tickServer, tickChildren), startup (initServer) and shutdown (stopServer), the set of loaded levels, the player list, resource reload and the save cycle. Both the dedicated server and the client's integrated server are instances of it.

**Side.** server

**Threads.** The run loop executes on the thread that calls runServer; every injection Fabric API makes into tickServer, runServer and stopServer runs on that thread. Which other threads call into this class (netty, chunk workers) is not modelled here ([[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]).

**Persistence.** The save cycle lives here (BEFORE_SAVE / AFTER_SAVE events are published from this package's mixins).

## Extension points

- [[40-Interfaces/net.minecraft.server.MinecraftServer|MinecraftServer]] is the most-hooked server type; lifecycle and tick events are injected into runServer, tickServer and stopServer (`extracted/edges.json#injects_into`).
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.SERVER_STARTING|SERVER_STARTING]] fires from an injection into runServer before initServer; SERVER_STARTED after it.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerTickEvents.END_SERVER_TICK|END_SERVER_TICK]] fires from an injection at TAIL of tickServer.
- Commands are registered through [[50-Interactions/events/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT|EVENT]], injected into the Commands constructor.

## Interactions to expect

- Every mod's tick listener shares this one thread; total handler time is the tick budget.
- Data pack reload re-runs registration paths; listeners must be idempotent across END_DATA_PACK_RELOAD.

## Evidence

- `extracted/edges.json#injects_into`
- `extracted/minecraft_members.json`
- [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Open questions

- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]
- [[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]

