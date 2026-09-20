---
type: "workflow"
id: "wf.presentation.models_animation"
area: "presentation"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Models, textures and animation

**Intent.** Blocks, items and entities should look the way the creator designed them, including animated entity models, in the current pack format.

## Must be preserved

- Pack format compatibility with players' own resource packs ([[80-Unresolved/q.pack_format_old_packs|q.pack_format_old_packs]]).

## Mechanisms that can serve it

- Resource packs: the 15 asset entry types shipped in the jar, format 97.1 (`extracted/corpus.json`).
- [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] -- model loading hooks.
- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- entity renderer and model layer registration; [[50-Interactions/events/net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback.EVENT|EVENT]] for feature layers.
- Vanilla keyframe animation classes in net.minecraft.client.animation (29 classes in the jar; unhooked, members not extracted).

## Tools and artifacts used today

- An external modelling tool exporting Java model code or JSON ([[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]); textures from an image editor.

## Decisions the creator must make

- JSON block/item models (data only) versus coded entity models.
- Vanilla keyframe animation versus procedural animation in the renderer.

## Information those decisions need

- The animation API's 26.3 members ([[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]).
- Pack format changes since the creator's last version ([[80-Unresolved/q.mojang_changelog_26_3|q.mojang_changelog_26_3]]).

## Existing automation

- None in ModCheck.

## Remaining manual or unsupported work

- All authoring.

## ModCheck's contribution

- Extraction of the animation and model packages; a pack.mcmeta format check.

## Evidence

- `extracted/corpus.json`
- `extracted/fabric_api.json#fabric-model-loading-api-v1`
- `extracted/edges.json#callback_of`

## Open questions

- [[80-Unresolved/q.external_asset_tools|q.external_asset_tools]]
- [[80-Unresolved/q.unhooked_vanilla_members|q.unhooked_vanilla_members]]
- [[80-Unresolved/q.pack_format_old_packs|q.pack_format_old_packs]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
