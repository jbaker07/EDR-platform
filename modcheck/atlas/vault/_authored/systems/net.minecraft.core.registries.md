---
type: "system_note"
id: "net.minecraft.core.registries"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Built-in registries and registry keys

Package `net.minecraft.core.registries` -- generated view: [[20-Systems/net.minecraft.core.registries|hooked types]]

**Responsibility.** BuiltInRegistries holds the 97 static registries every mod registers content into; Registries holds the 157 keys, including the dynamic (data-driven) ones loaded from data packs.

**Side.** shared_by_design

**Threads.** Registration happens during mod initialisation on the main thread of the launching side; the freeze point relative to entrypoints is not extracted ([[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]).

## Extension points

- [[30-Mechanisms/Registries|Registries]] lists every registry field and key with the element type, read from the jar.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT|EVENT]] ([[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]]) fires before dynamic registries load.
- [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] synchronises registry ids between server and client so that a client missing a mod's content is rejected rather than desynchronised.

## Interactions to expect

- Two mods registering the same namespaced id is a hard failure at registration, not at use.

## Evidence

- `extracted/minecraft_registries.json`
- `extracted/edges.json#registers_into`

## Open questions

- [[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]

