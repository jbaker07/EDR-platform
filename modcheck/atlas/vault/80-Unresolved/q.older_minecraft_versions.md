---
type: "question"
id: "q.older_minecraft_versions"
kind: "missing_artifact_access"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.older_minecraft_versions

**Question.** Can the 1.21.x (and other 26.x) Minecraft jars be resolved and hashed the same way as 26.3, so that member and injection-target differences between versions can be extracted rather than remembered?

**Kind.** `missing_artifact_access` -- **Status.** open

**Why it matters.** Every migration claim in this atlas (mappings removal, END_WORLD_TICK to END_LEVEL_TICK, pack format bumps) rests on one 26.3 jar and on cached build scripts, not on a diff of two jars.

**Affects.** [[10-Workflows/wf.engineering.version_migration|wf.engineering.version_migration]], [[70-Requests/request.port_1_21_mod|request.port_1_21_mod]], [[10-Workflows/wf.state.migration|wf.state.migration]]

**Evidence already available.**
- [[00-Scope/Sources|fabric_example_mod_build_1_21]]
- [[00-Scope/Sources|fabric_example_mod_build_26_3]]
- [[00-Scope/Sources|mojang_version_manifest]]
- `resolution/remove_mappings_for_non_obfuscated_minecraft`

**Best remaining source.** Fabric Loom against a second gradle.properties pinned to the older version, in a scratch project, producing a second merged jar in the Gradle cache.

**Procedure.** Create a throwaway Loom project for the older version, run a dependency resolution, hash the resulting merged jar, run atlas/extract/corpus.py and vanilla_members.py against it, and diff member sets per hooked type.

**Done when.** corpus.json lists two Minecraft versions and a generated note reports members added, removed and re-signed between them.

**Conclusions affected while open.**
- wf.engineering.version_migration lists migrations from memory of build failures, not from a diff.
