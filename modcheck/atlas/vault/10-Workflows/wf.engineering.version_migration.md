---
type: "workflow"
id: "wf.engineering.version_migration"
area: "engineering"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Move a mod to 26.3

**Intent.** A mod written for an earlier version should build and behave on 26.3, with the changes listed and each one tied to evidence rather than folklore.

## Must be preserved

- Behaviour the creator did not ask to change.
- Players' worlds ([[10-Workflows/wf.state.migration|wf.state.migration]]).

## Mechanisms that can serve it

- Build script changes: mappings removed, Loom and Java bumped ([[00-Scope/Sources|fabric_example_mod_build_1_21]] versus [[00-Scope/Sources|fabric_example_mod_build_26_3]]).
- API renames found by compiling: END_WORLD_TICK to END_LEVEL_TICK, playS2C to clientboundPlay, HudElement's render-state signature, getGameRules moving to ServerLevel (all recorded in the corrected capability records).
- Pack formats 97.1 and 121.0 from version.json.

## Tools and artifacts used today

- Compile against the pinned corpus and read the errors; the corrected records.

## Decisions the creator must make

- Support one version or a range (Fabric mods usually branch per version).

## Information those decisions need

- A member-level diff between the source version and 26.3 ([[80-Unresolved/q.older_minecraft_versions|q.older_minecraft_versions]], [[80-Unresolved/q.version_diff_tool|q.version_diff_tool]]).
- The release notes for pack format changes ([[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]).

## Existing automation

- Build-failure-driven correction, by hand.

## Remaining manual or unsupported work

- Most of it.

## ModCheck's contribution

- The diff tool once a second corpus exists; until then, the corrected records are the migration notes.

## Evidence

- `resolution/remove_mappings_for_non_obfuscated_minecraft`
- `capability/subscribe_event.fabric_server_tick`
- `capability/sync_state.fabric_custom_payload`
- [[00-Scope/Sources|fabric_example_mod_build_1_21]]

## Open questions

- [[80-Unresolved/q.older_minecraft_versions|q.older_minecraft_versions]]
- [[80-Unresolved/q.version_diff_tool|q.version_diff_tool]]
- [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
