---
type: "question"
id: "q.unhooked_vanilla_members"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.unhooked_vanilla_members

**Question.** What are the members of vanilla types that no Fabric API module hooks but that a request needs to reason about -- for example the villager AI classes, the world-generation feature classes, and the entity animation classes?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** Member extraction was scoped to the 622 hooked types. A request that touches an unhooked type has only the package inventory to go on, so its candidate list names classes without confirming their signatures.

**Affects.** [[70-Requests/request.villager_fear|request.villager_fear]], [[70-Requests/request.crystal_caves|request.crystal_caves]], [[70-Requests/request.lantern_moth|request.lantern_moth]], [[10-Workflows/wf.behaviour.entity_ai|wf.behaviour.entity_ai]], [[10-Workflows/wf.world.features_biomes|wf.world.features_biomes]]

**Evidence already available.**
- `extracted/corpus.json`
- `extracted/minecraft_surface.json.gz`

**Best remaining source.** The merged 26.3 jar, already resolved and hashed.

**Procedure.** Give atlas/extract/vanilla_members.py an optional list of extra types (from the request notes' affected_systems), extract them the same way, and store them under a separate `requested_types` key so the hooked/unhooked distinction survives.

**Done when.** Every type named in a request's candidates has an interface note with declared members.

**Conclusions affected while open.**
- Candidate designs in 70-Requests that name unhooked classes are unverified against 26.3 signatures.
