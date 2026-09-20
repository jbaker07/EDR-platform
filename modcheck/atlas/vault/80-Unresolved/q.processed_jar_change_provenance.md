---
type: "question"
id: "q.processed_jar_change_provenance"
kind: "incomplete_extraction"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.processed_jar_change_provenance

**Question.** The jar a mod compiles against differs from Mojang's merged jar in 363 classes (771 member access changes, 13 class access changes, 88 hierarchy changes); which Fabric API module's access widener or interface injection produced each change?

**Kind.** `incomplete_extraction` -- **Status.** open

**Why it matters.** A member that is public only because a Fabric module widened it disappears when that module is absent; a mod that calls it depends on the module without declaring it.

**Affects.** [[10-Workflows/wf.behaviour.vanilla_modification|wf.behaviour.vanilla_modification]], [[10-Workflows/wf.engineering.project_setup|wf.engineering.project_setup]]

**Evidence already available.**
- `extracted/resolved_environment.json`
- `extracted/minecraft_surface.json.gz`
- `extracted/fabric_api.json`

**Best remaining source.** Each module's declared accessWidener / classtweaker file and interface-injection custom values, all inside the corpus jars.

**Procedure.** Parse every module's access widener file and fabric-api:interface-injection custom block; attach the responsible module to each change in resolved_environment.json; emit a `derives_from` edge per change.

**Done when.** Every processed-jar change names its module, or is recorded as produced by Loom itself.

**Conclusions affected while open.**
- Interface notes show widened access without saying which module a mod must depend on for it.
