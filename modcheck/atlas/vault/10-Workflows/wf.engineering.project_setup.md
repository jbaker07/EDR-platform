---
type: "workflow"
id: "wf.engineering.project_setup"
area: "engineering"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Set up a buildable project for 26.3

**Intent.** A project that compiles against the exact game and loader versions, runs, and can be handed to someone else, without guessing at toolchain versions.

## Must be preserved

- The creator's existing project layout when one exists.

## Mechanisms that can serve it

- Fabric Loom 1.17.21 in Gradle 9.5.1 on JDK 25 (`extracted/corpus.json` build_tooling group; [[00-Scope/Sources|fabric_example_mod_build_26_3]]; [[00-Scope/Sources|fabric_example_mod_versions_26_3]]).
- No mappings: 26.3 ships non-obfuscated (`resolution/remove_mappings_for_non_obfuscated_minecraft`; `failure/mojang_mappings_in_non_obfuscated_environment`).

## Tools and artifacts used today

- ModCheck's scaffold (`capability/build_and_test.fabric_project_setup_current_version`) using the provisioned toolchain in modcheck/toolchains (gitignored, never redistributed).

## Decisions the creator must make

- Which Fabric API modules to depend on (all, or the subset the atlas shows the mod uses).

## Information those decisions need

- Current loader and API versions ([[00-Scope/Sources|fabric_loader_meta]], [[00-Scope/Sources|fabric_api_maven_metadata]]).
- Java version required (version.json java_version 25; `failure/java_runtime_older_than_the_mod_requires`).

## Existing automation

- Scaffold and build in ModCheck; Loom resolves the game.

## Remaining manual or unsupported work

- Nothing for the common case.

## ModCheck's contribution

- Delivered.

## Evidence

- `capability/build_and_test.fabric_project_setup_current_version`
- `example/fabric_example_mod`
- [[00-Scope/Sources|fabric_example_mod_build_26_3]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: gradle build succeeds against the pinned corpus for the scaffold and the reference lantern
