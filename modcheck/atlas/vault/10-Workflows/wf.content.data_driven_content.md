---
type: "workflow"
id: "wf.content.data_driven_content"
area: "content"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add recipes, loot, advancements and tags with data

**Intent.** Most content additions are data, not code: a recipe, a loot table, an advancement, a tag membership, a worldgen definition. The creator wants the JSON right for 26.3 and loaded in the right order relative to other packs.

## Must be preserved

- Vanilla data the mod does not override; overriding a vanilla file replaces it for every player.

## Mechanisms that can serve it

- Data packs: the 44 data entry types the jar itself ships (`extracted/corpus.json`), each a directory the loader reads.
- [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] -- mod resources become built-in packs.
- [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] -- conditional loading of a JSON.
- [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] -- [[50-Interactions/events/net.fabricmc.fabric.api.loot.v3.LootTableEvents.MODIFY|MODIFY]] edits vanilla loot without replacing the file.
- [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]], [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]], [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]].
- [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- generates all of it from code.

## Tools and artifacts used today

- Data generation run in the Loom project; or hand-written JSON checked by loading a world.

## Decisions the creator must make

- Replace a vanilla file (simple, hostile to other mods) or modify through an event (composable).
- Hand-written or generated JSON.

## Information those decisions need

- The data pack format 121.0 JSON shapes; the jar's own 1866 advancements and other entries are the only in-corpus examples (`extracted/corpus.json`).
- Pack precedence ([[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]).

## Existing automation

- Data generation (mechanically inspected).

## Remaining manual or unsupported work

- Choosing the modify-versus-replace policy per file.

## ModCheck's contribution

- A check that flags a mod data file whose path shadows a vanilla one and suggests the event-based alternative.

## Interactions to check

- Two mods shadowing the same vanilla path; last pack wins silently.

## Evidence

- `extracted/corpus.json`
- `extracted/fabric_api.json#fabric-resource-loader-v0`
- `extracted/edges.json#callback_of`

## Open questions

- [[80-Unresolved/q.datapack_load_order|q.datapack_load_order]]
- [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
