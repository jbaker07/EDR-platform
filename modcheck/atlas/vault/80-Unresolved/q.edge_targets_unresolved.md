---
type: "question"
id: "q.edge_targets_unresolved"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.edge_targets_unresolved

**Question.** Which of the edge targets (75 at the time of writing; 90-Coverage carries the current count) that are neither declared on the hooked vanilla type nor on a superclass are (a) declared on an interface, (b) java.lang.Object methods, (c) wildcard targets such as <clinit>*, or (d) members Fabric adds by interface injection and that do not exist in the vanilla jar at all?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** A calls edge whose target does not exist in vanilla is either an extraction artefact or a Fabric-injected member. Until each is classified, a mod author reading the interface note for that type may look for a method that is not there.

**Affects.** [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[10-Workflows/wf.integration.mixin_coexistence|wf.integration.mixin_coexistence]]

**Evidence already available.**
- `extracted/minecraft_members.json`
- `extracted/edges.json#calls`

**Best remaining source.** The same jars: walk `implements` chains with javap (interfaces were not walked), and read fabric-api's interface-injection declarations from each module's fabric.mod.json custom block.

**Procedure.** Extend atlas/extract/vanilla_members.py to parse `implements` in javap headers and walk them; read `fabric-api:interface-injections` (or the equivalent custom key) from fabric_api.json declared.custom; re-run and expect the unresolved list to fall to zero with each item tagged by category.

**Done when.** minecraft_members.json reports 0 unresolved edge targets, or every remaining one carries a category tag with evidence.

**Conclusions affected while open.**
- Interface notes under 40-Interfaces for the owners of unresolved targets may omit an inherited or injected member that an edge names.
- The "targets resolved" count in 90-Coverage is a lower bound.
