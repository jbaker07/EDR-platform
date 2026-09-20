---
type: "workflow"
id: "wf.engineering.testing"
area: "engineering"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Test a mod without a player

**Intent.** Know that the mod's logic is right before anyone launches the game, and know which claims still need a game.

## Must be preserved

- Evidence discipline: unit tests with fakes establish logic, not game behaviour.

## Mechanisms that can serve it

- JUnit against the mod's own classes with fakes for game types (the reference lantern: 40 tests).
- [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] and [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- in-game tests run by a headless server or client (not exercised here: [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]).

## Tools and artifacts used today

- Gradle test task; gametest modules inspected but not run.

## Decisions the creator must make

- What to fake versus what to leave to a gametest.

## Information those decisions need

- A runnable 26.3 server in a sandbox ([[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]).

## Existing automation

- The creator pipeline runs the build and the JUnit tests it generates.

## Remaining manual or unsupported work

- Gametests; runtime evidence ingestion.

## ModCheck's contribution

- The `observed` evidence class exists in the schema and is empty; a headless run is the next step.

## Evidence

- `extracted/fabric_api.json#fabric-gametest-api-v1`
- `capability/build_and_test.fabric_project_setup_current_version`

## Open questions

- [[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]
- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: False
- interaction_analysed: False
- implemented_in_modcheck: True
- validated_scope: JUnit only; no gametest has run
