---
type: "workflow"
id: "wf.content.block_item"
area: "content"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Add a block and its item

**Intent.** A creator wants a new placeable block with a matching inventory item: it should look right in the world and in hand, drop something when broken, have a name in every language the mod ships, and behave like other blocks of its kind (light, hardness, sound, tool requirement).

## Must be preserved

- The creator's chosen id, namespace and display name exactly as given.
- Vanilla behaviour of every other block; the new block must not alter defaults.

## Mechanisms that can serve it

- [[30-Mechanisms/Registries|Registries]] -- BLOCK and ITEM built-in registries are the registration targets.
- [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] -- builders for block and item settings.
- [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] -- generates blockstate, model, loot and language JSON at build time.
- [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] -- [[50-Interactions/events/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents.MODIFY_OUTPUT_ALL|MODIFY_OUTPUT_ALL]] places the item in a creative tab.

## Tools and artifacts used today

- Fabric Loom project ([[00-Scope/Sources|fabric_example_mod_build_26_3]], [[00-Scope/Sources|fabric_example_mod_versions_26_3]]) compiled with JDK 25 and Gradle 9.5.1 (`capability/build_and_test.fabric_project_setup_current_version`).
- Hand-written or data-generated JSON under assets/<ns>/blockstates, models/block, models/item, lang and data/<ns>/loot_table (entry types present in the 26.3 jar itself: `extracted/corpus.json`).
- An image editor for the 16x16 texture (not in corpus; [[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]).

## Decisions the creator must make

- Register in code (needed for any block) versus define appearance and drops in data (always data).
- Which vanilla block's settings to copy, which decides sound, hardness and tool tags.
- Whether the block needs a block entity (state beyond blockstate properties) -- if so, [[10-Workflows/wf.content.block_entity|wf.content.block_entity]].

## Information those decisions need

- The exact registry field names and element types in 26.3 (`extracted/minecraft_registries.json`).
- The current model and blockstate JSON shapes for resource pack format 97.1 (only the jar's own examples are in corpus; [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]).
- Whether registration must happen before the registry freezes ([[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]).

## Existing automation

- Data generation module generates the JSON from code (mechanically inspected; its API surface is in `extracted/fabric_api.json#fabric-data-generation-api-v1`).
- The ModCheck scaffold produces a buildable project; it does not yet emit a block.

## Remaining manual or unsupported work

- Texture authoring.
- Choosing block settings; there is no capability record for block registration yet.

## ModCheck's contribution

- A capability record and deterministic generator for block+item registration with data-generated JSON, validated by compiling against the pinned corpus.
- A check that the mod's pack.mcmeta format matches 97.1 / 121.0 from version.json.

## Interactions to check

- Same namespaced id registered by two mods (hard failure at registration).
- Creative tab ordering when several mods add to one vanilla tab.

## Evidence

- `extracted/minecraft_registries.json`
- `extracted/corpus.json`
- `extracted/fabric_api.json#fabric-object-builder-api-v1`
- `capability/build_and_test.fabric_project_setup_current_version`

## Open questions

- [[80-Unresolved/q.registry_freeze_timing|q.registry_freeze_timing]]
- [[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
