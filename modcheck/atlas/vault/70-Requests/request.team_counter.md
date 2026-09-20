---
type: "request"
id: "request.team_counter"
canonical: "exercise.team_counter"
kind: "analyst_exercise"
family: "workflow:wf.multiplayer.networking"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Team resource counter shown on every player's HUD

**Canonical request.** `exercise.team_counter` (analyst_exercise)

> [!note] Analyst exercise
> No creator wrote this request. Nothing in it is approved intent.

## Request

Analyst-authored exercise request: each team accumulates a count of gathered resources (say, ores mined); every member sees their team's count on the HUD; the count persists with the world and survives players leaving and rejoining.

## Approved behaviour (the request's own words or acceptance criteria)

- None approved: there is no creator. The exercise carries two explicit team definitions below; neither is chosen.

## Analyst assumptions

- Definition T1 -- team = the vanilla scoreboard team (operator-managed with /team; can be dissolved; membership changes fire no Fabric event in `extracted/edges.json#callback_of`).
- Definition T2 -- team = a mod-defined group with its own commands and persistence (more work; independent of operators).
- Counts persist per world under either definition; the HUD shows the viewer's own team only.

## Preservation obligations

- Scoreboard teams as operators manage them.
- No count is ever decided by the client.

## Affected systems

- net.minecraft.server.level -- level-scoped (actually server-scoped) persistence; the block-break hook on ServerPlayerGameMode.
- net.minecraft.network.protocol -- the sync payload.
- net.minecraft.client.gui -- the HUD element.

## Implementation candidates

- Composition of existing generators: a SavedData keyed by team name (`capability/persist_state.fabric_saveddata`), incremented in a listener on [[50-Interactions/events/net.fabricmc.fabric.api.event.player.PlayerBlockBreakEvents.AFTER|AFTER]], synced by a payload (`capability/sync_state.fabric_custom_payload`) on change to team members and on join through [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN|JOIN]], rendered by a HUD element (`capability/display_information.fabric_hud_element`).
- Attachment on the server-level with syncWith and a predicate ([[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]]): removes the hand-written payload but the sync predicate options extracted (all, targetOnly, allButTarget) do not express "team members only", so a filtered send would still be manual.
- Scoreboard objective: vanilla scoreboard already syncs and persists objectives; a per-team objective shown through the vanilla sidebar needs no mod networking at all but gives no custom HUD.

## Data / control / state dependencies

- Data: team membership (vanilla scoreboard), the per-team count.
- Control: block-break event (server), join event (server), payload receiver (client), HUD (client).
- State: server-authoritative count; client copy per player; overworld-scoped SavedData reached from any level's server.

## Interactions with the selected environment

- PlayerBlockBreakEvents.AFTER fires from the server game mode; a mod that cancels breaking at BEFORE prevents the count (correct).
- Team changes via /team do not fire a Fabric event in the extracted list; the HUD shows a stale team until the next payload -- a design gap to close by re-sending on team change or polling per tick on the server (cheap, one comparison per player).
- A second mod's HUD element in the same corner: layout is not arbitrated by the API.

## Alternatives and tradeoffs

- Mod-defined teams with their own commands: more work, no dependence on operator-managed teams.

## Implementation work

- Compose the four generators; write the team-keyed SavedData codec with a version field ([[10-Workflows/wf.state.migration|wf.state.migration]]); join-time sync; unit tests with fakes.

## Verification obligations

- Compile and JUnit (possible now).
- Delivery to the right players and persistence across restart: need a game; none observed.

## Unresolved

- [[80-Unresolved/q.team_counter_scope|q.team_counter_scope]]
- [[80-Unresolved/q.payload_receiver_thread|q.payload_receiver_thread]]
- [[80-Unresolved/q.runtime_event_delivery|q.runtime_event_delivery]]
- [[80-Unresolved/q.mod_data_migration_practice|q.mod_data_migration_practice]]

## Evidence

- `capability/persist_state.fabric_saveddata`
- `capability/sync_state.fabric_custom_payload`
- `capability/display_information.fabric_hud_element`
- `extracted/edges.json#publishes_event`
- `extracted/fabric_api.json#fabric-data-attachment-api-v1`

## Status

- analysed: True
- implemented: none
- validated_scope: none
