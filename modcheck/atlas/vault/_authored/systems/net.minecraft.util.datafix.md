---
type: "system_note"
id: "net.minecraft.util.datafix"
side: "shared_by_design"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# DataFixers -- vanilla save migration

Package `net.minecraft.util.datafix` -- generated view: [[20-Systems/net.minecraft.util.datafix|hooked types]]

**Responsibility.** Vanilla's schema-versioned migration of saved data (444 classes): every world version bump ships fixes here. Two types are hooked by Fabric API.

**Side.** shared_by_design

**Threads.** Runs during world load on the loading thread.

**Persistence.** This is the mechanism by which vanilla keeps old worlds loadable; world version 5023 (`extracted/corpus.json`) is the current schema.

## Extension points

- No Fabric API in the corpus exposes a mod-facing fixer registration; the two hooks are Fabric's own (`extracted/edges.json#injects_into`). Mods therefore version their own data ([[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]).

## Evidence

- `extracted/corpus.json`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]

