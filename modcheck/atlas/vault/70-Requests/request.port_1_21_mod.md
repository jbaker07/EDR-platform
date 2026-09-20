---
type: "request"
id: "request.port_1_21_mod"
canonical: "exercise.port_1_21_mod"
kind: "analyst_exercise"
family: "workflow:wf.engineering.version_migration"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Port a 1.21-era Fabric mod with saved data to 26.3

**Canonical request.** `exercise.port_1_21_mod` (analyst_exercise)

> [!note] Analyst exercise
> No creator wrote this request. Nothing in it is approved intent.

## Request

Analyst-authored exercise request: a mod written for Minecraft 1.21 (Yarn or Mojang mappings, Java 21) that stores a custom per-level compound must build and run on 26.3, and worlds saved by the old version must load with their data intact.

## Approved behaviour (the request's own words or acceptance criteria)

- Provisional: same features; old saved compounds migrated on first load; no behaviour change beyond what the version forces.

## Preservation obligations

- Players' worlds and the mod's data in them.
- The mod's public ids (mod id, item and block ids, payload ids).

## Affected systems

- Build: mappings removed, Loom 1.17.21, Java 25 (`extracted/corpus.json` build_tooling).
- net.minecraft.util.datafix -- vanilla migrates its own data across world version 5023; the mod's data is its own responsibility.
- Every API the mod touched; the corrected records are the known renames.

## Implementation candidates

- Mechanical pass: apply `resolution/remove_mappings_for_non_obfuscated_minecraft`; bump Java (`failure/java_runtime_older_than_the_mod_requires`); compile; fix each error against the pinned corpus using the interface notes (the same process that corrected the capability records: END_WORLD_TICK to END_LEVEL_TICK, playS2C to clientboundPlay, the HUD signature, getGameRules on ServerLevel).
- Data migration: add a version field to the SavedData codec; on load of an unversioned compound, map the old keys to the new shape and mark the data dirty so the next save writes the new form.
- Keep two branches (1.21 and 26.3): standard Fabric practice; not a migration of the code but of the project.

## Data / control / state dependencies

- Data: the old compound layout (from the old code, not from any corpus artifact).
- Control: the load path of the SavedData is the only migration point; it must run once per level.
- State: old worlds; new worlds.

## Interactions with the selected environment

- Pack formats 97.1 and 121.0: the mod's bundled resources must declare them or players see a pack-format warning or failure ([[80-Unresolved/q.pack_format_old_packs|q.pack_format_old_packs]]).
- Protocol 777: any hand-rolled packet code must be re-registered as custom payloads.
- Other mods the old version integrated with may not exist for 26.3; the dependency inspection ([[10-Workflows/wf.integration.dependencies_conditions|wf.integration.dependencies_conditions]]) lists them.

## Alternatives and tradeoffs

- Drop old-world support and reset data: simpler, hostile to players; not recommended.

## Implementation work

- Build-script migration; compile-fix loop against the pinned corpus; codec versioning; a fixture test loading an old compound; pack.mcmeta update.

## Verification obligations

- Compile against the pinned corpus (possible now).
- Fixture-based migration test (possible now).
- Loading a real old world: needs a game and an old-version world; none observed.

## Unresolved

- [[80-Unresolved/q.older_minecraft_versions|q.older_minecraft_versions]]
- [[80-Unresolved/q.version_diff_tool|q.version_diff_tool]]
- [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]
- [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]
- [[80-Unresolved/q.pack_format_old_packs|q.pack_format_old_packs]]

## Evidence

- `resolution/remove_mappings_for_non_obfuscated_minecraft`
- `failure/mojang_mappings_in_non_obfuscated_environment`
- `failure/java_runtime_older_than_the_mod_requires`
- [[00-Scope/Sources|fabric_example_mod_build_1_21]]
- [[00-Scope/Sources|fabric_example_mod_build_26_3]]
- `extracted/corpus.json`

## Status

- analysed: True
- implemented: none
- validated_scope: none
