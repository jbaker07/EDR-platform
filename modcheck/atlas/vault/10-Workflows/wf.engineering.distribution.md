---
type: "workflow"
id: "wf.engineering.distribution"
area: "engineering"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Package, license and distribute

**Intent.** Ship a jar players can install, with a license that matches the mod's dependencies, on the channels players use.

## Must be preserved

- Third-party licenses (Fabric API is Apache-2.0: [[00-Scope/Sources|fabric_api_license]]; Loom: [[00-Scope/Sources|fabric_loom_license]]).
- No redistribution of Mojang's assets or jars.

## Mechanisms that can serve it

- Gradle build producing build/libs/<modid>-<version>.jar (`capability/build_and_test.fabric_project_setup_current_version`).
- Modrinth ([[00-Scope/Sources|modrinth_fabric_api]] shows the project metadata shape) and Nexus Mods ([[00-Scope/Sources|nexusmods_web]]); CurseForge not retrieved.

## Tools and artifacts used today

- Manual upload.

## Decisions the creator must make

- License choice; whether to bundle Fabric API (never) or declare it as a dependency.

## Information those decisions need

- Channel upload APIs (not cached).

## Existing automation

- None beyond the build.

## Remaining manual or unsupported work

- Upload and metadata.

## ModCheck's contribution

- Reuse-terms recording per source is already enforced in the store; a license compatibility check is a candidate.

## Evidence

- [[00-Scope/Sources|fabric_api_license]]
- [[00-Scope/Sources|modrinth_fabric_api]]
- `example/fabric_api`

## Status

- inventoried: True
- mechanically_inspected: False
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: False
- validated_scope: none
