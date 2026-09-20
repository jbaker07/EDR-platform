---
type: "workflow"
id: "wf.state.persistence"
area: "state"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Persist mod state with the world

**Intent.** Whatever the mod remembers -- a counter, a per-player flag, a per-block value -- must survive a server restart, belong to the right scope (level, player, entity, chunk, block), and never be written to a player's save except through the game's own save cycle.

## Must be preserved

- The world's existing data; a mod never rewrites files under the save directory itself.
- A stable shape once shipped; changing it is [[10-Workflows/wf.state.migration|wf.state.migration]].

## Mechanisms that can serve it

- Level-scoped SavedData (`capability/persist_state.fabric_saveddata`) -- what the discarded lantern scaffold generated; the hand-authored reference lantern persists per block entity instead ([[10-Workflows/wf.content.block_entity|wf.content.block_entity]]).
- Attachments ([[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]], `capability/persist_state.fabric_attachment`) on entities, block entities, chunks and levels, with optional persistence and sync (AttachmentRegistry.Builder.syncWith is in the extracted API surface).
- Block entity NBT ([[10-Workflows/wf.content.block_entity|wf.content.block_entity]]) and item stack data components.
- [[50-Interactions/events/net.fabricmc.fabric.api.event.lifecycle.v1.ServerLifecycleEvents.BEFORE_SAVE|BEFORE_SAVE]] for flush-before-save hooks.

## Tools and artifacts used today

- The generated SavedData or attachment class; codecs from net.minecraft.core / serialization; [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] helpers.

## Decisions the creator must make

- Scope: level, player, entity, chunk, block -- this decides the mechanism.
- Whether the client needs the value (then sync: attachment sync predicate or a payload).
- Codec versioning from day one ([[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]).

## Information those decisions need

- The 26.3 SavedData registration signature (corrected during the lantern build; the record carries the compiled form).
- Attachment sync predicates available (all / targetOnly / allButTarget in the extracted surface).

## Existing automation

- SavedData and attachment generators in ModCheck.

## Remaining manual or unsupported work

- Choosing the scope; the codec.

## ModCheck's contribution

- Delivered for the two generators; a planner rule mapping stated scope to mechanism.

## Interactions to check

- Two mods attaching to the same target with the same id.
- Save timing relative to SERVER_STOPPING.

## Evidence

- `capability/persist_state.fabric_saveddata`
- `capability/persist_state.fabric_attachment`
- `extracted/fabric_api.json#fabric-data-attachment-api-v1`

## Open questions

- [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: both generators compile against the pinned corpus (exercised by the discarded scaffold's build); the reference lantern's block-entity persistence is hand-authored and contract-tested with 5 JUnit tests
