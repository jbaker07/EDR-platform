---
type: "question"
id: "q.team_counter_scope"
kind: "ambiguous_creator_intent"
status: "open"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# q.team_counter_scope

**Question.** For the team resource counter, is a "team" the vanilla scoreboard team, a mod-defined group, or every player on the server; and must the count survive a player changing team?

**Kind.** `ambiguous_creator_intent` -- **Status.** open

**Why it matters.** Scoreboard teams are managed by operators with /team and can be dissolved; a mod-defined team needs its own persistence and UI. The persistence shape and the sync predicate both depend on the answer.

**Affects.** [[70-Requests/request.team_counter|request.team_counter]]

**Evidence already available.**
- [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]]
- `capability/persist_state.fabric_saveddata`

**Best remaining source.** The creator.

**Procedure.** Ask with the two models and their operator-facing consequences.

**Done when.** approved_behaviour names the team model and the behaviour on team change.

**Conclusions affected while open.**
- The request note's candidate data shape is provisional.
