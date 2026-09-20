---
type: "workflow"
id: "wf.state.migration"
area: "state"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Change a persisted shape without losing players' data

**Intent.** A later version of the mod stores something differently. Old worlds must still load, with their data carried across, and the migration must be safe to run twice.

## Must be preserved

- Every existing world's data; a failed migration must not leave a half-written state.

## Mechanisms that can serve it

- Codec versioning on read (a version field in the saved compound; migrate then write back on next save).
- Vanilla DataFixers exist (net.minecraft.util.datafix, 444 classes) but no mod-facing registration is in the corpus (system note net.minecraft.util.datafix; [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]).

## Tools and artifacts used today

- Hand-written migration in the load path; tests over fixture compounds.

## Decisions the creator must make

- In-place migrate on load (simple) versus keep both readers.
- When to drop support for the oldest shape.

## Information those decisions need

- Documented Fabric practice ([[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]).
- What vanilla's world version 5023 fixers touch that could interact with mod data (they do not touch mod namespaces by default; not verified).

## Existing automation

- None.

## Remaining manual or unsupported work

- All.

## ModCheck's contribution

- Template for a versioned codec in the SavedData generator; a fixture-based migration test scaffold.

## Evidence

- `extracted/corpus.json`
- `extracted/edges.json#injects_into`

## Open questions

- [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]
- [[80-Unresolved/q.older_minecraft_versions|q.older_minecraft_versions]]

## Status

- inventoried: True
- mechanically_inspected: False
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
